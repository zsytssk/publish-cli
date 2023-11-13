use crate::tools;
use crate::utils;
use crate::venus::{self, TierType, UserInfo};

pub fn deploy(user: &UserInfo) -> Result<(), String> {
    let tiers = match venus::get_tiers(vec![TierType::More, TierType::My]) {
        Ok(t) => t,
        Err(e) => panic!("get_tiers err: {}", e),
    };

    let names: Vec<String> = tiers
        .iter()
        .map(|item| String::from(&item.tier_name))
        .collect();

    let tier_index = match tools::select("choose tier", &names) {
        Ok(v) => v,
        Err(e) => panic!("user choose err: {}", e),
    };

    let tier = match venus::get_tier_info(&tiers[tier_index]) {
        Ok(v) => v,
        Err(e) => panic!("get tier info err: {}", e),
    };

    let (_, image_name) = tier.image_url.rsplit_once('/').unwrap();
    println!("Tier信息: {} {} {}", tier.env_name, tier.status, image_name);

    let mut s = String::from("Pods:");
    for item in &tier.pod_list {
        s = format!("{} {}:{} ", s, item.pod_name, item.status)
    }

    let images = venus::get_images(&tier)?;
    let names = utils::format_images_info(&images);

    let index = match tools::select("chose image", &names) {
        Ok(v) => v,
        Err(e) => panic!("user choose image err: {}", e),
    };

    println!("选择镜像：{}", &images[index].image_address);

    let deploy_info = venus::deploy_image(user, &tier, &images[index])?;

    println!("发布镜像 成功：{}", &deploy_info.workflow_instance_id);
    Ok(())
}
