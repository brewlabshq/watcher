use crate::watcher_config::WatcherConfig;
use notify::{Event, RecursiveMode, Watcher};
use serde::Serialize;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::sync::mpsc::channel;

const BATCH_SIZE: usize = 500;
const MAX_BATCH_BYTES: usize = 1_048_576;

#[derive(Debug, Serialize, Clone)]
pub struct LogEntry {
    content: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct LogWatcher {
    config: WatcherConfig,
    file: BufReader<File>,
}

impl LogWatcher {
    fn init(config: WatcherConfig) -> io::Result<Self> {
        let file = File::open(&config.log_path).expect("Unable to open log file");

        let mut reader = BufReader::with_capacity(MAX_BATCH_BYTES, file);

        reader.seek(SeekFrom::End(0))?;
        Ok(LogWatcher {
            config,
            file: reader,
        })
    }

    pub fn from_file(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = std::fs::read_to_string(config_path)?;
        let config: WatcherConfig = toml::from_str(&config_str)?;
        Ok(LogWatcher::init(config)?)
    }

    fn should_process_line(&self, line: &str) -> bool {
        if let Some(patterns) = &self.config.filter {
            patterns.iter().any(|pattern| line.contains(pattern))
        } else {
            true
        }
    }

    async fn process_new_lines(
        &mut self,
        client: &dyn crate::adapters::log_adapter::LogAdapter,
    ) -> io::Result<()> {
        let mut line = String::new();
        let mut batch = Vec::new();
        let mut batch_size_bytes = 0;

        while self.file.read_line(&mut line)? > 0 {
            if !line.trim().is_empty() && self.should_process_line(&line) {
                let entry = LogEntry {
                    content: line.clone(),
                    timestamp: chrono::Utc::now(),
                };

                let entry_size = line.as_bytes().len();

                if batch_size_bytes + entry_size > MAX_BATCH_BYTES || batch.len() >= BATCH_SIZE {
                    if let Err(e) = client.ingest(std::mem::take(&mut batch)).await {
                        println!("Error sending batch logs");
                        println!("{:?}", e);
                    }
                    println!("Sent {:?} logs", batch.len());
                    batch.clear();
                    batch_size_bytes = 0;
                }

                batch.push(entry);
                batch_size_bytes += entry_size;
            }
            line.clear();
        }

        if !batch.is_empty() {
            if let Err(_) = client.ingest(batch).await {
                println!("Error sending final batch logs");
            }
        }
        Ok(())
    }

    pub async fn watch(
        &mut self,
        clinet: Box<dyn crate::adapters::log_adapter::LogAdapter>,
    ) -> notify::Result<()> {
        let (tx, rx) = channel();

        let mut watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                tx.send(event)
                    .unwrap_or_else(|e| eprintln!("Failed to send event: {}", e));
            }
        })?;

        println!(
            "Started watching {} for new content...",
            self.config.log_path
        );

        // Watch the file's parent directory for modifications
        watcher.watch(
            std::path::Path::new(&self.config.log_path)
                .parent()
                .unwrap(),
            RecursiveMode::NonRecursive,
        )?;

        let mut previous_size = self.file.get_ref().metadata()?.len();

        for event in rx {
            match event {
                Event {
                    kind: notify::EventKind::Modify(_),
                    ..
                } => {
                    // Check if file was truncated (rotated)
                    let current_size = self.file.get_ref().metadata()?.len();
                    if current_size < previous_size {
                        // File was truncated, seek to beginning
                        self.file.seek(SeekFrom::Start(0))?;
                        println!("Log rotation detected, starting from beginning of new file");
                    }
                    previous_size = current_size;

                    if let Err(e) = self.process_new_lines(&*clinet).await {
                        eprintln!("Error processing new lines: {}", e);

                        // Handle file deletion/rotation by reopening the file
                        if e.kind() == io::ErrorKind::NotFound {
                            println!("File not found, waiting for it to reappear...");
                            while let Err(_) = File::open(&self.config.log_path) {
                                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            }
                            let file = File::open(&self.config.log_path)?;
                            self.file = BufReader::new(file);
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
