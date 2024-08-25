pub mod fs;
pub mod git;
pub mod version;

use ::error::Error;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref HOME: String = std::env::var("HOME").expect("no env var HOME");
    pub static ref LOCAL_RESOURCE: LocalResource = LocalResource {
        dist: ".arknights-api",
        repo: "ArknightsGameResource",
        branch: "main",
    };
    pub static ref REMOTE_RESOURCE: RemoteResource = RemoteResource {
        code_server: "https://github.com",
        resource_server: "https://raw.githubusercontent.com",
        user: "yuanyan3060",
        repo: "ArknightsGameResource",
        branch: "main",
    };
}

pub struct LocalResource {
    pub dist: &'static str,
    pub repo: &'static str,
    pub branch: &'static str,
}

pub struct RemoteResource {
    pub code_server: &'static str,
    pub resource_server: &'static str,
    pub user: &'static str,
    pub repo: &'static str,
    pub branch: &'static str,
}

pub async fn need_update() -> Result<bool, Error> {
    Ok(LOCAL_RESOURCE.version()? == REMOTE_RESOURCE.version().await?)
}

pub async fn update_local_resource() -> Result<(), Error> {
    let local_path = format!(
        "{}/{}/{}",
        HOME.as_str(),
        LOCAL_RESOURCE.dist,
        LOCAL_RESOURCE.repo
    );

    if !fs::exists(&local_path) || (fs::is_dir(&local_path) && fs::is_empty(&local_path)?) {
        LOCAL_RESOURCE.init()?;
    }

    LOCAL_RESOURCE.pull(&REMOTE_RESOURCE)?;

    Ok(())
}
