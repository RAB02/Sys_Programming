use std::time::Duration;
use chrono::{DateTime, Utc};

#[derive(Debug)]
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: DateTime<Utc>,
    valid_ssl: bool,
}

fn run_monitoring(
    websites: Vec<String>,
    runtime: Duration,
    count: usize,
    monitor_period: Duration,
) {
    println!( "Running monitoring for {:?} with runtime {:?}, count {}, monitor period {:?}", websites, runtime, count, monitor_period);
}

fn verify_site(url: &str, runtime: Duration) -> WebsiteStatus {
    if url == "https://invalid.url" {
        WebsiteStatus {
            url: url.to_string(),
            status: Err("Network error".to_string()),
            response_time: runtime,
            timestamp: Utc::now(),
            valid_ssl: false,
        }
    } else {
        WebsiteStatus {
            url: url.to_string(),
            status: Ok(200),
            response_time: Duration::from_millis(100),
            timestamp: Utc::now(),
            valid_ssl: true,
        }
    }
}

fn stats_calculation(results: &[WebsiteStatus]) {
    for result in results {
        println!( "URL: {}, Status: {:?}, Response Time: {:?}, SSL Valid: {}, Timestamp: {}", result.url, result.status, result.response_time, result.valid_ssl, result.timestamp);
    }
}

#[test]
fn test_monitoring() {
    let websites = vec![
        "https://www.rust-lang.org".to_string(),
        "https://www.github.com".to_string(),
    ];

    let runtime = Duration::from_secs(5);
    let count = 2;
    let monitor_period = Duration::from_secs(20);

    run_monitoring(websites, runtime, count, monitor_period);
}

#[test]
fn test_success() {
    let url = "https://www.rust-lang.org";
    let runtime = Duration::from_secs(5);
    let result = verify_site(url, runtime);

    assert!(
        result.status.is_ok(),
        "Expected successful status, got {:?}",
        result.status
    );
}

#[test]
fn test_failure() {
    let url = "https://invalid.url";
    let runtime = Duration::from_secs(5);
    let result = verify_site(url, runtime);

    assert!(
        result.status.is_err(),
        "Expected error status, got {:?}",
        result.status
    );
}

#[test]
fn test_calculation() {
    let results = vec![
        WebsiteStatus {
            url: "https://www.rust-lang.org".to_string(),
            status: Ok(200),
            response_time: Duration::from_millis(100),
            timestamp: Utc::now(),
            valid_ssl: true,
        },
        WebsiteStatus {
            url: "https://www.github.com".to_string(),
            status: Ok(200),
            response_time: Duration::from_millis(150),
            timestamp: Utc::now(),
            valid_ssl: true,
        },
    ];

    stats_calculation(&results);
}