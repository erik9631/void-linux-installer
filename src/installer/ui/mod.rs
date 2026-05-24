pub mod commands;
pub mod errors;
pub mod traits;
pub mod tui;

use tokio::sync::{mpsc, oneshot};

use crate::installer::core::{
    errors::{InstallFormError, InstallFormResult, PackageListFormError, PackageListFormResult},
    model::{BootMode, FormData, PackageList, WirelessNetwork},
    traits::UserInput,
};
use commands::Command;
use errors::{NetworkSelectionScreenError, UiChannelError};
use traits::{NetworkSelectionResult, NetworkSelectionScreen};

/// Generic handle for any UI backend. Implements installer traits by
/// serialising commands over the channel supplied at construction. Use
/// `Tui::sender()` to obtain the sender for a terminal UI backend.
pub struct UiHandle {
    tx: mpsc::Sender<Command>,
}

impl UiHandle {
    pub fn new(tx: mpsc::Sender<Command>) -> Self {
        Self { tx }
    }

    fn send<F, T>(&self, build: F) -> Result<oneshot::Receiver<T>, UiChannelError>
    where
        F: FnOnce(oneshot::Sender<T>) -> Command,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.tx
            .blocking_send(build(reply_tx))
            .map_err(|_| UiChannelError::SendFailed)?;
        Ok(reply_rx)
    }
}

impl NetworkSelectionScreen for UiHandle {
    fn show_network_selection(
        &self,
        networks: &[WirelessNetwork],
    ) -> errors::NetworkSelectionScreenResult<NetworkSelectionResult> {
        let reply_rx = self
            .send(|reply| Command::NetworkSelection {
                networks: networks.to_vec(),
                reply,
            })
            .map_err(|e| NetworkSelectionScreenError::InputFailed(e.to_string()))?;

        reply_rx
            .blocking_recv()
            .map_err(|e| NetworkSelectionScreenError::InputFailed(e.to_string()))
    }
}

impl UserInput for UiHandle {
    fn install_form(&self, boot_options: &[BootMode]) -> InstallFormResult<FormData> {
        let reply_rx = self
            .send(|reply| Command::InstallForm {
                boot_options: boot_options.to_vec(),
                reply,
            })
            .map_err(|e| InstallFormError::InputFailed(e.to_string()))?;

        reply_rx
            .blocking_recv()
            .map_err(|e| InstallFormError::InputFailed(e.to_string()))
    }

    fn package_list_form(&self, packages: PackageList) -> PackageListFormResult<Vec<String>> {
        let reply_rx = self
            .send(|reply| Command::PackageListForm { packages, reply })
            .map_err(|e| PackageListFormError::InputFailed(e.to_string()))?;

        reply_rx
            .blocking_recv()
            .map_err(|e| PackageListFormError::InputFailed(e.to_string()))
    }
}
