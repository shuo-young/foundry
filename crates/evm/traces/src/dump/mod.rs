use std::{collections::HashMap, error::Error, ops::Add};

use alloy_primitives::{Address, Bytes};
use foundry_block_explorers::Client;
use foundry_common::abi::find_source;
use foundry_config::Chain;


#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AttackTrace {
    pub from: Address,
    pub to: Address,
    pub traces: Vec<Vec<usize>>,
    pub chain: Option<Chain>,
    pub api_key: Option<String>,
}

impl AttackTrace {
    pub fn new() -> Self {
        Self {
            from: Address::default(),
            to: Address::default(),
            traces: Vec::new(),
            chain: Option::default(),
            api_key: Option::default(),
        }
    }

    pub fn add_trace(&mut self, trace: Vec<usize>) {
        self.traces.push(trace);
    }

    pub async fn get_vuln_contract_code(&self) -> Result<(), Box<dyn Error>> {
        println!("Getting contract code for address: {}", self.to.clone());
        println!("Chain: {:?}", self.chain);
        println!("API Key: {:?}", self.api_key);
        let client = Client::new(self.chain.unwrap(), self.api_key.as_ref().unwrap())?;
        let source = find_source(client, self.to).await?;
        println!("{}: {:?}", self.to.clone(), source);
        Ok(())
    }

    pub fn get_vuln_called_funcs(&self) {
        for trace in &self.traces {
            let called_func = trace[trace.len()-1];
            
        }
    }
}

