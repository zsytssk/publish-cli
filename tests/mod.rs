mod utils;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::utils;

    #[test]
    fn parse_config() {
        let now = Instant::now();

        utils::init_config();
        for _ in 0..10 {
            utils::parse_config();
        }

        println!("bench copy_to_clipboard: {:?}", now.elapsed());
    }
}
