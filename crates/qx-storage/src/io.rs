use std::path::{Path, PathBuf};

use color_eyre::Result;

pub trait FileAccess: Clone {
    fn user_data_directory(&self) -> PathBuf;
    fn read_to_string(&self, path: &Path) -> Result<String>;
    fn write<D: AsRef<[u8]>>(&self, path: &Path, data: D) -> Result<()>;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn file_exists(&self, path: &Path) -> bool;
}

#[derive(Default, Clone)]
pub struct FileAccessIo {}

impl FileAccess for FileAccessIo {
    fn user_data_directory(&self) -> PathBuf {
        dirs::data_dir().expect("Could not retrieve data directory.")
    }

    fn read_to_string(&self, path: &Path) -> Result<String> {
        std::fs::read_to_string(path).map_err(Into::into)
    }

    fn write<D: AsRef<[u8]>>(&self, path: &Path, data: D) -> Result<()> {
        std::fs::write(path, data).map_err(Into::into)
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        std::fs::create_dir_all(path).map_err(Into::into)
    }

    fn file_exists(&self, path: &Path) -> bool {
        path.exists()
    }
}
