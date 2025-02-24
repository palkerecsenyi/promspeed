use axum::http::HeaderMap;
use lazy_static::lazy_static;
use prometheus::{Encoder, Gauge, TextEncoder, register_gauge};
use reqwest::header;

lazy_static! {
    pub static ref RESULTS_GAUGE: Gauge = register_gauge!(
        "promspeed_bytes_per_second",
        "Results of the speed test in Bytes per second.",
    )
    .unwrap();
}

pub async fn metrics_route() -> (HeaderMap, Vec<u8>) {
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, encoder.format_type().parse().unwrap());

    (headers, buffer)
}
