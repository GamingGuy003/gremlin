use std::{path::PathBuf, sync::OnceLock};

use sysdirs::PathExt;

/// stores the cache file path
static CACHE: OnceLock<PathBuf> = OnceLock::new();
pub fn get_cache() -> PathBuf {
    CACHE
        .get_or_init(|| sysdirs::cache_dir().join("gremlin").ensure().unwrap())
        .to_owned()
}

/// stores the config location file path
static CONFIG: OnceLock<PathBuf> = OnceLock::new();
pub fn get_config() -> PathBuf {
    CONFIG
        .get_or_init(|| sysdirs::config_dir().join("gremlin").ensure().unwrap())
        .to_owned()
}

/// stores the data file path
static DATA: OnceLock<PathBuf> = OnceLock::new();
pub fn get_data() -> PathBuf {
    CONFIG
        .get_or_init(|| sysdirs::data_dir().join("gremlin").ensure().unwrap())
        .to_owned()
}
