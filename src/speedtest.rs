use futures_util::stream::StreamExt;
use std::time::Instant;

pub async fn speedtest() -> Result<f64, anyhow::Error> {
    let now = Instant::now();
    let mut bytes = reqwest::get("https://speedtest.palk.dev/bigfile.bin")
        .await?
        .bytes_stream();

    let mut num_bytes = 0_f64;
    while let Some(b) = bytes.next().await {
        let b = b.unwrap();
        num_bytes += b.len() as f64;
    }

    let elapsed = now.elapsed();
    Ok(num_bytes / elapsed.as_secs_f64())
}
