# DSM - Dart SDK Manager
[![ci](https://github.com/Yakiyo/dsm/actions/workflows/ci.yml/badge.svg)](https://github.com/Yakiyo/dsm)

Simplified version manager for the Dart SDK. Inspired by [fnm](https://github.com/Schniz/fnm) and [nvm](https://github.com/nvm-sh/nvm). 

## Installation

### crates.io:
```bash
$ cargo install dsm
```

### Locally
Clone the repo first. Then build it. You need the rust toolchain installed in your local environment. Get it from [here](https://www.rust-lang.org/tools/install). You also need [git](git-scm.com).
```bash
$ git clone https://github.com/Yakiyo/dsm

$ cd dsm

$ cargo build --release
```

### Pre-built binaries
Pre-built binaries for some platforms are available in [github releases](https://github.com/Yakiyo/dsm/releases) section. You can download the file, rename it and add it to your system PATH.

### Scripts
Installation scripts will be added soon. If you'd like to contribute, please see https://github.com/Yakiyo/dsm/issues/10

After installation, setup your shell as mentioned in [setup](#setup).

## Setup
The `dsm env <SHELL>` command is used for setting up shell. Currently powershell, bash. zsh, cmd and fish are supported.

### Bash/Zsh
Add the following to your `.bashrc` or `.zshrc` file.
```bash
eval $(dsm env bash)
# or for zsh
eval $(dsm env zsh)
```

### Fish
Create `~/.config/fish/conf.d/dsm.fish` and add the following.
```fish
dsm env fish | source
```

### Powershell
Add the following to your powershell profile
```powershell
dsm env powershell | Out-String | Invoke-Expression
```
You can view the path to your profile with the `$PROFILE` variable.

### Command Prompt
Create a [startup script](https://superuser.com/questions/144347/is-there-windows-equivalent-to-the-bashrc-file-in-linux/144348#144348) and add the following.
```batch
FOR /f "tokens=*" %i IN ('dsm env cmd') DO CALL %i
```
## Usage

For installing a specific version
```bash
$ dsm install 3.0.3
```
You can use `latest` to install the latest stable sdk version. Latest versions of dev/beta channels are also supported via `latest/channel` or `latest-channel` format.
```bash
$ dsm install latest-dev # or dsm install latest/dev

$ dsm install latest # same as latest-stable or latest/stable
```

Then use the `use` command to activate it
```bash
$ dsm use 3.0.3
```
For creating aliases, use the `alias` and `unalias` commands
```bash
$ dsm alias 3.0.3 pinned

$ dsm unalias pinned
```
View all installed versions
```bash
$ dsm ls # or dsm list
```

For a list of all commands, do `dsm --help`.

## Contributing
Contributions are always welcome. You can start with any of the open [issues](https://github.com/Yakiyo/dsm/issues) to work on. For adding a new feature, please open an issue before working on it in order to discuss it. Feature commits are prefered so please open individual prs for individual features instead of doing several feature additions/changes in a single pull request. For any changes, add a changeset via `pnpm changeset` if its applicable. 

Make sure your code passes the CI and please merge and resolve conflicts from upstream before finalizing the pr. 

If you cannot code yourself but would like to request a feature, please open an issue and I'll see what can be done.

## Credits
A huge amount of code and inspiration has been taken from Schniz's [`fnm`](https://github.com/Schniz/fnm). The project structure, some of the hacks and codes are based from it. 

## Author

**dsm** © [Yakiyo](https://github.com/Yakiyo). Authored and maintained by Yakiyo.

Released under [MIT](https://opensource.org/licenses/MIT) License

If you like this project, consider leaving a star ⭐ and sharing it with your friends and colleagues.