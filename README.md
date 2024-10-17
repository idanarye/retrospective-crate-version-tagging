# Retrospective Crate Version Tagging

## Requirements

* [The GitHub CLI](https://github.com/cli/cli) (`gh`) must be installed and in the `PATH`.
* The authentication token used by the GitHub CLI must have the `workflow` scope.
  * To verify it has that scope, run `gh auth status` and look for `'workflow'` in the `Token scopes` list.
  * If it's not there, run `gh auth refresh --scopes workflow` and follow the instructions.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
