// Module for pastebin

pub mod models;
pub mod routes;

use std::env;
use std::time::{Duration, SystemTime};
use std::fs;
use log;


// Remove old pastebins
pub fn clean_old_pastes() {
    let paths = fs::read_dir("upload")
        .expect("unable to read files from upload directory");
        
    for path in paths {
        let path_str = path.unwrap().path();
        let metadata = fs::metadata(&path_str).unwrap();
        let modification_sys_time = metadata.modified().unwrap();
        let now_sys_time = SystemTime::now();
        let age_of_file = now_sys_time.duration_since(modification_sys_time).unwrap();

        // 5 minutes by default
        let limit_duration = Duration::from_secs(300);
    
        if age_of_file >= limit_duration {
            let _ = match fs::remove_file(&path_str) {
                Ok(_) => log::info!("pastebin {} removed", path_str.display()),
                Err(e) => log::error!("{}", e)
            };
        }   
    }
}