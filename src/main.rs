#![forbid(unsafe_code)]

use std::{env, time::Duration};

use axum::{Router, routing::get};
use health::health_route;
use metrics::{RESULTS_GAUGE, metrics_route};
use speedtest::speedtest;
use tokio::{net::TcpListener, task, time};

mod health;
mod metrics;
mod speedtest;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(health_route))
        .route("/metrics", get(metrics_route));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let poll_frequency: u64 = env::var("PS_POLL_FREQ_SECS")
        .unwrap_or("120".to_string())
        .parse()
        .expect("Env var to specify a valid number");

    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(poll_frequency));

        loop {
            interval.tick().await;

            println!("Running speed test...");
            let bytes_per_second = speedtest().await;

            if let Ok(bytes_per_second) = bytes_per_second {
                println!("Done, got {:.2}B/s", bytes_per_second);
                RESULTS_GAUGE.set(bytes_per_second);
            } else {
                println!("Speed test failed: {}", bytes_per_second.unwrap_err());
            }
        }
    });

    axum::serve(listener, app).await.unwrap();
}
