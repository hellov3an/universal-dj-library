use crate::domain::error::AppError;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::Path;

pub struct FileHasher;

impl FileHasher {
    /// Computes a SHA-256 hash for a file.
    /// Reads in chunks for memory efficiency with large files.
    pub fn sha256(path: &Path) -> Result<String, AppError> {
        let mut file = std::fs::File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    /// Computes a partial hash (first 64KB) for quick comparison.
    pub fn sha256_partial(path: &Path) -> Result<String, AppError> {
        let mut file = std::fs::File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 65536]; // 64KB
        let bytes_read = file.read(&mut buffer)?;
        hasher.update(&buffer[..bytes_read]);
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }
}
