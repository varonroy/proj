# proj
A CLI project directory management tool.

![demo](./demo/demo.gif)

## Installation
1. Clone the project.
```
https://github.com/varonroy/proj
```
2. Build.
```
cd proj
cargo build --release
```
3. Add the build directory to the `PATH` in your `rc` file.
```
export PATH="$PATH:/path/to/proj/target/release"
```
* Alternatively, copy the binary to `/usr/bin/` or `/usr/local/bin`.
```
cp path/to/proj/target/release/proj /usr/bin/
```

## Usage
To add a directory to the projects list (mark it):
```
cd /path/to/my/project
proj --mark
```
To launch the tool:
```
proj
```
for more options use the `help` command:
```
proj --help
```

## Configuration
The projects list file is located at `~/.config/proj/projects.toml`.
Each projects entry has two fields:
- `dir` - the directory of the project.
- `command` - the command to execute when entering a project.
    - The command is written as a list of arguments.
    - If no command is provided, the default shell will be used.

For example:

```toml
[projects.exit_to]
dir = "/home/my-user/projects/my-project"
command = ["tmux", "new", ";", "neww", ";", "prev", ";"]
```

This configuration specifies a project named `my-project` which is located under `/home/my-user/projects/my-project`. The command specifies a series of `tmux` operations which will create a new session, then create a new window, and then navigate back to the first window.
