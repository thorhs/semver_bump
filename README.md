# semver_bump

SemVer version bumper tool

semver_bump is a helper tool to bump SemVer compatible versions.  It can accept the version on the command line, or through stdin.

# Usage:
```
Usage: semver_bump <major|minor|patch> <version|-> [-b build_string] [-p pre-release_version]
input version must confirm to SemVer 2.0, with an optional v prefix
input version must contain major, minor and patch parts.
input version can optionally start with v, if it does, the output also starts with a v
input version can optionally have a -pre_release postfix
input version can optionally have a +build_string postfix
input version can optionally be -, causing semver_bump to read version from stdin
```

# Examples

Bump major version:
```bash
semver_bump major 1.1.1
```
returns 2.0.0

Bump minor version from stdin:
```bash
echo 2.3.4 | semver_bump minor -
```
returns 2.4.0

Bump patch version, include build string:
```bash
semver_bump patch 1.1.1 -b build142
```
returns 1.1.2+build142

Bump patch version, include pre-release version and build string:
```bash
semver_bump patch 1.1.1 -b build142 -p 3.1
```
returns 1.1.2-3.1+build142
