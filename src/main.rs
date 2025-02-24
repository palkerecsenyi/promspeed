#![forbid(unsafe_code)]

use std::time::Duration;

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

    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(2 * 60));

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
