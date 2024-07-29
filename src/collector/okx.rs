use api_services::{exchanges::okx::OkxExchange, utils::read_configs};
use std::collections::HashMap;

struct OkxAccountCollector {
    clients: Vec<OkxExchange>,
}

impl OkxAccountCollector {
    fn new(account_names: &Vec<String>) -> Self {
        let mut clients: Vec<OkxExchange> = Vec::new();

        for account_name in account_names {
            let configs: HashMap<String, String> = read_configs("configs.json", account_name);
            let client: OkxExchange = OkxExchange::new(&configs);
            clients.push(client);
        }

        Self { clients }
    }
}
