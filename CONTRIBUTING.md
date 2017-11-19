# Contributing to Little Engine
## Bugs
Include everything needed to reproduce the bug in your bug report.

## Pull Request Pre-Requisites
### Format your code
Run [rustfmt](https://github.com/rust-lang-nursery/rustfmt) on your
code BEFORE opening a PR. Ensure that the rules specified in 
[rustfmt.toml](./rustfmt.toml) are followed.

### Grab the latest development changes
Merge the latest development changes on master into your changeset 
before opening a PR. This ensures that your code works with the
latest changes and does not rely on outdated assumptions / API.
