use druid::{ArcStr, Data};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Data, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DocType {
    FileBased(String), // i.e. actual files, read from disk
    Hardcoded(String), // e.g. help / command output
    Empty,
}

#[derive(Data, Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub doc_type: DocType,
    // source: Option<String>,
    last_saved_ts: Option<u128>,
}

impl Header {
    pub fn new(doc_type: DocType) -> Header {
        Header {
            doc_type,
            last_saved_ts: None,
        }
    }
}

// thoughts on last_saved_ts - periodically save to a tmp file regardless if the user saves the
//                             file, that way system shutdowns etc don't lose data; always appreciated
//                             sublime not expecting me to save!

// idea: :md command to render as markdown, for 'reading mode' :)
