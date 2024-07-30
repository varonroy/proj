use itertools::Itertools;
use std::collections::HashMap;
use tui::widgets::ListState;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use crate::projects::{ExitTo, Project, Projects};

pub enum RunState {
    Stay,
    Exit,
    ExitToDir(ExitTo),
}

#[derive(Clone)]
pub struct SearchResult {
    pub name: String,
    pub dir: String,
    pub project: Project,
    pub score: i64,
    pub fuzzy_match: Vec<usize>,
}

pub struct AppState {
    pub run_state: RunState,
    pub search: String,
    pub search_results_cache: HashMap<String, Vec<SearchResult>>,
    pub projects: Projects,
    matcher: SkimMatcherV2,
    list_state: ListState,
}

impl AppState {
    pub fn new(projects: Projects) -> Self {
        Self {
            run_state: RunState::Stay,
            search: "".to_string(),
            search_results_cache: HashMap::new(),
            projects,
            matcher: SkimMatcherV2::default(),
            list_state: ListState::default(),
        }
    }

    pub fn search_results(&mut self) -> Vec<SearchResult> {
        if !self.search_results_cache.contains_key(&self.search) {}

        match self.search_results_cache.get(&self.search) {
            Some(x) => x.clone(),
            None => {
                let search_results = self
                    .projects
                    .projects
                    .iter()
                    .filter_map(|project| {
                        self.matcher
                            .fuzzy_indices(&project.name, &self.search)
                            .map(|x| (project, x))
                    })
                    .map(|(project, (score, fuzzy_indices))| SearchResult {
                        name: project.name.clone(),
                        dir: project.exit_to.dir.to_string_lossy().to_string(),
                        project: project.clone(),
                        score,
                        fuzzy_match: fuzzy_indices,
                    })
                    .sorted_by(|a, b| a.score.cmp(&b.score))
                    .collect_vec();

                self.search_results_cache
                    .insert(self.search.clone(), search_results.clone());

                search_results
            }
        }
    }

    pub fn selected(&mut self) -> Option<Project> {
        self.list_state
            .selected()
            .map(|i| self.search_results()[i].project.clone())
    }

    pub fn list_state(&mut self) -> &mut ListState {
        self.update_list_state(0);
        &mut self.list_state
    }

    fn update_list_state(&mut self, offset: i32) {
        let len = self.search_results().len();

        if len == 0 {
            self.list_state.select(None);
            return;
        }

        let mut i = self.list_state.selected().unwrap_or_default() as i32;
        i += offset;
        i = i.min((len - 1) as _).max(0);
        self.list_state.select(Some(i as _));
    }

    pub fn character_typed(&mut self, ch: char) {
        self.search.push(ch);
        self.update_list_state(0);
    }

    pub fn backspace_typed(&mut self) {
        if !self.search.is_empty() {
            self.search = self.search[..self.search.len() - 1].to_string();
            self.update_list_state(0);
        }
    }

    pub fn down_typed(&mut self) {
        self.update_list_state(1);
    }

    pub fn up_typed(&mut self) {
        self.update_list_state(-1);
    }
}
