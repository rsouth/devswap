mod insert_mode;
pub mod new_project;
mod save_settings;
mod show_help;

use crate::command::insert_mode::InsertMode;
use crate::command::new_project::NewProject;
use crate::command::save_settings::SaveSettings;
use crate::command::Identity::{Parameterised, Simple};
use crate::data::AppState;
use druid::DelegateCtx;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

#[rustfmt::skip]
lazy_static! {
    static ref CMD_NEW_PROJ: Identity = Parameterised(":n".to_string(), ":new".to_string());
    static ref CMD_INSERT_MODE: Identity = Simple("i".to_string());
    static ref CMD_SAVE_SETTINGS: Identity = Parameterised(":ws".to_string(), ":write-settings".to_string());
}

/// `Identity` defines an `Executable`.
///
/// There are 2 types of command:
/// - `ColonPrefixed` e.g. ':n' or ':new'
/// - `SingleChar`    e.g i or ↑
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum Identity {
    Simple(String),
    Parameterised(String, String),
}
impl Identity {
    pub fn matches(&self, selector: &Selector) -> bool {
        match self {
            Identity::Parameterised(a, b) => match selector {
                Selector::ColonPrefixed(s_a) => s_a.starts_with(a) || s_a.starts_with(b),
                Selector::SingleChar(_) => false,
            },
            Identity::Simple(a) => match selector {
                Selector::SingleChar(s_a) => a == s_a,
                Selector::ColonPrefixed(_) => false,
            },
        }
    }
}

/// Partial `Identifier` for an `Executable`
///
/// Contains only part of the `Identifier` and can be matched to the full
/// `Identifier` using `matches`.
///
#[derive(Debug)]
pub enum Selector {
    ColonPrefixed(String),
    SingleChar(String),
}

impl TryFrom<&str> for Selector {
    type Error = ParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 1 {
            Ok(Selector::SingleChar(value.to_string()))
        } else if value.starts_with(':') {
            Ok(Selector::ColonPrefixed(value.to_string()))
        } else {
            Err(ParsingError)
        }
    }
}

#[derive(Debug)]
pub struct ParsingError;
impl std::error::Error for ParsingError {}
impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid command")
    }
}

// ====================

pub fn resolve<'r>(
    id: String,
    ctx: &'r DelegateCtx,
    app_data: &'r mut AppState,
) -> Result<Box<dyn Executable + 'r>, ParsingError> {
    match Selector::try_from(id.as_str()) {
        Ok(selector) => {
            if CMD_NEW_PROJ.matches(&selector) {
                println!("Found New Project Command");
                Ok(Box::new(NewProject::new("filename", id, app_data, ctx)))
            } else if CMD_INSERT_MODE.matches(&selector) {
                println!("Found Insert Mode Command");
                Ok(Box::new(InsertMode::new(app_data, ctx)))
            } else if CMD_SAVE_SETTINGS.matches(&selector) {
                println!("Found Save Settings Command");
                Ok(Box::new(SaveSettings::new(app_data)))
            } else {
                Err(ParsingError)
            }
        }
        Err(_) => Err(ParsingError),
    }
}

// ====================
#[derive(Debug)]
pub struct ExecutionError;
impl std::error::Error for ExecutionError {}
impl Display for ExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error while executing command")
    }
}

pub trait Executable {
    fn execute(&mut self) -> Result<u128, ExecutionError>;
}

trait HasCommandIdentity {
    fn identity(&self) -> &Identity;
}

trait HasArguments {
    fn args(&self) -> String;
    fn process_args(&mut self);
}
