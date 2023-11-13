use crate::tools::{copy_to_clipboard, select};
use crate::utils::format_images_info;
use crate::venus::{self, get_images, get_tier_info, TierType};

pub fn copy() -> Result<(), String> {
    let tiers = match venus::get_tiers(vec![TierType::My]) {
        Ok(t) => t,
        Err(e) => panic!("get_tiers err={}", e),
    };

    let names: Vec<String> = tiers
        .iter()
        .map(|item| String::from(&item.tier_name))
        .collect();

    let tier_index = match select("choose tier", &names) {
        Ok(v) => v,
        Err(e) => panic!("user choose tier err: {}", e),
    };

    let tier = match get_tier_info(&tiers[tier_index]) {
        Ok(v) => v,
        Err(e) => panic!("get tier info err: {}", e),
    };

    let (_, image_name) = tier.image_url.rsplit_once('/').unwrap();
    println!("Tier信息: {} {} {}", tier.env_name, tier.status, image_name);

    let mut s = String::from("Pods:");
    for item in &tier.pod_list {
        s = format!("{} {}:{} ", s, item.pod_name, item.status)
    }

    let images = get_images(&tier)?;
    let names = format_images_info(&images);

    let index = match select("chose image", &names) {
        Ok(v) => v,
        Err(e) => panic!("user choose image err: {}", e),
    };

    println!("复制镜像：{}", &images[index].image_address);

    copy_to_clipboard(&images[index].image_address);

    println!("复制镜像 成功");
    Ok(())
}
