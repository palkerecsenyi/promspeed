use futures_util::stream::StreamExt;
use std::time::Instant;

pub async fn speedtest() -> f64 {
    let now = Instant::now();
    let mut bytes = reqwest::get("https://speedtest.palk.dev/bigfile.bin")
        .await
        .expect("Random data to load.")
        .bytes_stream();

    let mut num_bytes = 0_f64;
    while let Some(b) = bytes.next().await {
        let b = b.unwrap();
        num_bytes += b.len() as f64;
    }

    let elapsed = now.elapsed();
    num_bytes / elapsed.as_secs_f64()
}
