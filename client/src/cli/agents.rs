use prettytable::{Cell, Row, Table};

use crate::{api, Error};

pub fn run(api_client: &api::Client) -> Result<(), Error> {
    let agents = api_client.list_agents().unwrap();

    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Agent ID"),
        Cell::new("Created At"),
        Cell::new("Last seen at"),
    ]));

    for agent in &agents {
        table.add_row(Row::new(vec![
            Cell::new(agent.id.to_string().as_str()),
            Cell::new(agent.created_at.to_string().as_str()),
            Cell::new(agent.last_seen_at.to_string().as_str()),
        ]));
    }

    table.printstd();

    Ok(())
}

