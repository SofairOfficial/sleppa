# Sleppa

This project aims to provide a semantic-release for Rust written project in a Rust written way.

Original [semantic-release](https://github.com/semantic-release/semantic-release/discussions) is a very powerful tool to operate semantic release. It automates the whole package release workflow including: determining the next version number, generating the release notes, and publishing the package.

## How it works

The module `repositories` of the `sleppa_primitives` analyzes a given repository to retrieve the `commits` since the last tag. GitHub reposiroties are now supported but others git file system could be added thanks to the trait `Reposiroty`.

These `commits` are given to the `sleppa_commit_analyzer` which will analyze their message. In order to proceed, a configuration file that defines the release rules is loaded.
The crate determines the new release action type to apply.

The crate `sleppa_versioner` uses this new release action type to determine the new tag of the repository.

Hence the crate `sleppa_changelog` uses all these informations to write the new changelog and to commit the file into the reposiroty. To commit the file, credentials must be loaded however.

### Release action definition

The following table describes each `<type>` of commit and how the latter impacts (i.e. increments) the `MAJOR`, `MINOR` and/or `PATCH` digits of a [semantic version](https://semver.org).

| Type      | Category        | Description                                                                                      | Versioning | Example                                                                        |
| --------- | --------------- | ------------------------------------------------------------------------------------------------ | ---------- | ------------------------------------------------------------------------------ |
| **break** | _Development_   | Breaking changes that causes a new major version of a component to be launched                   | `MAJOR`    | `break(service): new feature impacting the data model`                         |
| **build** | _Development_   | Changes related to the build system (involving configurations or tools) and package dependencies | `MINOR`    | `build(cargo): bump tokio-tower to version 1.5.2`                              |
| **ci**    | _Development_   | Changes impacting the CI/CD pipeline (e.g. GitHub Actions scripts, tools, ...)                   | `MINOR`    | `docs(changelog): update CHANGELOG to new version 0.1.1`                       |
| **docs**  | _Documentation_ | Changes impacting the project documentation                                                      | `MINOR`    | `docs(changelog): update CHANGELOG to new version 0.1.1`                       |
| **feat**  | _Production_    | Changes related to new backward-compatible features or functionalities                           | `MINOR`    | `feat(largo): implement Quic/RPC API server`                                   |
| **fix**   | _Production_    | Changes related to backward-compatible bug fixes                                                 | `PATCH`    | `fix(service): correctly resolve shorthand property declarations`              |
| **perf**  | _Production_    | Changes related to backward-compatible performance improvements                                  | `PATCH`    | `perf(net): use of non-blocking data structures for faster packets processing` |
| **refac** | _Development_   | Changes that restructure/rewrite the code base (not a new feature or a bug fix)                  | `PATCH`    | `refac(largo): adopt a graph data model for the storage engine`                |
| **sec**   | _Production_    | Changes related to backward-compatible security improvements                                     | `PATCH`    | `sec(net): use TLS 1.3`                                                        |
| **style** | _Development_   | Changes that do not affect the meaning of the source code (e.g. indentation, whitespaces, ...)   | `PATCH`    | `style(largo): bump indentation to 4 blank characters`                         |
| **test**  | _Development_   | Changes related to tests (i.e. refactoring or adding tests)                                      | `PATCH`    | `test(service): implement property-based tests on financial algorithms`        |

These types and versionning are the default implementation of `Sleppa`.

## Licenses and copyright

All contributions to this project are licensed under either of the following licenses:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)