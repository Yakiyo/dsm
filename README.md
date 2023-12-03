# DSM - Dart SDK Manager
[![ci](https://github.com/Yakiyo/dsm/actions/workflows/ci.yml/badge.svg)](https://github.com/Yakiyo/dsm)

Simplified version manager for the Dart SDK. Inspired by [fnm](https://github.com/Schniz/fnm) and [nvm](https://github.com/nvm-sh/nvm). 

## Installation
To quickly install dsm, you can use one of the following scripts

#### bash (linux/macos/windows)
```bash
$ curl -fsSL https://dsm-vm.vercel.app/install.sh | bash
```
The bash script works on windows too if you have git bash, since it comes with the associated utilities like `uname`, `curl` and others. The script accepts options like the `-F` or `--filename` flag to override what file to use (see the release section to see the available ones). Run the `--help` flag to see available ones. For example
```bash
$ curl -fsSL https://dsm-vm.vercel.app/install.sh | bash -s -- --install-dir "path/to/file" -F "x86_64-unknown-linux-musl" --skip-shell
```
#### powershell (windows)
```powershell
$ irm https://dsm-vm.vercel.app/install.ps1 | iex
```
You can specify the version by declaring a `$v` variable beforehand and change the installation dir by a env variable named `DSM_INSTALL`
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
Windows users using Git Bash should check out this issue: https://github.com/Yakiyo/dsm/issues/20

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

## Logs
Dsm logs additional information throughout the program. The level of the emitted logs can be manually customized using the `DSM_LOG` environment variable. The env takes one of the following values:

- error
- warn
- info
- debug
- trace

The default is error. If `info` is set, then all logs of level info, warn and error will be emitted. Similar for the others too. See the docs at [`env_logger`](https://docs.rs/env_logger/0.10.0/env_logger/#enabling-logging) for additional details

Colors can be disabled either via the `--disable-colors` flag or by setting `DSM_LOG_STYLE` env to "never".

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
