#![forbid(unsafe_code)]

use std::time::Duration;

use prometheus::{Gauge, Opts, Registry};
use speedtest::speedtest;
use tokio::{task, time};

mod speedtest;

#[tokio::main]
async fn main() {
    let gauge = Gauge::with_opts(Opts::new(
        "promspeed_bytes_per_second",
        "Results of the speed test in Bytes per second.",
    ))
    .unwrap();

    let r = Registry::new();
    r.register(Box::new(gauge.clone())).unwrap();

    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(30));

        loop {
            interval.tick().await;

            println!("Running speed test...");
            let bytes_per_second = speedtest().await;
            println!("Done, got {:.2}B/s", bytes_per_second);
            gauge.set(bytes_per_second);
        }
    });

    forever.await.unwrap();
}
