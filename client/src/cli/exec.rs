use std::{thread::sleep, time::Duration};

use crate::{api, Error};
use uuid::Uuid;

pub fn run(api_client: &api::Client, agent_id: &str, command: &str) -> Result<(), Error> {
    let agent_id = Uuid::parse_str(agent_id)?;
    let sleep_for = Duration::from_secs(500);

    let input = common::api::CreateJob {
        agent_id,
        command: command.trim().to_string(),
    };

    let job_id = api_client.create_job(input)?;

    loop {
        let job_output = api_client.get_job_result(job_id)?;
        if let Some(job_output) = job_output {
            println!("{}", job_output);
            break;
        }
        sleep(sleep_for);
    }

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let client = api::Client::new("http://localhost:8080".to_string());
        let agent_id = "c7f31787-ef12-4548-a30a-73096347af64";
        let command = "ls -la";
        run(&client, agent_id, command).unwrap();
    }
}