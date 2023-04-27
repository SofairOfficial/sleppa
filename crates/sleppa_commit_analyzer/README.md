# Sleppa commit message parser

Original [semantic-release](https://github.com/semantic-release/semantic-release/discussions) is a very powerful tool to operate semantic release. It automates the whole package release workflow including: determining the next version number, generating the release notes, and publishing the package.

As we are using squash-and-merge strategy to keep a clean and lean history, we have to be able to read the message of squashed commits.
Our strategy is as follows :

- Creating a new branch to operate change. The branch name will be the message of the squashed commits.
- Do some commits on this branch (named inner commit) with valid conventionnal commit message.
- Create a pull request (PR) with a valid name like: `Issue to solve (#3)` where the number `3` is the number of the PR.
- Squash-and-merge the PR with the valid name.

## How it works

If the squash-and-merged strategy is applied from the beginning, the first initial commit is the only one which is not.
Therefore, each following commit will be a PR (with a valid name) containing inner commits.
From the last known tag, these PR will be peeled to analyze their inner commit's messages. As these messages look like the
provided `grammar`, they can be parsed to match the type of the `ReleaseAction`.

The table below shows which commit message gets you which release type from the `grammar` and a `Regex format` :

| Commit message example                                                            | Release type |
| --------------------------------------------------------------------------------- | ------------ |
| `break(cpu): upgrade from 32 to 64 bits`                                          | Major        |
| `feat(cpu): add L1 cache`                                                         | Minor        |
| `fix(cpu): add RAM memory to allow swapping`                                      | Patch        |

| Grammar using Regex format (defined in the `sleppa.tom`)                          | Release type |
| --------------------------------------------------------------------------------- | ------------ |
| `^(?P<type>break){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$`                         | Major        |
| `^(?P<type>build|ci|docs|feat){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$`            | Minor        |
| `^(?P<type>fix|perf|refac|sec|style|test){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$` | Patch        |

### View of a squashed PR with inner commits

![Alt text](https://user-images.githubusercontent.com/15166875/229083489-82a73e59-7f64-468a-88f7-8714d0630e37.png "squashed commit")