# Sleppa commit message parser

Original [semantic-release](https://github.com/semantic-release/semantic-release/discussions) is a very powerful tool to operate semantic release. It automates the whole package release workflow including the computation of next [semantic release](https://semver.org) number, the generation of release notes and the publication of the final package.

This crate `sleppa_commit_analyzer` is opiniated because it uses a squash-and-merge commit strategy. However Sleppa wants to be agnostic of commit strategies. Another commit analyzer, using a rebase strategy for instance, can be used in Sleppa thanks to the trait `Repository`.

As we are using squash-and-merge strategy to keep a clean and lean history, we have to be able to read the message of squashed commits.
Our strategy is as follows :

- Creating a new branch to operate change. The branch name will be the message of the squashed commits.
- Do some commits on this branch (named inner commit) with valid conventionnal commit message.
- Create a pull request (PR) with a valid name like: `Issue to solve (#3)` where the number `3` is the number of the PR.
- Squash-and-merge the PR with the valid name.

## How it works

If the squash-and-merged strategy is applied from the beginning, the first initial commit is the only one which is not.
Therefore, each following commit will be a PR (with a valid name) containing inner commits.

```
Branch to merge
^
|-squash commit name (#13)
                        ^
                        |----pull request number
                             ^
                             |-- inner commit 1: "feat(github): some inner commit message"
                             |-- inner commit 2: "docs: some inner commit message"
```

From the last known tag, these PR will be peeled to analyze their inner commit messages. As these messages look like the
provided `grammar`, they can be parsed to match the type of the `ReleaseAction`.

The table below shows which commit message gets you which release type from the `grammar` and a `Regex format` :

| Commit message example                                                            | Release type |
| --------------------------------------------------------------------------------- | ------------ |
| `break(cpu): upgrade from 32 to 64 bits`                                          | Major        |
| `feat(cpu): add L1 cache`                                                         | Minor        |
| `fix(cpu): add RAM memory to allow swapping`                                      | Patch        |

| Grammar using Regex format (defined in the `sleppa.toml`)                         | Release type |
| --------------------------------------------------------------------------------- | ------------ |
| `^(?P<type>break){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$`                         | Major        |
| `^(?P<type>build\|ci\|docs\|feat){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$`         | Minor        |
| `^(?P<type>fix\|perf\|refac\|sec\|style\|test){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$` | Patch        |
