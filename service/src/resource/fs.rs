use error::Error;

pub fn exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

pub fn is_file(path: &str) -> bool {
    std::path::Path::new(path).is_file()
}

pub fn is_dir(path: &str) -> bool {
    std::path::Path::new(path).is_dir()
}

pub fn is_empty(path: &str) -> Result<bool, Error> {
    Ok(std::path::Path::new(path).read_dir()?.next().is_none())
}

pub fn create_dir(path: &str) -> Result<(), Error> {
    if exists(path) {
        return Err(std::io::Error::other(format!("exisits: {}", path)).into());
    }

    std::fs::create_dir_all(path)?;

    Ok(())
}

pub fn remove_dir(path: &str) -> Result<(), Error> {
    if !exists(path) {
        return Err(std::io::Error::other(format!("not exisits: {}", path)).into());
    }

    if !is_dir(path) {
        return Err(std::io::Error::other(format!("not a dir: {}", path)).into());
    }

    if !is_empty(path)? {
        return Err(std::io::Error::other(format!("not empty: {}", path)).into());
    }

    std::fs::remove_dir(path)?;

    Ok(())
}
