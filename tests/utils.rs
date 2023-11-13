use whale_cli::{
    config::{get_config, set_config_token},
    venus::{self, TierType},
};

pub fn init_config() {
    match whale_cli::parse_config() {
        Ok(config) => config,
        Err(e) => panic!("parse config err: {}", e.to_string()),
    };
    // println!("bench parse_config: {:?}", now.elapsed());
    let token = match whale_cli::venus::get_token() {
        Ok(t) => t,
        Err(e) => panic!("parse config err: {}", e),
    };
    // println!("bench get_token: {:?}", now.elapsed());

    set_config_token(String::from(&token));
}

pub fn parse_config() {
    // let now = Instant::now();
    // whale_cli::parse_config().expect();
    let token = &get_config().token;

    // println!("bench set_config_token: {:?}", now.elapsed());

    let _user_info = match venus::get_user_info(String::from(token)) {
        Ok(t) => t,
        Err(e) => panic!("get_user_info err: {}", e),
    };

    // println!("bench user_info: {:?}", now.elapsed());

    let tiers = match venus::get_tiers(vec![
        TierType::My,
        TierType::Stage,
        TierType::More,
    ]) {
        Ok(t) => t,
        Err(e) => panic!("get_tiers err={}", e),
    };

    // println!("bench get_tiers: {:?}", now.elapsed());

    let tier = match venus::get_tier_info(&tiers[0]) {
        Ok(v) => v,
        Err(e) => panic!("get tier info err: {}", e),
    };

    // println!("bench get_tier_info: {:?}", now.elapsed());

    let _images = match venus::get_images(&tier) {
        Ok(img) => img,
        Err(e) => panic!("get_images err={}", e),
    };
    // println!("bench get_images: {:?}", now.elapsed());

    // copy_to_clipboard(&images[0].image_address);
    // println!("bench copy_to_clipboard: {:?}", now.elapsed());
}
