use reqwest;

use crate::resource::{LocalResource, RemoteResource};

impl LocalResource {
    pub fn version(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(std::fs::read_to_string(format!(
            "{}/{}/{}",
            self.dist, self.repo, "version"
        ))?)
    }
}

impl RemoteResource {
    pub async fn version(&self) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let request = client.get(format!(
            "{}/{}/{}/{}/{}",
            self.resource_server, self.user, self.repo, self.branch, "version"
        ));

        let response = request.send().await?;

        Ok(response.text().await?)
    }
}
