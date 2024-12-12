mod file;
mod api_result;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::api_result::ApiResult;
use crate::file::{Website, Monitor};

fn main() {
    
    let urls = vec![
        "https://www.reddit.com".to_string(),
        "https://www.tumblr.com".to_string(),
        "https://www.behance.net".to_string(),
        "https://www.vimeo.com".to_string(),
        "https://www.soundcloud.com".to_string(),
        "https://www.dribbble.com".to_string(),
        "https://www.kickstarter.com".to_string(),
        "https://www.patreon.com".to_string(),
        "https://www.udemy.com".to_string(),
        "https://www.coursera.org".to_string(),
        "https://www.etsy.com".to_string(),
        "https://www.fiverr.com".to_string(),
        "https://www.upwork.com".to_string(),
        "https://www.slack.com".to_string(),
        "https://www.trello.com".to_string(),
        "https://www.nasa.gov".to_string(),
        "https://www.archlinux.org".to_string(),
        "https://www.debian.org".to_string(),
        "https://www.mozilla.org".to_string(),
        "https://www.wikipedia.com".to_string(),
        "https://www.gnu.org".to_string(),
        "https://www.docker.com".to_string(),
        "https://www.jenkins.io".to_string(),
        "https://www.gitlab.com".to_string(),
        "https://www.bitbucket.org".to_string(),
        "https://www.atlassian.com".to_string(),
        "https://www.cloudflare.com".to_string(),
        "https://www.shopify.com".to_string(),
    ];


    let worker_threads = 2;
    let timeout = Duration::from_secs(5);


    let results = Arc::new(Mutex::new(Vec::new()));
    let urls = Arc::new(urls);

    let file_name = "website_statuses.txt".to_string();


    let mut handles = vec![];

    for thread_id in 0..worker_threads {
        let urls = Arc::clone(&urls);
        let results = Arc::clone(&results);
        let file_name = file_name.clone();

        let handle = thread::spawn(move || {
            for (i, url) in urls.iter().enumerate() {
                if i % worker_threads == thread_id {
                    let website = Website {
                        web_url: url.clone(),
                        file_name: file_name.clone(),
                    };

                    let status = website.fetch_website(timeout);

                    match status {
                        ApiResult::Success(status) => {
                            results.lock().unwrap().push(status);
                        }
                        ApiResult::ApiError(err) | ApiResult::NetworkError(err) => {
                            eprintln!("{}", err);
                        }
                    }
                }
            }
        });

        handles.push(handle);
    }


    for handle in handles {
        handle.join().unwrap();
    }


    let results = results.lock().unwrap();
    let website = Website {
        web_url: String::new(),
        file_name: file_name.clone(),
    };
    website.save_to_file(&results);

    for result in results.iter() {
        println!("URL: {}, Status: {:?}, Response Time: {:?}, Time Stamp: {:?}", result.url, result.status, result.response_time, result.timestamp);
    }
}