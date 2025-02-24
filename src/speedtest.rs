use std::time::Instant;

pub async fn speedtest() -> f64 {
    let now = Instant::now();
    let bytes = reqwest::get("https://bigfile.fsn1.your-objectstorage.com/bigfile.bin")
        .await
        .expect("Random data to load.")
        .bytes()
        .await
        .expect("Bytes to parse.");
    let elapsed = now.elapsed();

    let num_bytes = bytes.len() as f64;

    num_bytes / elapsed.as_secs_f64()
}
