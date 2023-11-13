use std::{collections::HashMap, sync::OnceLock, time::Duration};

use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use ureq::Agent;

use crate::config::get_config;

pub type Data<T> = HashMap<String, HashMap<String, T>>;

pub static AGENT: OnceLock<Agent> = OnceLock::new();
fn init_agent() -> Agent {
    ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(10))
        .timeout_write(Duration::from_secs(10))
        .build()
}

pub fn request<U>(query: &str, variables: Value) -> Result<U, String>
where
    U: DeserializeOwned,
{
    let config = get_config();

    let graphql = String::from(&config.graphql);
    let token = String::from(config.get_token());

    // let now = Instant::now();
    let agent = AGENT.get_or_init(|| init_agent());
    // println!("bench request: {:?}", now.elapsed());

    let body = json!({
        "query": query,
        "variables": variables,
    });

    agent
        .post(&graphql)
        .set("authorization", &token)
        .send_json(&body)
        .map_err(|e| e.to_string())?
        .into_json()
        .map_err(|e| e.to_string())
}
