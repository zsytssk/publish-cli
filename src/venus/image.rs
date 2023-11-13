use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    config::get_config,
    tools::request::{self, Data},
};

use super::{tier::TierInfo, user::UserInfo};

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageInfo {
    pub create_time: String,
    pub branch: String,
    pub creator: String,
    pub image_address: String,
    pub id: String,
}

type ImageListWrap<T> = HashMap<String, request::Data<T>>;
pub fn get_images(tier_info: &TierInfo) -> Result<Vec<ImageInfo>, String> {
    let config = get_config();
    let variables = json!({
        "form_slug": "cd_image_list",
        "limit": config.show_img_num,
        "offset": 0,
        "filters": json!({
          "workload_id": tier_info.service_id,
          "tier_id": tier_info.tier_id,
        })
    });

    let mut img_list_wrap: ImageListWrap<Vec<ImageInfo>> =
        request::request(IMAGE_QUERY, variables)?;

    let img_list = img_list_wrap
        .remove("data")
        .unwrap()
        .remove("searchFormValue")
        .unwrap()
        .remove("items")
        .unwrap();

    Ok(img_list)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeployInfo {
    pub workflow_instance_id: String,
}

pub fn deploy_image(
    user: &UserInfo,
    tier: &TierInfo,
    image: &ImageInfo,
) -> Result<DeployInfo, String> {
    if tier.cluster_name != "product-dev" {
        return Err(String::from("为了安全考虑只支持发布product-dev的镜像"));
    }

    let variables = json!({
        "req": {
            "company_id": user.company_id,
            "user_id": user.user_name,
            "user_name": user.nick_name,
            "image_id": image.id,
            "workload_id": tier.service_id,
            "tier_id": tier.tier_id.to_string(),
            "workflow_template_id": tier.env_template_id,
            "workflow_id":  tier.env_id,
            "tier_attribute": "TierAttributeImage",
        }
    });

    let mut deploy_info_wrap: Data<DeployInfo> =
        request::request(DEPLOY_QUERY, variables)?;

    let deploy_info = deploy_info_wrap
        .remove("data")
        .unwrap()
        .remove("releaseService_createCdWorkflowProcess")
        .unwrap();

    Ok(deploy_info)
}

static IMAGE_QUERY: &str = "query searchFormValue(
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

static DEPLOY_QUERY: &str = "mutation releaseService_createCdWorkflowProcess(
    $req: Chameleon_CreateWorkflowRequest_Input
    ) {
    releaseService_createCdWorkflowProcess(req: $req) {
      workflow_instance_id
    }
    }";
