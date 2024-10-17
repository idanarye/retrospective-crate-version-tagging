# Retrospective Crate Version Tagging

Workflows like like [release-please](https://github.com/googleapis/release-please) can manage the entire release cycle for you, but they only work if you manage everything from there. If you have a more conmplex release scheme (e.g. - multiple released crates in one repository) or if you didn't use release-please from the beginning and want to generate the release history, these workflows can't help you. You need something a bit more manual.

`retrospective-crate-version-tagging` is a CLI tool for generating GitHub releases for Rust projects based on the already existing release information:

* A changelog file in the [Keep a Changelog](https://keepachangelog.com).
* The versions uploaded to [crates.io](https://crates.io/).

`retrospective-crate-version-tagging` takes the release notes from the changelog and the commit hash from crates.io, and combines them to create GitHub releases.

## Requirements

* [The GitHub CLI](https://github.com/cli/cli) (`gh`) must be installed and in the `PATH`.
* The authentication token used by the GitHub CLI must have the `workflow` scope.
  * To verify it has that scope, run `gh auth status` and look for `'workflow'` in the `Token scopes` list.
  * If it's not there, run `gh auth refresh --scopes workflow` and follow the instructions.

## Installing

Install from crates.io using:

```bash
> cargo install retrospective-crate-version-tagging
```

## Usage

`retrospective-crate-version-tagging` has two commands:

* `retrospective-crate-version-tagging detect` - read the changelog and query crates.io to generate a YAML with information required to create the releases.
* `retrospective-crate-version-tagging create-releases` - receive (via STDIN) the YAML created by the first command and use `gh` to create GitHub releases from it.

The output of the first command can be redirected directly into the second command, or it can be stored in a file that can manually be inspected and edited before creating the versions.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
