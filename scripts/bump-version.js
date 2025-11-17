#!/usr/bin/env node

/**
 * Version bumping script for ChenChen
 * Updates version across package.json, Cargo.toml, and tauri.conf.json
 *
 * Usage:
 *   npm run version:patch  # 0.1.0 -> 0.1.1
 *   npm run version:minor  # 0.1.0 -> 0.2.0
 *   npm run version:major  # 0.1.0 -> 1.0.0
 *   npm run version 1.2.3  # Set to specific version
 */

import { readFileSync, writeFileSync } from 'fs';

const PACKAGE_JSON = 'package.json';
const CARGO_TOML = 'src-tauri/Cargo.toml';
const TAURI_CONF = 'src-tauri/tauri.conf.json';

function parseVersion(version) {
  const match = version.match(/^(\d+)\.(\d+)\.(\d+)$/);
  if (!match) {
    throw new Error(`Invalid version format: ${version}`);
  }
  return {
    major: parseInt(match[1]),
    minor: parseInt(match[2]),
    patch: parseInt(match[3]),
  };
}

function bumpVersion(currentVersion, type) {
  const v = parseVersion(currentVersion);

  switch (type) {
    case 'major':
      return `${v.major + 1}.0.0`;
    case 'minor':
      return `${v.major}.${v.minor + 1}.0`;
    case 'patch':
      return `${v.major}.${v.minor}.${v.patch + 1}`;
    default:
      // Assume it's a specific version
      parseVersion(type); // Validate format
      return type;
  }
}

function updatePackageJson(newVersion) {
  const path = PACKAGE_JSON;
  const content = JSON.parse(readFileSync(path, 'utf8'));
  const oldVersion = content.version;
  content.version = newVersion;
  writeFileSync(path, JSON.stringify(content, null, 2) + '\n');
  return oldVersion;
}

function updateCargoToml(newVersion) {
  const path = CARGO_TOML;
  let content = readFileSync(path, 'utf8');
  content = content.replace(
    /^version = "[\d.]+"/m,
    `version = "${newVersion}"`
  );
  writeFileSync(path, content);
}

function updateTauriConf(newVersion) {
  const path = TAURI_CONF;
  const content = JSON.parse(readFileSync(path, 'utf8'));
  content.version = newVersion;
  writeFileSync(path, JSON.stringify(content, null, 2) + '\n');
}

function main() {
  const bumpType = process.argv[2];

  if (!bumpType) {
    console.error('Usage: npm run version <patch|minor|major|x.y.z>');
    process.exit(1);
  }

  try {
    // Read current version from package.json
    const pkg = JSON.parse(readFileSync(PACKAGE_JSON, 'utf8'));
    const currentVersion = pkg.version;

    // Calculate new version
    const newVersion = bumpVersion(currentVersion, bumpType);

    console.log(`Bumping version: ${currentVersion} -> ${newVersion}`);

    // Update all files
    updatePackageJson(newVersion);
    console.log(`✓ Updated ${PACKAGE_JSON}`);

    updateCargoToml(newVersion);
    console.log(`✓ Updated ${CARGO_TOML}`);

    updateTauriConf(newVersion);
    console.log(`✓ Updated ${TAURI_CONF}`);

    console.log(`\nVersion bump complete! New version: ${newVersion}`);
    console.log('\nNext steps:');
    console.log('  1. Review changes: git diff');
    console.log(`  2. Commit: git commit -am "chore: bump version to ${newVersion}"`);
    console.log(`  3. Tag: git tag v${newVersion}`);
    console.log('  4. Push: git push && git push --tags');

  } catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
  }
}

main();
