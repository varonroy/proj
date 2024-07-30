use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{anyhow, Context};
use serde::de::DeserializeOwned;

use crate::projects::ExitTo;

/// The default directory where the projects and configuration files site
pub fn default_dir() -> anyhow::Result<PathBuf> {
    let home = home::home_dir().ok_or(anyhow!("Could not get the path to the home directory"))?;
    Ok(home.join(".config").join("proj"))
}

/// Creates all the directories leading to a file, then creates the file.
pub fn write_file_create_dir(path: impl AsRef<Path>, content: &str) -> anyhow::Result<()> {
    let path = path.as_ref();
    let prefix = path.parent().context(format!(
        "Getting parent directory of project file {:?}.",
        path
    ))?;
    std::fs::create_dir_all(prefix).unwrap();
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn parse<T: DeserializeOwned>(path: impl AsRef<Path>) -> anyhow::Result<T> {
    let content = std::fs::read_to_string(path)?;
    let value = toml::from_str::<T>(&content)?;
    Ok(value)
}

pub fn spawn_shell(exit_to: ExitTo) {
    let args = if exit_to.command.is_empty() {
        vec![format!("{}", std::env::var("SHELL").unwrap())]
    } else {
        exit_to.command
    };

    // let shell = ;
    // tmux new ";" neww ";" splitw
    // let shell = "tmux";
    let mut args = args.into_iter();
    let initial = args.next().unwrap();
    Command::new(initial)
        .current_dir(exit_to.dir)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn shell process")
        .wait()
        .expect("Erorr while handling command");
}
