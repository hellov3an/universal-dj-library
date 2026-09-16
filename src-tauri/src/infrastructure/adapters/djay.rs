use crate::domain::error::AppError;
use std::path::Path;

/// djay by Algoriddim stores its data in a proprietary format.
/// This is a skeleton adapter.
pub struct DjayAdapter;

impl DjayAdapter {
    /// Detects if a djay library exists at the given path.
    pub fn detect(root: &Path) -> bool {
        // djay stores data in ~/Music/djay/ on macOS
        let djay_path = root.join("djay");
        djay_path.exists()
    }

    /// Placeholder for djay import — the format is not well-documented.
    pub fn import(_path: &Path) -> Result<usize, AppError> {
        Err(AppError::Adapter {
            adapter: "djay".to_string(),
            message: "djay adapter is not yet implemented. The djay format is proprietary and requires reverse engineering.".to_string(),
        })
    }
}
