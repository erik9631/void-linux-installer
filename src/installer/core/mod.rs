use tracing::{error, warn};

use crate::installer::core::traits::{
    Audio, DevTools, Hardware, Network, OptSetup, PackageManager, Service, Storage, SystemInstall,
    SystemLifecycle, User, UserInput,
};
pub mod errors;
pub mod model;
pub mod traits;

pub struct Setup<
    NetworkType,
    StorageType,
    SystemInstallType,
    PackageManagerType,
    HardwareType,
    AudioType,
    ServiceType,
    UserType,
    UserInputType,
    SystemLifecycleType,
    DevToolsType,
    OptSetupType,
> {
    pub network: NetworkType,
    pub storage: StorageType,
    pub system_install: SystemInstallType,
    pub package_manager: PackageManagerType,
    pub hardware: HardwareType,
    pub audio: AudioType,
    pub service: ServiceType,
    pub user: UserType,
    pub user_input: UserInputType,
    pub system_lifecycle: SystemLifecycleType,
    pub dev_tools: DevToolsType,
    pub opt_setup: OptSetupType,
    pub installer_root_dir: &'static str,
}

impl<
    NetworkType,
    StorageType,
    SystemInstallType,
    PackageManagerType,
    HardwareType,
    AudioType,
    ServiceType,
    UserType,
    UserInputType,
    SystemLifecycleType,
    DevToolsType,
    OptSetupType,
>
    Setup<
        NetworkType,
        StorageType,
        SystemInstallType,
        PackageManagerType,
        HardwareType,
        AudioType,
        ServiceType,
        UserType,
        UserInputType,
        SystemLifecycleType,
        DevToolsType,
        OptSetupType,
    >
where
    NetworkType: Network,
    StorageType: Storage,
    PackageManagerType: PackageManager,
    HardwareType: Hardware,
    SystemInstallType: SystemInstall,
    AudioType: Audio,
    ServiceType: Service,
    UserType: User,
    UserInputType: UserInput,
    SystemLifecycleType: SystemLifecycle,
    DevToolsType: DevTools,
    OptSetupType: OptSetup,
{
    pub fn new(
        network: NetworkType,
        storage: StorageType,
        system_install: SystemInstallType,
        package_manager: PackageManagerType,
        hardware: HardwareType,
        audio: AudioType,
        service: ServiceType,
        user: UserType,
        user_input: UserInputType,
        system_lifecycle: SystemLifecycleType,
        dev_tools: DevToolsType,
        opt_setup: OptSetupType,
        installer_root_dir: &'static str,
    ) -> Self {
        Self {
            network,
            storage,
            system_install,
            package_manager,
            hardware,
            audio,
            service,
            user,
            user_input,
            system_lifecycle,
            dev_tools,
            opt_setup,
            installer_root_dir,
        }
    }

    const SYS_MOUNT_POINT: &str = "/mnt";

    pub fn run(&self) {
        loop {
            if let Ok(connected) = self.network.check_internet() {
                if !connected {
                    if let Err(e) = self.network.connect_wireless() {
                        warn!("Failed to connect to wireless network: {}", e);
                        continue;
                    }
                    break;
                }
                break;
            } else {
                warn!("Failed to check internet connection");
                continue;
            }
        }

        let path = match self.storage.open_disk_edit() {
            Ok(p) => p,
            Err(e) => {
                error!("Failed to open disk editor: {}", e);
                return;
            }
        };

        let root_device_path = std::path::Path::new(&path);
        let root_mount_point = std::path::Path::new(Self::SYS_MOUNT_POINT);
        let fstab_path_str = format!("{}/etc/fstab", Self::SYS_MOUNT_POINT);
        let fstab_path = std::path::Path::new(&fstab_path_str);
        let user_dots_str = format!("{}/dots", self.installer_root_dir);
        let user_dots = std::path::Path::new(&user_dots_str);
        let package_list_path_str = format!("{}/packages.toml", self.installer_root_dir);
        let package_list_path = std::path::Path::new(&package_list_path_str);
        let system_path_str = format!("{}/void.tar", self.installer_root_dir);
        let system_path = std::path::Path::new(&system_path_str);

        if let Err(e) = self
            .storage
            .create_sys_mnt(root_device_path, root_mount_point)
        {
            error!("Failed to create system mount: {}", e);
            return;
        }

        let package_list = match self
            .package_manager
            .parse_package_list_from_file(package_list_path)
        {
            Ok(p) => p,
            Err(e) => {
                error!("Failed to parse package list: {}", e);
                return;
            }
        };

        let bios_types = match self.system_install.bios_supported_type() {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to determine BIOS type: {}", e);
                return;
            }
        };

        let form_data = self.user_input.install_form(&bios_types);
        let selected_packages = self.user_input.package_list_form(package_list);

        if let Err(e) = self
            .system_install
            .install_sys(system_path, root_mount_point)
        {
            error!("Failed to install system: {}", e);
            return;
        }

        if let Err(e) = self.storage.generate_fstable(root_mount_point, fstab_path) {
            error!("Failed to generate fstab: {}", e);
            return;
        }

        if let Err(e) = self
            .package_manager
            .update_package_manager(root_mount_point)
        {
            error!("Failed to update package manager: {}", e);
            return;
        }

        if let Err(e) = self
            .system_install
            .setup_bootloader(form_data.boot_mode, root_mount_point)
        {
            error!("Failed to setup bootloader: {}", e);
            return;
        }

        if let Err(e) = self
            .system_install
            .setup_hostname(&form_data.hostname, root_mount_point)
        {
            error!("Failed to setup hostname: {}", e);
            return;
        }

        if let Err(e) =
            self.user
                .setup_user(&form_data.username, &form_data.password, root_mount_point)
        {
            error!("Failed to setup user: {}", e);
            return;
        }

        if let Err(e) = self
            .opt_setup
            .setup_opt(&form_data.username, root_mount_point)
        {
            error!("Failed to setup opt: {}", e);
            return;
        }

        if let Err(e) = self
            .package_manager
            .install_packages(selected_packages.as_slice(), root_mount_point)
        {
            error!("Failed to install packages: {}", e);
            return;
        }

        if let Err(e) = self.audio.configure_pipewire(root_mount_point) {
            error!("Failed to configure PipeWire: {}", e);
            return;
        }

        if let Err(e) =
            self.user
                .configure_user_dots(&form_data.username, user_dots, root_mount_point)
        {
            error!("Failed to configure user dotfiles: {}", e);
            return;
        }

        if let Err(e) = self
            .dev_tools
            .install_pyenv(&form_data.username, root_mount_point)
        {
            error!("Failed to install pyenv: {}", e);
            return;
        }

        if let Err(e) = self
            .dev_tools
            .install_nvm(&form_data.username, root_mount_point)
        {
            error!("Failed to install nvm: {}", e);
            return;
        }

        if let Err(e) = self
            .dev_tools
            .install_rust(&form_data.username, root_mount_point)
        {
            error!("Failed to install Rust: {}", e);
            return;
        }

        if let Err(e) = self.service.configure_system_services(
            &[
                "dbus",
                "bluetoothd",
                "sddm",
                "NetworkManager",
                "chronyd",
                "rtkit",
                "polkitd",
            ],
            root_mount_point,
        ) {
            error!("Failed to configure system services: {}", e);
            return;
        }

        let mut user_services = Vec::new();
        if let Some(cloud_client) = form_data.cloud_client {
            match cloud_client {
                model::CloudClientType::OneDrive => {
                    user_services.push("onedrive");
                }
                _ => {}
            }
        }

        if let Err(e) = self.service.configure_user_services(
            &form_data.username,
            user_services.as_slice(),
            root_mount_point,
        ) {
            error!("Failed to configure user services: {}", e);
            return;
        }

        if let Err(e) = self.service.install_service_manager(root_mount_point) {
            error!("Failed to install service manager: {}", e);
            return;
        }

        let gpu_type = match self.hardware.detect_gpu_type() {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to detect GPU type: {}", e);
                return;
            }
        };

        if let Err(e) = self
            .hardware
            .install_gpu_drivers(gpu_type, root_mount_point)
        {
            error!("Failed to install GPU drivers: {}", e);
            return;
        }

        if let Err(e) = self.system_lifecycle.reboot() {
            error!("Failed to reboot: {}", e);
        }
    }
}
