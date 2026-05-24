use crate::installer::core::{
    errors::{
        InternetCheckResult, WirelessConnectResult, WirelessInterfacesResult, WirelessScanResult,
    },
    model::{WirelessInterface, WirelessNetwork},
    traits::Network,
};
use crate::installer::wifi::WifiAdapter;

impl Network for WifiAdapter {
    fn check_internet(&self) -> InternetCheckResult<bool> {
        todo!()
    }

    fn list_wireless_interfaces(&self) -> WirelessInterfacesResult<Vec<WirelessInterface>> {
        todo!()
    }

    fn scan_wireless_networks(
        &self,
        _iface: &WirelessInterface,
    ) -> WirelessScanResult<Vec<WirelessNetwork>> {
        todo!()
    }

    fn connect_wireless(&self) -> WirelessConnectResult<()> {
        todo!()
    }
}
