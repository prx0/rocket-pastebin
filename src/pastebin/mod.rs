pub mod models;
pub mod routes;

use std::env;
use std::time::{Duration, SystemTime};
use std::fs;
use log;

pub fn clean_old_pastes() {
    let paths = fs::read_dir("upload")
        .expect("unable to read files from upload directory");
        
    for path in paths {
        let path_str = path.unwrap().path();
        let metadata = fs::metadata(&path_str).unwrap();
        let modification_sys_time = metadata.modified().unwrap();
        let now_sys_time = SystemTime::now();
        let age_of_file = now_sys_time.duration_since(modification_sys_time).unwrap();
        let file_lifetime_duration = env::var("FILE_LIFETIME_DURATION")
            .expect("FILE_LIFETIME_DURATION must be set")
            .parse::<u64>()
            .unwrap();
            
        let limit_duration = Duration::from_secs(file_lifetime_duration);
    
        if age_of_file >= limit_duration {
            let _ = match fs::remove_file(&path_str) {
                Ok(_) => log::info!("pastebin {} removed", path_str.display()),
                Err(e) => log::error!("{}", e)
            };
        }   
    }
}