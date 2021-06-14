pub mod new_project;

use crate::command::new_project::NewProject;
use crate::data::AppData;
use druid::DelegateCtx;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

// pub const CMD_NEW_PROJ: CommandIdentity = CommandIdentity::ColonPrefixed("n", "new");

/// actual definition of a command...
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum Identity {
    ColonPrefixed(String, String),
    SingleChar(String),
}
impl Identity {
    pub fn matches(&self, selector: &Selector) -> bool {
        match self {
            Identity::ColonPrefixed(a, b) => match selector {
                Selector::ColonPrefixed(s_a) => s_a.starts_with(a) || s_a.starts_with(b),
                Selector::SingleChar(_) => false,
            },
            Identity::SingleChar(a) => match selector {
                Selector::SingleChar(s_a) => a == s_a,
                Selector::ColonPrefixed(_) => false,
            },
        }
    }
}

///
/// simple version created from user-input, use to check vs the Identited
#[derive(Debug)]
pub enum Selector {
    ColonPrefixed(String),
    SingleChar(String),
}

#[derive(Debug)]
pub struct ParsingError;
impl std::error::Error for ParsingError {}
impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid command")
    }
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

// ====================

pub fn resolve<'r>(
    id: String,
    ctx: &'r DelegateCtx,
    app_data: &'r mut AppData,
) -> Result<impl Executable + 'r, ParsingError> {
    // todo somehow lift the identitied out; cannot be const due to String. Maybe load from conf?
    let cmd_new_proj = Identity::ColonPrefixed(":n".to_string(), ":new".to_string());
    match Selector::try_from(id.as_str()) {
        Ok(selector) => {
            if cmd_new_proj.matches(&selector) {
                println!("Found New Project Command");
                // Ok(NewProject {
                //     args: id.to_string(),
                //     file_name: None,
                // })

                Ok(NewProject::new("filename", id, app_data, ctx))

                // Err(ParsingError)
            } else {
                Err(ParsingError)
            }
        }
        Err(_) => Err(ParsingError),
    }
}

// ====================
#[derive(Debug)]
pub struct CommandExecutionError;
impl std::error::Error for CommandExecutionError {}
impl Display for CommandExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error while executing command")
    }
}

pub trait Executable {
    fn execute(&mut self) -> Result<u128, CommandExecutionError>;
}

trait HasCommandIdentity {
    fn identity(&self) -> &Identity;
}

trait HasArguments {
    fn args(&self) -> String;
    fn process_args(&mut self);
}
