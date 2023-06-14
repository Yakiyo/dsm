# 0.1.1 (Unreleased)
- Support aliases. Aliases can be created with the `alias` subcommand and removed via `unalias`
- Better version handling. Most commands can now take aliases instead of versions as input (i.e. the uninstall and use command). Semver Versions can now also be followed by a `v` in the start without causing issues.
- Better `list` command. List command now prints all associated aliases beside the version names
- Removed spinners. It was only used in the install command for two lines. Unnecessary bloat
- Better panic handling. Instead of the messy error message from compiler, panics now print a nice message advicing users to file an issue and also prints the error to a log file. Uses [`human-panic`](https://docs.rs/human-panic/latest/human_panic/) create for this.
- On new installation, if no `default` version exists, the newly installed one is tagged as default.
- `Default` subcommand. Can be used to set or view the current default version.

# 0.1.0
- Initial version