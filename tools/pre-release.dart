/// Pre-release script
///
/// The script checks that version has been bumped and that
/// an entry for the version has been added to `CHANGELOG.md`

import '_utils.dart';
import 'package:toml/toml.dart';
import 'dart:io';

void main() async {
  // Parse Cargo.toml and extract current version
  final version = await TomlDocument.load("Cargo.toml")
      .then((value) => value.toMap())
      .then((value) => "${value['package']['version']}");

  // Check if the current version is already released or not
  // final git = await Process.run(
  //   "git",
  //   ["tag", "-l"],
  // );
  // if (git.exitCode != 0) {
  //   eprint("Command error. Returned exitcode ${git.exitCode}");
  //   exitCode = git.exitCode;
  //   return;
  // }
  // final gitTags = (git.stdout as String).split("\n");
  // gitTags.removeLast();

  // if (gitTags.contains("v$version")) {
  //   eprint("A git tag for v$version already exists");
  //   exitCode = 1;
  //   return;
  // }

  // Ensure entry for current version is there on `CHANGELOG.md` in
  // the format of `# {{ version }} (Unreleased)
  final md = await File("CHANGELOG.md").readAsLines();

  if (!md.contains("# $version (Unreleased)")) {
    eprint("An entry for v$version does not exist in Changelog."
        "It must be appended with an `(Unreleased)` suffix in the title");
  }
  final index = md.indexOf("# $version (Unreleased)");
  md[index] = "# $version";

  // Remove the `(Unreleased)` suffix
  File("CHANGELOG.md").writeAsString(md.join("\n"));
}
