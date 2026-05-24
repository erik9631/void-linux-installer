#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GpuType {
    Nvidia,
    Amd,
    Integrated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootMode {
    Uefi,
    Bios,
}

pub enum CloudClientType {
    OneDrive,
    NextCloud,
}

#[derive(Debug, Clone)]
pub struct WirelessInterface {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct WirelessNetwork {
    pub ssid: String,
    /// Signal strength in dBm (e.g. -50 is strong, -90 is weak).
    pub signal_strength: i32,
    /// BSSID — the MAC address of the access point, unique per radio.
    pub bssid: String,
}

#[derive(Debug, Clone)]
pub struct UserConfig {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub dns_servers: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct UserRcConfig {
    pub bashrc_lines: Vec<String>,
    pub bash_profile_lines: Vec<String>,
    pub rc_local_lines: Vec<String>,
}

pub struct FormData {
    pub hostname: String,
    pub username: String,
    pub password: String,
    pub boot_mode: BootMode,
    pub boot_menu_name: String,
    pub cloud_client: Option<CloudClientType>,
    pub cloud_client_path: Option<String>,
}

pub struct Package {
    pub name: String,
    pub doc: String,
}

pub type PackageList = Vec<Package>;
