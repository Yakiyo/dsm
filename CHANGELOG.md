# dsm

## 1.0.2

### Patch Changes

- aa9d43b: use macos for platform name in url schemes

## 1.0.1

### Patch Changes

- e206502: fix issues with paths not showing correctly on bash like terminal on windows (?)
- a4b43b7: Remove default alias
- a4b43b7: Support aliases and latest format in uninstall

## 1.0.0

### Major Changes

- 490ac52: First stable release (v1.0.0)

### Patch Changes

- dfbce19: handle ensuring dsm dir better. print which dir actually throwed the error
- dfbce19: Support sdk channels in latest. latest/dev or latest-beta patterns are now supported.
- ef86c8a: clean up aliases after uninstalling
- 7cc2504: Print necessary config envs of dsm when `dsm env` is used alongside bin path
- 53fca11: create subcommand for checking new available versions of the app from gh
- 4130af5: Shell script to install dsm
- 0085fba: Generate completion scripts in runtime via the completions subcommand

## 0.1.2

### Patch Changes

- 88f1e27: Handle unexpected panics
- 88f1e27: Better list command. print associated aliases alongside versions
- 88f1e27: Default subcommand for setting and viewing default versions. automatically set new installation as default if no prior one exists
- 88f1e27: Support aliases. aliases ca now be created and removed using the `alias` and `unalias` commands
- 88f1e27: Better version handling. Take aliases as well semvers as inputs

## 0.1.1

- More updates

## 0.1.0

- Inital version
