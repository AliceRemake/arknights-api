pub mod fs;
pub mod git;
pub mod version;

use lazy_static::lazy_static;

lazy_static!{
    pub static ref HOME: String = std::env::var("HOME").expect("no env var HOME");
}

pub struct RemoteResource {
    pub code_server: &'static str,
    pub resource_server: &'static str,
    pub user: &'static str,
    pub repo: &'static str,
    pub branch: &'static str,
}

pub fn get_remote_resource_instance() -> &'static RemoteResource {
    static REMOTE_RESOURCE: RemoteResource = RemoteResource {
        code_server: "https://github.com",
        resource_server: "https://raw.githubusercontent.com",
        user: "yuanyan3060",
        repo: "ArknightsGameResource",
        branch: "main",
    };
    &REMOTE_RESOURCE
}

pub struct LocalResource {
    pub dist: &'static str,
    pub repo: &'static str,
    pub branch: &'static str,
}

pub fn get_local_resource_instance() -> &'static LocalResource {
    // let home: String = std::env::var("HOME").expect("");
    static LOCAL_RESOURCE: LocalResource = LocalResource {
        dist: ".arknights-api",
        repo: "ArknightsGameResource",
        branch: "main",
    };
    &LOCAL_RESOURCE
}
