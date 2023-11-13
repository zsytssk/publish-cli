use crate::tools;
use crate::venus::{self, TierType};

pub fn open() -> Result<(), String> {
    let tiers = match venus::get_tiers(vec![
        TierType::More,
        TierType::My,
        TierType::Stage,
    ]) {
        Ok(t) => t,
        Err(e) => panic!("get_tiers err: {}", e),
    };

    let names: Vec<String> = tiers
        .iter()
        .map(|item| {
            if item.is_stage {
                String::from(format!("stage:{}", &item.tier_name))
            } else {
                String::from(&item.tier_name)
            }
        })
        .collect();

    let tier_index = match tools::select("choose tier", &names) {
        Ok(v) => v,
        Err(e) => panic!("user choose err: {}", e),
    };

    println!("{:?}", &tiers[tier_index]);
    let url = format!(
        "https://test.com/deployment/tierManage?tier_id={}",
        &tiers[tier_index].tier_id
    );

    match open::that(&url) {
        Ok(()) => {}
        Err(err) => {
            return Err(format!(" open url error = '{}'", err.to_string()))
        }
    };
    Ok(())
}
