use anyhow::anyhow;

use crate::projects::{Project, Projects};

pub fn mark_current_directory(projects: &mut Projects) -> anyhow::Result<()> {
    let dir = std::env::current_dir()?;
    projects.projects.insert(Project {
        name: dir
            .file_name()
            .ok_or(anyhow!("could not"))?
            .to_string_lossy()
            .to_string(),
        dir,
    });

    Ok(())
}
