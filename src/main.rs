mod app;
mod common;
mod config_file;
mod mark;
mod projects;

use std::path::PathBuf;

use clap::Parser;
use common::{default_dir, parse, spawn_shell, write_file_create_dir};
use config_file::Config;
use projects::Projects;

/// A utility to handle projects through the CLI. Similar to "open recent" in other programs.
/// Running the program without any arguments will open the projects list. The search bar is
/// automatically active and is powered by fuzzy search for easy searching.
/// To add a direcotry as a project cd to it, and run `proj -m`.
///
/// By default:
///
/// - config file:   $HOME/.config/proj/config.toml
///
/// - projects file: $HOME/.config/proj/projects.toml
///
#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    /// Add the current directory to the projects list.
    #[arg(short, long)]
    mark: bool,

    /// Specify a custom configuration file.
    #[arg(short, long)]
    config_file: Option<PathBuf>,

    /// Specify a custom projects list file.
    #[arg(short, long)]
    projects_file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // parse config file from custom path, or parse from default default path,
    // or create default config
    let config = if let Some(path) = cli.config_file {
        parse(&path)?
    } else {
        let path = default_dir()?.join("config.toml");
        parse(path).unwrap_or(Config::default())
    };

    // parse projects from cutstom path, or parse from default path
    let (projects_path, mut projects) = if let Some(path) = cli.projects_file {
        (path.clone(), parse(&path)?)
    } else {
        let path = default_dir()?.join("projects.toml");
        (path.clone(), parse(path).unwrap_or(Projects::default()))
    };

    // run in mark mode
    if cli.mark {
        mark::mark_current_directory(&mut projects)?;
        write_file_create_dir(projects_path, &toml::to_string_pretty(&projects)?)?;
        return Ok(());
    }

    // run in default mode (search app)
    let exit_dir = app::run(config, projects)?;
    if let Some(exit_dir) = exit_dir {
        spawn_shell(exit_dir);
    }

    Ok(())
}
