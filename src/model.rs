use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HostState {
    pub CPU: f64,
    pub MemUsed: u64,
    pub SwapUsed: u64,
    pub NetInTransfer: u64,
    pub NetOutTransfer: u64,
    pub NetInSpeed: u64,
    pub NetOutSpeed: u64,
    pub Uptime: u64,
    pub Load1: f64,
    pub Load5: f64,
    pub Load15: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Host {
    pub Name: String,
    pub Platform: String,
    pub PlatformVersion: String,
    pub CPU: Vec<String>,
    pub MemTotal: u64,
    pub SwapTotal: u64,
    pub Arch: String,
    pub Virtualization: String,
    pub BootTime: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushJson {
    pub Host: Host,
    pub State: HostState,
    pub TimeStamp: i64,
}
