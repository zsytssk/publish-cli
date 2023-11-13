use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    config::{get_config, FilterEntities},
    tools::{request, run_in_thread, thread_pool::PoolJoinHandle},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct TierItemInfo {
    pub tier_id: String,
    pub create_time: String,
    pub service_name: String,
    pub tier_name: String,
    pub create_user: String,
    #[serde(default)]
    pub is_stage: bool,
}

static QUERY: &str = "query searchFormValue(
	$form_slug: String
	$filters: JSON
	$order_by: [OrderByEntity]
	$limit: Int
	$offset: Int
	$filter_entities: JSON
	$relation: Int
	$full_text_search_keyword: String
  ) {
	searchFormValue(
	  form_slug: $form_slug
	  filters: $filters
	  order_by: $order_by
	  offset: $offset
	  limit: $limit
	  filter_entities: $filter_entities
	  relation: $relation
	  full_text_search_keyword: $full_text_search_keyword
	) {
	  items
	}
  }";

pub enum TierType {
    My,
    More,
    Stage,
}

struct VariablesInfo {
    variables: Value,
    is_stage: bool,
}

type TiersWrap<T> = request::Data<HashMap<String, T>>;
pub fn get_tiers(
    tier_types: Vec<TierType>,
) -> Result<Vec<TierItemInfo>, String> {
    let mut res: Vec<TierItemInfo> = vec![];
    let mut var_list: Vec<VariablesInfo> = vec![];

    let config = get_config();
    for tier_type in tier_types {
        let mut is_stage = false;
        let filter_entities = match tier_type {
            TierType::My => &config.filter_entities_my,
            TierType::More => &config.filter_entities_more,
            TierType::Stage => {
                is_stage = true;
                &config.filter_entities_stage
            }
        };

        if filter_entities.len() == 0 {
            continue;
        }

        let relation = get_relation(filter_entities);
        let variables = json!({
          "form_slug": "tier_list",
          "limit": config.show_img_num,
          "offset": 0,
          "filter_entities": filter_entities,
          "relation": relation,
        });
        var_list.push(VariablesInfo {
            variables: variables,
            is_stage: is_stage,
        });
    }

    let mut handles: Vec<PoolJoinHandle<Result<Vec<TierItemInfo>, String>>> =
        vec![];
    for var_item in var_list {
        let th = run_in_thread(move || {
            let mut tier_data_wrap: TiersWrap<Vec<TierItemInfo>> =
                request::request(QUERY, var_item.variables)?;

            let mut tier_data: Vec<TierItemInfo> = tier_data_wrap
                .remove("data")
                .unwrap()
                .remove("searchFormValue")
                .unwrap()
                .remove("items")
                .unwrap();

            for item in &mut tier_data {
                item.is_stage = var_item.is_stage
            }

            Ok(tier_data)
        });
        handles.push(th);
    }

    for th in handles {
        let tier_data = th.join().unwrap();
        res.extend(tier_data);
    }

    Ok(res)
}

/** 如果有两个同样的filter_key，那就是or */
fn get_relation(filter_entities: &Vec<FilterEntities>) -> u16 {
    for (i, item) in filter_entities.iter().enumerate() {
        let li = filter_entities
            .iter()
            .rposition(|r| {
                r.filter_key == item.filter_key && r.operation == item.operation
            })
            .unwrap();

        if i != li {
            return 2;
        }
    }
    1
}
