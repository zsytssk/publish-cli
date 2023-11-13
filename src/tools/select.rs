use console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::io::{Error, ErrorKind, Result};

pub fn select(title: &str, items: &Vec<String>) -> Result<usize> {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(title)
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => Ok(index),
        None => Err(Error::new(ErrorKind::Other, "User did not select anything")),
    }
}
