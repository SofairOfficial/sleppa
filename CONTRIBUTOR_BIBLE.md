# Contributor's Bible

## Contents

## Introduction

This guide provides all the necessary information for a contributor to land and work comfortably with the project's maintainers and contributors.

## Terminology

| Term        | Description                                           |
| ----------- | ----------------------------------------------------- |
| Contributor | A member of the community contributing to the project |
| Maintainer  | A member of the Sofair core team                      |

## How to contact the project's maintainers

If you do not find an answer to one of your questions in this document, or for any other request or inquiry you may have, do not hesitate to contact the project's maintainers at 'maintainers@sofair.io'. You will always find an attentive ear!

## Getting started with the project

[WIP] explain the forking and cloning (Hugo's part)

## How to write semantic commit messages

The project follows the [semantic-release](https://github.com/semantic-release/semantic-release) pattern to automatically determine a [semantic version](https://semver.org) number when building and publishing a package in the CI/CD pipeline.

A [semantic version](https://semver.org) number is made up of 3 digits separated by periods, namely `MAJOR.MINOR.PATCH`. For instance, `0.1.0` means a new minor release, `1.0.0` means a major release, and `0.0.1` means a patch. Please refer to the [semantic version](https://semver.org) specification for more details.

For such an automated semantic release process to work, the latter requires a strict format when writting commit messages, so that to be able to determine if a commit is in fact a breaking change (i.e. a major change), a new feature implementation (i.e. a minor change), a bug fix (i.e. a patch) or a documentation enhancement (i.e. a minor change), for instance. The main motivation when using such semantic commit with conventional format, is to:

- Generate CHANGELOG files automatically when building and publishing a new version of a component (e.g. crate, NPM packages, ...)
- Automatically determine the new [semantic version](https://semver.org) of a component, based on the type of commit messages. For instance, a breaking change increments the major number, while a bug fix bumps the patch number.
- Maintain a clean and easy-to-follow history of commits
- Communicating the nature of changes to teammates, the community, and other stakeholders
- Automate the release process, without implying human intervention

The present commit convention should be considered as a lightweight version of a mix between the [Conventional Commits Specification](https://www.conventionalcommits.org/) and the one of [Angular](https://github.com/angular/angular/blob/22b96b9/CONTRIBUTING.md#-commit-message-guidelines), both of which have been a valuable source of inspiration.

### Commit message syntax

Here's the general (somewhat opinionated) syntax for writing a (_scoped_ or _unscoped_) commit message:

`<type>[(scope)]: <content>`

The `(scope)` (within parenthesis) term is optional (hence the square brackets surrounding it). An _unscoped_ commit message is thus written as follows:

`<type>: <content>`

The aim of the `(scope)` term is to help contextualizing the commit. It indicates the section of the code base that is impacted by the commit, the name of the crate or the package, and so on. A properly formed commit message must be concise, yet meaningful, with the following <content> characteristics:

- **Only describe the what not the how**: a commit message must inform on the 'what' or the 'why', but not on the 'how'. It is preferable to describe the how in pull requests (PRs), code reviews (CRs), and issues.

- **Use imperative tense**: write "Fix bug" instead of "Fixed bug"

- **Single-line content**: although a commit message can technically be spread over several lines, dont't do it! In fact, don't move in commmit messages what should reside in PRs, in issues, in tasks or in the code documentation (e.g. algorithmic details, technical decisions, and so on). Code reviews is also a proper place for sharing on technical decisions or problems.

Commit types are usually categorised as follows:

- **Development** - sort of maintenance types which classify changes, intended the developers, that don’t actually affect the production code but rather the development environment and workflow internally

- **Production** - sort of enhancement types which classify changes, intended the end users, that solely affect the production code.

- **Documentation** - any modification pertaining the documentation of the project, including source code documentation, pull request comments, code reviews and so on.

Now let us describe each `<type>` of commit and how the latter impacts (i.e. increments) the `MAJOR`, `MINOR` and/or `PATCH` digits of a [semantic version](https://semver.org).

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

It is worth pointing out that any message that does not follow this strict syntax convention will be rejected by the continuous integration process.

### Browsing commits using their type

Leveraging on pre-defined commit types, it is easy to vagabond the history of commits, such as, for instance, to track new features, bug fixes and performance enhancements, using, for instance, the following `git` command:

`$ git log --oneline --grep "^feat\|^fix\|^perf"`
