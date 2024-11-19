use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct WatcherConfig {
    pub log_path:String, // Path to log file
    pub  batch_size:Option<usize>, // Batch of logs to send 
    pub retry_attempts:Option<u32> ,// Retry when request fails
   pub  retry_delay_ms:Option<u64>, // Retry delay
    pub filter:Option<Vec<String>> // ERROR, INFO, Debug filters 
}

impl Default for WatcherConfig {
    fn default() -> Self {
        WatcherConfig {
            filter:None,
            retry_delay_ms:Some(1000),
            log_path:"solana-validator.log".to_string(),
            batch_size:Some(10),
            retry_attempts:Some(3),
        }
    }
}
