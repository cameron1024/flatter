use std::path::PathBuf;

pub fn test_asset(s: impl AsRef<str>) -> PathBuf {
    PathBuf::from(format!("src/test/data/{}", s.as_ref()))
}