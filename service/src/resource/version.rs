use crate::resource::*;

use ::error::Error;

use reqwest::Client;

impl LocalResource {
    pub fn version(&self) -> Result<String, Error> {
        Ok(std::fs::read_to_string(format!(
            "{}/{}/{}/{}",
            HOME.as_str(),
            self.dist,
            self.repo,
            "version"
        ))?)
    }
}

impl RemoteResource {
    pub async fn version(&self) -> Result<String, Error> {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let request = client.get(format!(
            "{}/{}/{}/{}/{}",
            self.resource_server, self.user, self.repo, self.branch, "version"
        )).header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0");

        let response = request.send().await?;

        Ok(response.text().await?)
    }
}
