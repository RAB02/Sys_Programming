use std::time::{Duration, Instant};
use crate::api_result::{ApiResult, WebsiteStatus};
use std::fs::File;
use std::io::Write;

pub trait Monitor{
    fn fetch_website(&self, timeout: Duration) -> ApiResult<WebsiteStatus>;
    fn save_to_file(&self, statuses: &[WebsiteStatus]);
}

#[derive(Debug)]
pub struct Website{
    pub web_url: String,
    pub file_name: String,
}

impl Monitor for Website{
    fn fetch_website(&self, timeout: Duration) -> ApiResult<WebsiteStatus>{
        let url = &self.web_url;

        match ureq::get(url).timeout(timeout).call() {
            Ok(response) => {
                let response_time = Instant::now().elapsed();

                if response.status() == 200 {
                    ApiResult::Success(WebsiteStatus {
                        url: url.to_string(),
                        status: Ok(response.status()),
                        response_time,
                        timestamp: chrono::Utc::now(),
                    })
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", response.status()))
                }
            }
            Err(e) => {
                let error_details = format!("Request failed: {}", e);
                ApiResult::NetworkError(error_details)
            }
        }
    }

    fn save_to_file(&self, statuses: &[WebsiteStatus]){
        let mut file = match File::create(&self.file_name) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to create file: {}", e);
                return;
            }
        };

        for status in statuses {
            writeln!(file, "URL: {}, Status: {:?}, Response Time: {:?}, Time Stamp: {:?}", status.url, status.status, status.response_time, status.timestamp).unwrap();
        }    
    }
}