use super::Client;
use crate::{config, Error};
use common::api::{AgentList, self};

impl Client {
    pub fn list_agents(&self) -> Result<Vec<api::Agent>, Error> {
        let get_agents_route = format!("{}/api/agents", config::SERVER_URL.to_string());

        let res = self.http_client.get(get_agents_route).send()?;
        let api_res: api::Response<AgentList> = res.json().expect("Failed to parse response");


        Ok(api_res.data.unwrap().agents)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list_agents() {
        let client = Client::new(config::SERVER_URL.to_string());
        let agents = client.list_agents().unwrap();
        dbg!(agents);
    }
}