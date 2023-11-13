mod copy;
mod deploy;
mod open;

pub use self::open::open;
pub use copy::copy;
pub use deploy::deploy;

#[derive(Debug)]
pub enum Cmd {
    Deploy,
    Copy,
    Open,
}

impl Cmd {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "deploy" => Some(Cmd::Deploy),
            "copy" => Some(Cmd::Copy),
            "open" => Some(Cmd::Open),
            _ => None,
        }
    }
}
