#![forbid(unsafe_code)]

use std::time::Duration;

use axum::{Router, routing::get};
use metrics::{metrics_route, RESULTS_GAUGE};
use speedtest::speedtest;
use tokio::{net::TcpListener, task, time};

mod metrics;
mod speedtest;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/metrics", get(metrics_route));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(30));

        loop {
            interval.tick().await;

            println!("Running speed test...");
            let bytes_per_second = speedtest().await;
            println!("Done, got {:.2}B/s", bytes_per_second);
            RESULTS_GAUGE.set(bytes_per_second);
        }
    });

    axum::serve(listener, app).await.unwrap();
}
