# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.2.0](https://github.com/cocogitto/cocogitto-bot/compare/0.1.0..0.2.0) - 2023-11-30
#### Bug Fixes
- base64-decode `cog.toml` contents - ([a71dada](https://github.com/cocogitto/cocogitto-bot/commit/a71dada3f72c2fcf48e876f079e986ec0881caa9)) - Mark S
- pull cog.toml from repo default branch - ([48888ef](https://github.com/cocogitto/cocogitto-bot/commit/48888ef48b33b2ea86234a513500614043f93492)) - Mark S
- restore previous `ignore_merge_commits` behavior - ([c160f7a](https://github.com/cocogitto/cocogitto-bot/commit/c160f7af2bbdb7c3e58f34f6918811abd93e2470)) - Mark S
- display only one hash on PR with a single commit - ([d4079a6](https://github.com/cocogitto/cocogitto-bot/commit/d4079a6745d19613a33db3ce62e7e67384bff246)) - [@oknozor](https://github.com/oknozor)
- use git signature when github login is not found - ([ee19677](https://github.com/cocogitto/cocogitto-bot/commit/ee19677085764f2d3b0089492d4c01110da61d65)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- fix ghcr.io namespace - ([d081e2c](https://github.com/cocogitto/cocogitto-bot/commit/d081e2cea36a095c22b879f5a132f7002e4b7726)) - [@oknozor](https://github.com/oknozor)
- fix deocker image - ([80fbc3d](https://github.com/cocogitto/cocogitto-bot/commit/80fbc3d4450cef194eb448653db643bc9b6219cb)) - [@oknozor](https://github.com/oknozor)
- fix tag condition - ([166f102](https://github.com/cocogitto/cocogitto-bot/commit/166f102e188cf8727ee371086ab9fd5fbf8d3cba)) - [@oknozor](https://github.com/oknozor)
- use ghcr.io instead of docker hub - ([460c61e](https://github.com/cocogitto/cocogitto-bot/commit/460c61e2c684bb990d24c43d14793da2c597cf55)) - [@oknozor](https://github.com/oknozor)
- add build cache - ([cc1b044](https://github.com/cocogitto/cocogitto-bot/commit/cc1b04413304f5c998e3c51683e81f92fcf6f1ae)) - [@oknozor](https://github.com/oknozor)
- use cocogitto for release - ([e155a24](https://github.com/cocogitto/cocogitto-bot/commit/e155a24b7f26db556cb926a1f5b89f50d9e82fe9)) - [@oknozor](https://github.com/oknozor)
- fix workflow error - ([c07b5f5](https://github.com/cocogitto/cocogitto-bot/commit/c07b5f588966501ec312b8c8d437b7ab8df5a8fc)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- add installation instructions to README - ([190ee01](https://github.com/cocogitto/cocogitto-bot/commit/190ee01a484ffd0d3ebe50c0b938ceb611297ec0)) - [@oknozor](https://github.com/oknozor)
#### Features
- add health endpoint - ([f3208bb](https://github.com/cocogitto/cocogitto-bot/commit/f3208bb4dda0c8a4251a3c77afa514671de5f63c)) - [@oknozor](https://github.com/oknozor)
- add vergen build info - ([70de57e](https://github.com/cocogitto/cocogitto-bot/commit/70de57e210a60a7a517267cb0cff5f2bc0adaa3f)) - [@oknozor](https://github.com/oknozor)
- add autometrics prometheus exporter - ([b6cb540](https://github.com/cocogitto/cocogitto-bot/commit/b6cb54058c3e98fb3eb7b266f7065dadbf39aa7e)) - [@oknozor](https://github.com/oknozor)
- add `default_branch` field to `PullRequestRepository` struct - ([5ca2772](https://github.com/cocogitto/cocogitto-bot/commit/5ca2772dc9cbe24b02ce09899c2017430864811e)) - Mark S
- add docker build and CI - ([f46182e](https://github.com/cocogitto/cocogitto-bot/commit/f46182e74d02d86c2518cb991c8032f968575577)) - [@oknozor](https://github.com/oknozor)
- respect `cog.toml` if it exists in target repo - ([fabb99b](https://github.com/cocogitto/cocogitto-bot/commit/fabb99b9ff5fafa6d19b4da1f00b7da56c281e9d)) - Mark S
- parse commit messages with `cocogitto` instead of `conventional_commit_parser` - ([bb97439](https://github.com/cocogitto/cocogitto-bot/commit/bb9743949b031835858f08dd28cd14f8b2d26a5b)) - Mark S
- use pagination to fetch commits - ([3b84847](https://github.com/cocogitto/cocogitto-bot/commit/3b84847266701a4728e41365904510638935f34e)) - [@oknozor](https://github.com/oknozor)
- add X emoji on failure report - ([5fcc57a](https://github.com/cocogitto/cocogitto-bot/commit/5fcc57ae12655267d51ecd7bd67465f5b2d95542)) - [@oknozor](https://github.com/oknozor)
- ignore merge commits and pull-request 'closed' event - ([73c84fc](https://github.com/cocogitto/cocogitto-bot/commit/73c84fc6bf388b2aebe5b2717525f22c16fc1624)) - [@oknozor](https://github.com/oknozor)
- improve successfull check-run report comment format - ([d566ca2](https://github.com/cocogitto/cocogitto-bot/commit/d566ca20b438c2253ad37d41ddb0a0d3ba7b843d)) - [@oknozor](https://github.com/oknozor)
#### Miscellaneous Chores
- **(dependencies)** replace `conventional_commit_parser` with `cocogitto` - ([3fcead7](https://github.com/cocogitto/cocogitto-bot/commit/3fcead7b947f7a1bb0516799fefa4372e9b8d629)) - Mark S
- **(housekeeping)** remove unused imports and dead code - ([8ef4fd2](https://github.com/cocogitto/cocogitto-bot/commit/8ef4fd2aae054ac4a16a606409268e963b7303c6)) - Mark S
- cargo machete - ([58c9b6d](https://github.com/cocogitto/cocogitto-bot/commit/58c9b6da1b84465e7e0a23a4eb45c8f1e3749502)) - [@oknozor](https://github.com/oknozor)
- fmt all - ([261edbe](https://github.com/cocogitto/cocogitto-bot/commit/261edbeae2259816f23d243a62ada052b6820f7d)) - [@oknozor](https://github.com/oknozor)
- use cocogitto next - ([943defd](https://github.com/cocogitto/cocogitto-bot/commit/943defdc338d384facdd0a16c5f8ce7e102b5c2f)) - [@oknozor](https://github.com/oknozor)
- update dependencies - ([58165bf](https://github.com/cocogitto/cocogitto-bot/commit/58165bfa8939635a7efa82688d3df6d0a9907901)) - [@oknozor](https://github.com/oknozor)
- update authentication for compat with octocrab 0.28.0 - ([84b65e1](https://github.com/cocogitto/cocogitto-bot/commit/84b65e1d8712cbfccfb7acb12ec40bad3a1400f4)) - Mark S
- ignore local cargo configuration overrides - ([c6f4b39](https://github.com/cocogitto/cocogitto-bot/commit/c6f4b39210b71a0a1a2a4cc18d4519bc556ee429)) - Mark S
- add privacy policy - ([edfdfa8](https://github.com/cocogitto/cocogitto-bot/commit/edfdfa8ea04713ba07fcf936e97ce65737ddc6c2)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- migrate to axum - ([d069196](https://github.com/cocogitto/cocogitto-bot/commit/d069196af3a8b9879b3cebf025552825428169c6)) - [@oknozor](https://github.com/oknozor)

- - -

## [0.1.0](https://github.com/cocogitto/cocogitto-bot/compare/bd3147bf87e01e1e4eeaed607db38e9fcf86cf27..0.1.0) - 2021-11-21
#### Continuous Integration
- add cog.toml config - ([5b12341](https://github.com/cocogitto/cocogitto-bot/commit/5b12341f538f385fdd368ab9574687ae972b3a98)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- add a readme - ([c0f54e5](https://github.com/cocogitto/cocogitto-bot/commit/c0f54e51f6a90ae04e56071608ec5935c644d72f)) - [@oknozor](https://github.com/oknozor)
#### Features
- show unmanaged event type - ([061819a](https://github.com/cocogitto/cocogitto-bot/commit/061819a3299e0fd6af4a74cafc3f4064b96953bb)) - [@oknozor](https://github.com/oknozor)
- implement pull request checks - ([6a087b7](https://github.com/cocogitto/cocogitto-bot/commit/6a087b79538f15e2c66c6f60e94944832243d94f)) - [@oknozor](https://github.com/oknozor)
- first implementation - ([6d0651d](https://github.com/cocogitto/cocogitto-bot/commit/6d0651d029240d2415d9623c8636a30a5efe34e8)) - [@oknozor](https://github.com/oknozor)
#### Miscellaneous Chores
- format and lints - ([7de9d52](https://github.com/cocogitto/cocogitto-bot/commit/7de9d52602fc338fb4bb581d7d5589c34e008387)) - [@oknozor](https://github.com/oknozor)
- remove unused dependencies - ([a82dfe1](https://github.com/cocogitto/cocogitto-bot/commit/a82dfe15c278704a180b345ffbe91e9a81a22379)) - [@oknozor](https://github.com/oknozor)
- use port 8080 - ([4967fd3](https://github.com/cocogitto/cocogitto-bot/commit/4967fd38d8aa6a06ef59e1ae06af01957a7c0faa)) - [@oknozor](https://github.com/oknozor)
- fmt all - ([f449d8c](https://github.com/cocogitto/cocogitto-bot/commit/f449d8c1171ec857eb3b8bab5e82d98989317087)) - [@oknozor](https://github.com/oknozor)
- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).