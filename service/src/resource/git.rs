use std::process::Command;

use super::*;
use error::Error;

impl LocalResource {
    pub fn init(&self) -> Result<(), Error> {
        let local_path = format!("{}/{}/{}", HOME.as_str(), self.dist, self.repo);

        if !fs::exists(&local_path) {
            fs::create_dir(&local_path)?;
        }

        if !fs::is_dir(&local_path) {
            return Err(std::io::Error::other(format!("not a dir: {}", &local_path)).into());
        }

        if !fs::is_empty(&local_path)? {
            return Err(std::io::Error::other(format!("not empty: {}", &local_path)).into());
        }

        let output = Command::new("git")
            .current_dir(local_path)
            .args(["init", "-b", self.branch])
            .output()?;
        if !output.status.success() {
            return Err(Error::RuntimeError(String::from("can not init git repo")));
        }

        Ok(())
    }

    pub fn pull(&self, remote_resource: &RemoteResource) -> Result<(), Error> {
        let local_path = format!("{}/{}/{}", HOME.as_str(), self.dist, self.repo);
        let remote_path = format!(
            "{}/{}/{}.git",
            remote_resource.code_server, remote_resource.user, remote_resource.repo
        );

        if !fs::exists(&local_path) {
            return Err(std::io::Error::other(format!("not exisits: {}", &local_path)).into());
        }

        if !fs::is_dir(&local_path) {
            return Err(std::io::Error::other(format!("not a dir: {}", &local_path)).into());
        }

        if !fs::exists(&format!("{}/.git", local_path))
            || !fs::is_dir(&format!("{}/.git", local_path))
        {
            return Err(std::io::Error::other(format!("not a git repo: {}", &local_path)).into());
        }

        let output = Command::new("git")
            .current_dir(&local_path)
            .args(["remote", "-v"])
            .output()?;
        if !output.status.success() {
            return Err(Error::RuntimeError(String::from("can not get remote")));
        }
        if output
            .stdout
            .iter()
            .map(|&byte| byte as char)
            .collect::<String>()
            != format!(
                "{}\t{} (fetch)\n{}\t{} (push)\n",
                "origin", remote_path, "origin", remote_path
            )
        {
            let output = Command::new("git")
                .current_dir(&local_path)
                .args(["remote", "add", "origin", &remote_path])
                .output()?;
            if !output.status.success() {
                return Err(Error::RuntimeError(String::from("can not add remote")));
            }
        }

        let output = Command::new("git")
            .current_dir(&local_path)
            .args(["pull", "origin", "main", "--depth=1"])
            .output()?;
        if !output.status.success() {
            return Err(Error::RuntimeError(String::from("can not pull")));
        }

        Ok(())
    }
}
