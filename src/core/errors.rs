use thiserror::Error;

// --- Network ---

pub type InternetCheckResult<T> = Result<T, InternetCheckError>;
#[derive(Error, Debug)]
pub enum InternetCheckError {
    #[error("Ping failed: {0}")]
    PingFailed(String),
    #[error("Internet check timed out")]
    Timeout,
}

pub type WirelessInterfacesResult<T> = Result<T, WirelessInterfacesError>;
#[derive(Error, Debug)]
pub enum WirelessInterfacesError {
    #[error("No wireless interfaces found")]
    NoInterfacesFound,
    #[error("Failed to enumerate wireless interfaces: {0}")]
    EnumerationFailed(String),
}

pub type WirelessScanResult<T> = Result<T, WirelessScanError>;
#[derive(Error, Debug)]
pub enum WirelessScanError {
    #[error("Network scan failed on interface {0}: {1}")]
    ScanFailed(String, String),
    #[error("No wireless networks found")]
    NoNetworksFound,
}

pub type WirelessConnectResult<T> = Result<T, WirelessConnectError>;
#[derive(Error, Debug)]
pub enum WirelessConnectError {
    #[error("Authentication failed for network {0}: {1}")]
    AuthenticationFailed(String, String),
    #[error("Connection timed out")]
    ConnectionTimeout,
    #[error("wpa_supplicant failed: {0}")]
    WpaSupplicantFailed(String),
}

// --- Storage ---

pub type DiskEditResult<T> = Result<T, DiskEditError>;
#[derive(Error, Debug)]
pub enum DiskEditError {
    #[error("Failed to launch disk editor: {0}")]
    EditorLaunchFailed(String),
    #[error("Partitioning was aborted")]
    PartitioningAborted,
}

pub type MountResult<T> = Result<T, MountError>;
#[derive(Error, Debug)]
pub enum MountError {
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    #[error("Failed to mount {0}: {1}")]
    MountFailed(String, String),
    #[error("Mount target does not exist: {0}")]
    TargetNotFound(String),
}

pub type FstableResult<T> = Result<T, FstableError>;
#[derive(Error, Debug)]
pub enum FstableError {
    #[error("Failed to generate fstab: {0}")]
    GenerationFailed(String),
    #[error("Failed to write fstab to {0}: {1}")]
    WriteFailed(String, String),
}

// --- System installation ---

pub type TarballExtractResult<T> = Result<T, TarballExtractError>;
#[derive(Error, Debug)]
pub enum TarballExtractError {
    #[error("System tarball not found: {0}")]
    NotFound(String),
    #[error("Tarball extraction failed: {0}")]
    ExtractionFailed(String),
    #[error("Base system bootstrap failed: {0}")]
    BootstrapFailed(String),
}

pub type HostnameResult<T> = Result<T, HostnameError>;
#[derive(Error, Debug)]
pub enum HostnameError {
    #[error("Invalid hostname: {0}")]
    InvalidHostname(String),
    #[error("Failed to write hostname file: {0}")]
    WriteFailed(String),
}

pub type BiosDetectResult<T> = Result<T, BiosDetectError>;
#[derive(Error, Debug)]
pub enum BiosDetectError {
    #[error("Failed to detect firmware type: {0}")]
    DetectionFailed(String),
}

pub type BootloaderResult<T> = Result<T, BootloaderError>;
#[derive(Error, Debug)]
pub enum BootloaderError {
    #[error("Bootloader installation failed: {0}")]
    InstallFailed(String),
    #[error("Bootloader config generation failed: {0}")]
    ConfigGenerationFailed(String),
}

// --- Package management ---

pub type PackageManagerUpdateResult<T> = Result<T, PackageManagerUpdateError>;
#[derive(Error, Debug)]
pub enum PackageManagerUpdateError {
    #[error("Package manager self-update failed: {0}")]
    UpdateFailed(String),
    #[error("Failed to add repository {0}: {1}")]
    RepoAddFailed(String, String),
    #[error("Repository sync failed: {0}")]
    SyncFailed(String),
}

pub type PackageInstallResult<T> = Result<T, PackageInstallError>;
#[derive(Error, Debug)]
pub enum PackageInstallError {
    #[error("Package not found: {0}")]
    PackageNotFound(String),
    #[error("Installation failed: {0}")]
    InstallFailed(String),
}

pub type PackageListParseResult<T> = Result<T, PackageListParseError>;
#[derive(Error, Debug)]
pub enum PackageListParseError {
    #[error("Package list file not found: {0}")]
    FileNotFound(String),
    #[error("Invalid package list format at line {0}: {1}")]
    InvalidFormat(usize, String),
}

// --- Hardware ---

pub type GpuDetectResult<T> = Result<T, GpuDetectError>;
#[derive(Error, Debug)]
pub enum GpuDetectError {
    #[error("GPU detection failed: {0}")]
    DetectionFailed(String),
}

pub type DriverInstallResult<T> = Result<T, DriverInstallError>;
#[derive(Error, Debug)]
pub enum DriverInstallError {
    #[error("Driver installation failed: {0}")]
    InstallFailed(String),
    #[error("Driver incompatible with detected hardware: {0}")]
    IncompatibleDriver(String),
}

// --- Audio ---

pub type PipewireConfigResult<T> = Result<T, PipewireConfigError>;
#[derive(Error, Debug)]
pub enum PipewireConfigError {
    #[error("Failed to create PipeWire config directory: {0}")]
    DirectoryCreateFailed(String),
    #[error("Failed to create symlink {0}: {1}")]
    SymlinkFailed(String, String),
}

// --- Services ---

pub type SystemServiceResult<T> = Result<T, SystemServiceError>;
#[derive(Error, Debug)]
pub enum SystemServiceError {
    #[error("Service definition not found for: {0}")]
    ServiceNotFound(String),
    #[error("Failed to enable service {0}: {1}")]
    EnableFailed(String, String),
    #[error("Service already enabled: {0}")]
    AlreadyEnabled(String),
}

pub type UserServiceResult<T> = Result<T, UserServiceError>;
#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("Service definition not found for: {0}")]
    ServiceNotFound(String),
    #[error("Failed to enable user service {0} for {1}: {2}")]
    EnableFailed(String, String, String),
}

pub type ServiceManagerInstallResult<T> = Result<T, ServiceManagerInstallError>;
#[derive(Error, Debug)]
pub enum ServiceManagerInstallError {
    #[error("Failed to install svm to {0}: {1}")]
    InstallFailed(String, String),
    #[error("Failed to set executable permissions on svm: {0}")]
    PermissionsFailed(String),
}

// --- User management ---

pub type UserCreateResult<T> = Result<T, UserCreateError>;
#[derive(Error, Debug)]
pub enum UserCreateError {
    #[error("User already exists: {0}")]
    UserAlreadyExists(String),
    #[error("User creation failed: {0}")]
    CreationFailed(String),
    #[error("Failed to set password for {0}: {1}")]
    PasswordSetFailed(String, String),
    #[error("Failed to add {0} to group {1}: {2}")]
    GroupAddFailed(String, String, String),
}

pub type DotfilesCopyResult<T> = Result<T, DotfilesCopyError>;
#[derive(Error, Debug)]
pub enum DotfilesCopyError {
    #[error("Dotfiles source not found: {0}")]
    SourceNotFound(String),
    #[error("Failed to copy dotfiles to {0}: {1}")]
    CopyFailed(String, String),
    #[error("Permission denied copying dotfiles to {0}")]
    PermissionDenied(String),
    #[error("Failed to write rc file {0}: {1}")]
    RcWriteFailed(String, String),
}

// --- Opt setup ---

pub type OptSetupResult<T> = Result<T, OptSetupError>;
#[derive(Error, Debug)]
pub enum OptSetupError {
    #[error("Failed to create opt group: {0}")]
    GroupCreateFailed(String),
    #[error("Failed to add {0} to opt group: {1}")]
    GroupAddFailed(String, String),
    #[error("Failed to set /opt ownership: {0}")]
    ChownFailed(String),
    #[error("Failed to set /opt permissions: {0}")]
    ChmodFailed(String),
}

// --- Dev tools ---

pub type PyenvInstallResult<T> = Result<T, PyenvInstallError>;
#[derive(Error, Debug)]
pub enum PyenvInstallError {
    #[error("pyenv installer download failed: {0}")]
    DownloadFailed(String),
    #[error("pyenv installation failed for user {0}: {1}")]
    InstallFailed(String, String),
    #[error("Failed to configure pyenv in shell profile for {0}: {1}")]
    ProfileConfigFailed(String, String),
}

pub type NvmInstallResult<T> = Result<T, NvmInstallError>;
#[derive(Error, Debug)]
pub enum NvmInstallError {
    #[error("nvm installer download failed: {0}")]
    DownloadFailed(String),
    #[error("nvm installation failed for user {0}: {1}")]
    InstallFailed(String, String),
}

pub type RustInstallResult<T> = Result<T, RustInstallError>;
#[derive(Error, Debug)]
pub enum RustInstallError {
    #[error("rustup installer download failed: {0}")]
    DownloadFailed(String),
    #[error("Rust toolchain installation failed for user {0}: {1}")]
    InstallFailed(String, String),
}

// --- System lifecycle ---

pub type RebootResult<T> = Result<T, RebootError>;
#[derive(Error, Debug)]
pub enum RebootError {
    #[error("Reboot command failed: {0}")]
    RebootFailed(String),
    #[error("Insufficient permissions to reboot")]
    PermissionDenied,
}
