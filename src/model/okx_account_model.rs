// OKX exchange account information structs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct OkxAccount {
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub total_equity: f64,
    pub maintenance_margin_ratio: f64,
    pub positions: Vec<Position>,
    pub balances: Vec<Balance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub instrument_id: String,
    pub amount: f64,
    pub notional_value: f64,
    pub unrealized_pnl: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub instrument_id: String,
    pub amount: f64,
}

impl OkxAccount {
    pub fn new(name: String, total_equity: f64, mmr: f64) -> Self {
        Self {
            name,
            timestamp: Utc::now(),
            total_equity,
            maintenance_margin_ratio: mmr,
            positions: Vec::new(),
            balances: Vec::new(),
        }
    }

    pub fn add_position(&mut self, position_info: HashMap<String, String>) {
        todo!()
    }

    pub fn add_balance(&mut self, balance_info: HashMap<String, String>) {
        todo!()
    }

    // TODO: add database operations.
}
