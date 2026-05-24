use tokio::sync::oneshot;

use crate::installer::core::model::{BootMode, FormData, PackageList};

/// Every interaction the TUI needs to fulfil on behalf of the installer.
/// Each variant carries a `reply` sender so the caller can block-wait for
/// the result without knowing anything about the channel underneath.
pub enum Command {
    InstallForm {
        boot_options: Vec<BootMode>,
        reply: oneshot::Sender<FormData>,
    },
    PackageListForm {
        packages: PackageList,
        reply: oneshot::Sender<Vec<String>>,
    },
}
