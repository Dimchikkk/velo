use crate::Location;
use std::path::Path;

impl<'a> Location<'a> {
    pub fn get_path(&self) -> std::path::PathBuf {
        match self {
            Self::CustomPath(path) => path.to_path_buf(),
            Self::PlatformDefault(config) => {
                let dirs = directories::ProjectDirs::from(
                    config.qualifier.as_deref().unwrap_or(""),
                    &config.organization,
                    &config.application,
                );
                match dirs.as_ref() {
                    Some(dirs) => dirs.data_dir(),
                    None => Path::new("."), // todo: maybe warn?
                }
                .to_path_buf()
            }
        }
    }
}
