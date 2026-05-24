use crate::installer::core::model::WirelessNetwork;
use crate::installer::ui::errors::NetworkSelectionScreenResult;

pub struct NetworkSelectionResult {
    /// Index into the original `networks` slice.
    pub index: usize,
    pub password: String,
}

pub trait NetworkSelectionScreen {
    fn show_network_selection(
        &self,
        networks: &[WirelessNetwork],
    ) -> NetworkSelectionScreenResult<NetworkSelectionResult>;
}
