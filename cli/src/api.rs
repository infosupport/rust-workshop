use model::{PagedResult, Task};

mod model;

pub trait TaskApiClient {
    fn list(&self, page_num: u8) -> Result<PagedResult<Task>, reqwest::Error>;
}

pub struct ApiClient {
    api_key: String,
    host_name: String,
    http_client: reqwest::blocking::Client,
}

impl ApiClient {
    pub fn new(api_key: String) -> Self {
        ApiClient {
            api_key: api_key,
            // TODO Update host name when we have a hosted version
            host_name: "http://localhost:3000".to_string(),
            http_client: reqwest::blocking::Client::new(),
        }
    }
}

impl TaskApiClient for ApiClient {
    fn list(&self, page_num: u8) -> Result<PagedResult<Task>, reqwest::Error> {
        let url = format!("{}/v1/todos?page={}", &self.host_name, page_num);
        log::debug!("GET {}", url);
        let response = self
            .http_client
            .get(url)
            .header("X-Api-Key", &self.api_key)
            .send();

        match response?.error_for_status() {
            Ok(body) => {
                log::debug!("Looks good so far!");
                body.json::<PagedResult<Task>>()
            }
            Err(error) => {
                log::error!(
                    "Remote API at {} returned response with status {}",
                    error.url().unwrap(),
                    error.status().unwrap()
                );
                Err(error)
            }
        }
    }
}
