use std::{collections::HashSet, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Project {
    pub name: String,
    pub dir: PathBuf,
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
