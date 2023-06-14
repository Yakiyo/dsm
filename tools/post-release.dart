/// A script to handle post-release works like adding a
/// section for the next version to the CHANGELOG and bumping the version
/// in CARGO.toml

import 'dart:io';
import 'package:version/version.dart';
import 'package:toml/toml.dart';

void main() async {
  final document =
      await TomlDocument.load("Cargo.toml").then((value) => value.toMap());

  final version =
      Version.parse(document['package']['version'] as String).incrementPatch();

  final md = File("CHANGELOG.md");
  final content = ["# ${version.toString()} (Unreleased)", "- TODO!", ""];
  content.addAll((await md.readAsLines()));

  md.writeAsString(content.join("\n"));
  document['package']['version'] = version.toString();

  File("Cargo.toml").writeAsString(TomlDocument.fromMap(document).toString());
}
