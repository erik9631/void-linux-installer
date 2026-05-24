use crate::core::{
    errors::*,
    model::{BootMode, FormData, GpuType, PackageList, WirelessInterface, WirelessNetwork},
};
use std::path::Path;

/// Collects all user input up front. Both methods are infallible — validation
/// is the UI's responsibility. install_form gathers hostname, username, password,
/// boot mode, and cloud client selection. package_list_form presents the parsed
/// package list for the user to confirm or trim.
pub trait UserInput {
    fn install_form(&self, boot_options: &[BootMode]) -> FormData;
    fn package_list_form(&self, packages: PackageList) -> Vec<String>;
}

/// Handles internet connectivity detection and wireless setup via wpa_supplicant.
/// NetworkManager configuration is intentionally excluded — the desktop environment
/// handles that post-install.
pub trait Network {
    fn check_internet(&self) -> InternetCheckResult<bool>;
    fn list_wireless_interfaces(&self) -> WirelessInterfacesResult<Vec<WirelessInterface>>;
    fn scan_wireless_networks(
        &self,
        iface: &WirelessInterface,
    ) -> WirelessScanResult<Vec<WirelessNetwork>>;
    fn connect_wireless(&self) -> WirelessConnectResult<()>;
}

/// Manages disk partitioning, target filesystem mounting, and fstab generation.
/// open_disk_edit launches an interactive partitioner (e.g. cfdisk) and returns
/// the chosen root partition path. generate_fstable must be called after install_sys
/// to capture all mounts with their UUIDs.
pub trait Storage {
    fn open_disk_edit(&self) -> DiskEditResult<String>;
    fn create_sys_mnt(&self, device: &Path, target: &Path) -> MountResult<()>;
    fn generate_fstable(&self, target: &Path, output: &Path) -> FstableResult<()>;
}

/// Bootstraps the base OS into the target root. install_sys extracts the rootfs
/// tarball AND runs the full xbps bootstrap (installs base-system, removes
/// base-container-full, runs xbps-reconfigure -fa) — replacing the separate
/// setup_base_system bash step. Hostname is set here as machine identity rather
/// than user identity.
pub trait SystemInstall {
    fn install_sys(&self, tarball: &Path, target: &Path) -> TarballExtractResult<()>;
    fn setup_hostname(&self, hostname: &str, root: &Path) -> HostnameResult<()>;
    fn bios_supported_type(&self) -> BiosDetectResult<Vec<BootMode>>;
    fn setup_bootloader(&self, mode: BootMode, root: &Path) -> BootloaderResult<()>;
}

/// Manages xbps package operations. update_package_manager handles xbps self-update
/// AND adding nonfree/multilib repositories — replacing the separate update_xbps_repo
/// bash step. install_packages handles the bulk user-selected install; retry logic
/// lives in the impl. parse_package_list_from_file reads the TOML package manifest.
pub trait PackageManager {
    fn update_package_manager(&self, root: &Path) -> PackageManagerUpdateResult<()>;
    fn install_packages(&self, packages: &[String], root: &Path) -> PackageInstallResult<()>;
    fn parse_package_list_from_file(&self, path: &Path) -> PackageListParseResult<PackageList>;
}

/// GPU detection via lspci and driver installation inside the chroot. AMD uses
/// mesa from the package list so only NVIDIA requires explicit driver installation.
pub trait Hardware {
    fn detect_gpu_type(&self) -> GpuDetectResult<GpuType>;
    fn install_gpu_drivers(&self, gpu: GpuType, root: &Path) -> DriverInstallResult<()>;
}

/// Configures PipeWire's ALSA and PulseAudio compatibility layers by symlinking
/// the upstream example configs into /etc/pipewire and /etc/alsa. Does NOT install
/// PipeWire itself — that comes from the package list.
pub trait Audio {
    fn configure_pipewire(&self, root: &Path) -> PipewireConfigResult<()>;
}

/// Manages runit service lifecycle. configure_system_services enables system-wide
/// services via /etc/runit/runsvdir/default/ symlinks. configure_user_services
/// enables per-user services (e.g. onedrive). install_service_manager installs
/// the svm wrapper script into the target system, replacing the bash copy_scripts
/// step and giving users a post-install service management tool.
pub trait Service {
    fn configure_system_services(&self, services: &[&str], root: &Path) -> SystemServiceResult<()>;
    fn configure_user_services(
        &self,
        username: &str,
        services: &[&str],
        root: &Path,
    ) -> UserServiceResult<()>;
    fn install_service_manager(&self, root: &Path) -> ServiceManagerInstallResult<()>;
}

/// Creates the user account with password and group memberships (wheel, audio, video)
/// and patches sudoers. configure_user_dots copies everything from the installer's
/// dots/ directory — this single step absorbs copy_configs, setup_kde_defaults,
/// copy_desktop_files, setup_bashrc, setup_rc_local, and set_home_permissions
/// from the bash installer.
pub trait User {
    fn setup_user(&self, username: &str, password: &str, root: &Path) -> UserCreateResult<()>;
    fn configure_user_dots(
        &self,
        username: &str,
        dots_src: &Path,
        root: &Path,
    ) -> DotfilesCopyResult<()>;
}

/// Creates the opt group, adds the target user and root to it, and sets /opt
/// ownership to root:opt with setgid + rw permissions so group members can
/// write to /opt without root.
pub trait OptSetup {
    fn setup_opt(&self, username: &str, root: &Path) -> OptSetupResult<()>;
}

/// Installs user-scoped language toolchains via curl installers, run as the
/// target user inside chroot. Replaces the separate setup_pyenv, setup_nvm,
/// and setup_rust bash steps. nnn was intentionally dropped.
pub trait DevTools {
    fn install_pyenv(&self, username: &str, root: &Path) -> PyenvInstallResult<()>;
    fn install_nvm(&self, username: &str, root: &Path) -> NvmInstallResult<()>;
    fn install_rust(&self, username: &str, root: &Path) -> RustInstallResult<()>;
}

/// Reboots into the freshly installed system at the end of installation.
pub trait SystemLifecycle {
    fn reboot(&self) -> RebootResult<()>;
}
