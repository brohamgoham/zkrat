use common::api::{JobList, self};

use super::Client;
use crate::{config, Error};

impl Client {
    pub fn list_jobs(&self) -> Result<Vec<api::Job>, Error> {
        let get_job_route = format!("{}/api/jobs", config::SERVER_URL);

        let res = self.http_client.get(get_job_route).send()?;
        let api_res: api::Response<JobList> = res.json()?;

        if let Some(err) = api_res.error {
            return Err(Error::Internal(err.message));
        }

        Ok(api_res.data.unwrap().jobs)
    }
}
