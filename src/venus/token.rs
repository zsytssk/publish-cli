use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{config::get_config, tools::request};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenData {
    token: String,
}

static QUERY: &str =
    "mutation getToken($username: String!, $password: String!) {
    getToken(username: $username, password: $password, app_client: 1) {
      token
    }
}";
pub fn get_token() -> Result<String, String> {
    let config = get_config();
    let variables = json!({"username": &config.username,"password":String::from(&config.password) });

    let token_data: request::Data<TokenData> =
        request::request(QUERY, variables)?;

    let token = &token_data["data"]["getToken"].token;
    Ok(String::from(token))
}
