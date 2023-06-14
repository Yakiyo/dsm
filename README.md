# DSM - Dart SDK Manager
[![ci](https://github.com/Yakiyo/dsm/actions/workflows/ci.yml/badge.svg)](https://github.com/Yakiyo/dsm)

Simplified version manager for the Dart SDK. Inspired by [fnm](https://github.com/Schniz/fnm) and [nvm](https://github.com/nvm-sh/nvm). This is currently **work in progress**, contributions are welcome, please open an issue before contributing mentioning what changes you are planning to do in order to prevent the same thing being done by multiple people.

Pre-release versions are available for now on crates.io

## Installation

With cargo:
```bash
$ cargo install dsm
```
After installation, you should setup your shell. The `dsm env <SHELL>` command is used for that. Currently powershell, bash. zsh, cmd and fish are supported. As an example, for setting up bash or zsh, add the following line in your `.bashrc` or `.zshrc` file:
```bash
$ eval $(dsm env bash)
# or for zsh
$ eval $(dsm env zsh)
```
Pre-built binaries are available in the github [release](https://github.com/Yakiyo/dsm/releases) section. Download, rename file and add it manually to your path.

More installation methods will be added later on.

## Usage

For installing a specific version
```bash
$ dsm install 3.0.3
```
Then use the `use` command to activate it
```bash
$ dsm use 3.0.3
```
For creating aliases, use the `alias` and `unalias` commands
```bash
$ dsm alias 3.0.3 latest

$ dsm unalias latest
```

Installed versions can be viewed with the `list` command. For uninstalling, use the `uninstall` command. Aliases are not yet supported but are planned. Due to lack of proper index of dart releases, shorthands like `latest` or `lts` are not possible but I'm trying to find a workaround for it.

For a list of all commands, do `dsm --help`.

## Credits
A huge amount of code and inspiration has been taken from Schniz's [`fnm`](https://github.com/Schniz/fnm). The project structure, some of the hacks and codes are based from it. 

## Author

**dsm** © [Yakiyo](https://github.com/Yakiyo). Authored and maintained by Yakiyo.

Released under [MIT](https://opensource.org/licenses/MIT) License