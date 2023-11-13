use std::collections::HashMap;

use crate::tools::{request, run_in_thread};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::tier_list::TierItemInfo;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TierInfo {
    #[serde(default)]
    pub tier_id: u64,
    #[serde(default)]
    pub tier_name: String,
    #[serde(default)]
    pub service_id: String,
    #[serde(default)]
    pub env_name: String,
    #[serde(default)]
    pub cluster_name: String,
    #[serde(default)]
    pub image_url: String,
    #[serde(default)]
    pub pod_list: Vec<PodInfo>,
    #[serde(default)]
    pub name_space: String,
    #[serde(default)]
    pub env_template_id: String,
    #[serde(default)]
    pub env_id: String,
    #[serde(default)]
    pub env_status: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PodInfo {
    pub pod_name: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EnvInfo {
    id: String,
    flow_id: String,
    name: String,
    status: String,
}

pub fn get_tier_info(tier_item_info: &TierItemInfo) -> Result<TierInfo, String> {
    // let now = Instant::now();

    let tier_id = tier_item_info.tier_id.parse::<u64>().unwrap();
    let mut tier_info = TierInfo::default();
    tier_info.tier_id = tier_id;

    let var = json!({
      "req": {
        // "workload_id":  tier_info.service_id,
        // "env_name":  tier_info.env_name,
        "tier_id":  tier_id.to_string(),
      }
    });

    let res1 = run_in_thread(move || get_tier_base_info(&tier_id));
    let res2 = run_in_thread(move || get_tier_env_info());
    let res3 = run_in_thread(move || get_tier_status_info(var));

    let tier_data = res1.join().unwrap();

    tier_info.env_name = tier_data.env_name;
    tier_info.tier_name = tier_data.tier_name;
    tier_info.service_id = tier_data.service_id;
    tier_info.name_space = tier_data.name_space;
    tier_info.cluster_name = tier_data.cluster_name;
    // println!("bench tier_info1: {:?}", now.elapsed());

    let env_name = String::from(&tier_info.env_name);
    let env_list = res2.join().unwrap();
    let env_info = env_list
        .into_iter()
        .find(|item| item.name == env_name)
        .ok_or("cant find match env!")?;

    tier_info.env_id = String::from(&env_info.flow_id);
    tier_info.env_template_id = String::from(&env_info.id);
    tier_info.env_status = String::from(&env_info.status);
    // println!("bench tier_info2: {:?}", now.elapsed());

    let tier_data = res3.join().unwrap();
    tier_info.status = tier_data.status;
    tier_info.image_url = tier_data.image_url;
    tier_info.pod_list = tier_data.pod_list;
    // println!("bench tier_info3: {:?}", now.elapsed());

    Ok(tier_info)
}

fn get_tier_base_info(tier_id: &u64) -> Result<TierInfo, String> {
    let variables = json!({
      "req": {
        "tier_id":  tier_id,
      }
    });

    // let now = Instant::now();
    let mut tier_data_wrap: request::Data<TierInfo> = request::request(TIER_QUERY, variables)?;
    // println!("bench get_tier_base_info: {:?}", now.elapsed());

    let tier_data = tier_data_wrap
        .remove("data")
        .unwrap()
        .remove("tierService_getTierInfo")
        .unwrap();

    Ok(tier_data)
}
fn get_tier_status_info(variables: Value) -> Result<TierInfo, String> {
    let mut tier_data_wrap: request::Data<TierInfo> = request::request(TIER_INFO_QUERY, variables)?;

    let tier_data = tier_data_wrap
        .remove("data")
        .unwrap()
        .remove("deployService_getCurrentServiceInfoByRancher")
        .unwrap();

    Ok(tier_data)
}

type EnvInfoWrap<T> = request::Data<HashMap<String, T>>;
fn get_tier_env_info() -> Result<Vec<EnvInfo>, String> {
    let variables = json!({
      "data": {
        "status":  "Enable",
        "limit":  999,
        "offset":  0,
      }
    });

    let mut env_list_wrap: EnvInfoWrap<Vec<EnvInfo>> =
        request::request(TIER_ENV_LIST_QUERY, variables)?;

    let env_data = env_list_wrap
        .remove("data")
        .unwrap()
        .remove("ticketTemplateListWithTypes")
        .unwrap()
        .remove("tasktemps")
        .unwrap();

    if env_data.len() == 0 {
        return Err(format!("cant find tier env info"));
    }

    Ok(env_data)
}

static TIER_QUERY: &str = "query tierService_getTierInfo($req: Tier_GetTierInfoReq_Input) {
  tierService_getTierInfo(req: $req) {
    tier_id
    tier_name
    service_id
    service_name
    config
    parent_tier_id
    parent_tier_name
    env
    flow_id
    create_user
    product_line
    name_space
    expiration_date
    tier_port_map
    resources
    register_name
    is_expired
    domain_name
    env_name
    cluster_name
    project_name
  }
  }";

static TIER_INFO_QUERY: &str = "query deployService_getCurrentServiceInfoByRancher(
	$req: Chameleon_GetCurrentServiceInfoReq_Input
  ) {
	deployService_getCurrentServiceInfoByRancher(req: $req) {
	  workload_id
	  status
	  meta
	  image_url
	  pod_list {
		pod_name
		status
	  }
	  ports
	  resource
	}
  }";

static TIER_ENV_LIST_QUERY: &str = "query ticketTemplateListWithTypes($data: TaskTemplateListReq) {
	ticketTemplateListWithTypes(data: $data) {
	  tasktemps {
		id
		flow_id
		name
		status
		task_type
		create_time
		start_time
		end_time
		category_id
		category_name
		operator
		operator_name
		use_times
	  }
	}
  }";
