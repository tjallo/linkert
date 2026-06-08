use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MetricsPayload {
    hostname: String,
    timestamp: u64,
    cpu_percent: f32,
    mem_used_bytes: u64,
    mem_total_bytes: u64,
    disk_used_bytes: u64,
    disk_total_bytes: u64,
    net_rx_bytes: u64,
    net_tx_bytes: u64,
}
