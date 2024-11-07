pub use anyhow::Result;

#[derive(Debug)]
pub enum GripError {
    PackageNotFound(String),
    RegistryNotFound(String),
    VersionNotFound(String),
    AssetNotFound(String),
    DownloadError(String),
    InstallError(String),
    RegistryError(String),
    IoError(std::io::Error),
    RequestError(reqwest::Error),
    Other(anyhow::Error),
}

impl std::fmt::Display for GripError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GripError::PackageNotFound(pkg) => write!(f, "Package not found: {}", pkg),
            GripError::RegistryNotFound(reg) => write!(f, "Registry not found: {}", reg),
            GripError::VersionNotFound(ver) => write!(f, "Version not found: {}", ver),
            GripError::AssetNotFound(asset) => write!(f, "Asset not found: {}", asset),
            GripError::DownloadError(msg) => write!(f, "Download error: {}", msg),
            GripError::InstallError(msg) => write!(f, "Installation failed: {}", msg),
            GripError::RegistryError(msg) => write!(f, "Registry error: {}", msg),
            GripError::IoError(e) => write!(f, "IO error: {}", e),
            GripError::RequestError(e) => write!(f, "Request error: {}", e),
            GripError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for GripError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GripError::IoError(e) => Some(e),
            GripError::RequestError(e) => Some(e),
            GripError::Other(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl From<std::io::Error> for GripError {
    fn from(err: std::io::Error) -> Self {
        GripError::IoError(err)
    }
}

impl From<reqwest::Error> for GripError {
    fn from(err: reqwest::Error) -> Self {
        GripError::RequestError(err)
    }
}

impl From<anyhow::Error> for GripError {
    fn from(err: anyhow::Error) -> Self {
        GripError::Other(err)
    }
}