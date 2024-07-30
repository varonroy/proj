use std::{collections::HashSet, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ExitTo {
    pub dir: PathBuf,
    pub command: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Project {
    pub name: String,
    pub exit_to: ExitTo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Projects {
    pub projects: HashSet<Project>,
}

impl std::default::Default for Projects {
    fn default() -> Self {
        Self {
            projects: HashSet::new(),
        }
    }
}
