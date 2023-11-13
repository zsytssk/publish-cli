use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::tools::request;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub user_name: String,
    pub nick_name: String,
    pub company_id: String,
}

static USER_QUERY: &str = "query getUserInfoByToken(
	$token: String!
  ) {
	getUserInfoByToken(
	  token: $token
	) {
		user {
			user_name
			nick_name
			company_id
		}
	}
  }";

type UserInfoWrap<T> = request::Data<HashMap<String, T>>;
pub fn get_user_info(token: String) -> Result<UserInfo, String> {
    let variables = json!({"token": token });
    let mut user_data_wrap: UserInfoWrap<UserInfo> =
        request::request(USER_QUERY, variables)?;

    let user_data = user_data_wrap
        .remove("data")
        .unwrap()
        .remove("getUserInfoByToken")
        .unwrap()
        .remove("user")
        .unwrap();

    Ok(user_data)
}
