use reqwest::redirect;
use std::time::Duration;

mod create_job;
mod list_agents;
mod list_jobs;
mod get_job_result;

#[derive(Debug)]
pub struct Client {
    pub http_client: reqwest::blocking::Client,
    pub server_url: String,
}

impl Client {
    pub fn new(server_url: String) -> Client {
        let http_timeout = Duration::from_secs(5);
        let http_client = request::blocking::Client::builder()
            .redirect(redirect::Policy::limited(4))
            .timeout(http_timeout)
            .build()
            .expect("api....: budl HTTP client")

            Client {
                http_client,
                server_url
            }
    }
}