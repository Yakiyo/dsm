#!/usr/bin/env -S node --loader @swc-node/register/esm

import toml from "toml";
import { readFile, writeFile } from "fs/promises";
import assert from "assert";

const CARGO_TOML = new URL("../Cargo.toml", import.meta.url).pathname;

async function run() {
	const { version: newVersion } = (await readFile(
		new URL("../package.json", import.meta.url),
		"utf-8",
	).then(JSON.parse)) as Record<string, string>;
	
	assert(newVersion, "Unable to get version from package.json");
	const oldToml = await readFile(CARGO_TOML, "utf-8");

	const { version: oldVersion } = toml.parse(oldToml).package as {
		version: string;
	};
	assert.notEqual(newVersion, oldVersion, "Old and new versions are the same");
	const newToml = oldToml.replace(
		`version = '${oldVersion}'`,
		`version = '${newVersion}'`,
	);
	assert.notEqual(newToml, oldToml, "Version did not change");

	await writeFile(CARGO_TOML, newToml, "utf-8");
}

run();