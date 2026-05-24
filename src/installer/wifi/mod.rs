pub mod errors;
pub mod traits;

/// Linux WiFi adapter. Talks to the kernel via `iw` and wpa_supplicant.
/// Implements `crate::installer::core::traits::Network`.
pub struct WifiAdapter;
