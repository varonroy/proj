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
* Alternatively, copy the binary to `/usr/bin/`
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
