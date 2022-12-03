# Rust Multi-Platform: Base Engine Library

> THIS IS A LIBRARY.

This project is supported by **all** Rust platforms and is to be used by other projects of the Rust Multi-Platform and may or may not be useless outside the project scope.

## Licenses

This project is dual licensed in Rust's fashion:

- [MIT License](https://spdx.org/licenses/MIT.html)
- [Apache License 2.0](https://spdx.org/licenses/Apache-2.0.html)

For your own project you can chose whichever fits you better.
For templates/examples we recommend to also dual-licensing.

## Targets & Architectures

This project is aiming to work across all platforms **and targets**.
All **Tier 1** targets are tested in CI's of this repository.
Additionally, _some_ **Tier 2** targets are tested.

However, this should work on all targets. If you find an issue please report it.

[Rust's Tier Policies](https://doc.rust-lang.org/rustc/target-tier-policy.html)
[Rust's Platform Support & Targets](https://doc.rust-lang.org/rustc/platform-support.html)

## Building

This is a simple Rust and Cargo project.  
Simply use the build-in commands:

For debug builds:  

```shell
cargo build
```

For release builds:  

```shell
cargo build --release
```

## Continuous Integration

### CI Platform: GitHub Actions

[![Base Engine Library](https://github.com/rust-multiplatform/Base-Engine-Library/actions/workflows/library.yml/badge.svg)](https://github.com/rust-multiplatform/Base-Engine-Library/actions/workflows/library.yml)

### CI Platform: Azure DevOps

[![Build Status](https://dev.azure.com/Rust-Multiplatform/Base%20Engine%20Library/_apis/build/status/Base%20Engine%20Library?branchName=main)](https://dev.azure.com/Rust-Multiplatform/Base%20Engine%20Library/_build/latest?definitionId=4&branchName=main)

### CI Platform: CircleCI

[![CircleCI](https://dl.circleci.com/status-badge/img/gh/rust-multiplatform/Base-Engine-Library/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/rust-multiplatform/Base-Engine-Library/tree/main)

### CI Platform: Travis

[![Build Status](https://app.travis-ci.com/rust-multiplatform/Base-Engine-Library.svg?branch=main)](https://app.travis-ci.com/rust-multiplatform/Base-Engine-Library)

### CI Platform: GitLab CI

[![pipeline status](https://gitlab.com/rust-multiplatform/base-engine-library/badges/main/pipeline.svg)](https://gitlab.com/rust-multiplatform/base-engine-library/-/commits/main)

### CI Platform: BitBucket Pipeline

[BitBucket Mirror, no badges 🙃](https://bitbucket.org/rust-multiplatform/base-engine-library/pipelines/)

### CI Platform: TeamCity

| Pipeline | Status Badge                                                                                                                                                      |
| -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Build    | ![TeamCity build status](https://teamcity.jetbrains.com/app/rest/builds/buildType:id:OpenSourceProjects_RustMultiplatform_BaseEngineLibrary_Build/statusIcon.svg) |
| Test     | ![TeamCity build status](https://teamcity.jetbrains.com/app/rest/builds/buildType:id:OpenSourceProjects_RustMultiplatform_BaseEngineLibrary_Test/statusIcon.svg)  |

### CI Platform: AppVeyor

[![Build status](https://ci.appveyor.com/api/projects/status/nsffumoc868yqsrj/branch/main?svg=true)](https://ci.appveyor.com/project/Sakul6499/base-project-template/branch/main)

Simple pipeline to showcase the usage of Rust on AppVeyor.  
Linux only currently, but [Windows](https://www.appveyor.com/docs/windows-images-software/) and [macOS](https://www.appveyor.com/docs/macos-images-software/) are possible.

### CI Platform: Jenkins

[![Build Status](https://jenkins.sakul-flee.de/job/github_rust_multiplatform/job/Base-Engine-Library/job/main/badge/icon)](https://jenkins.sakul-flee.de/job/github_rust_multiplatform/job/Base-Engine-Library/job/main/)

## Coverage

A combination of [grcov](https://github.com/mozilla/grcov) and [codecov.io](https://codecov.io) is used to provide code-to-test coverage.  
**Please note that it is impossible to reach 100% coverage on some platforms as e.g. bindgen-code (i.e. dynamically generated code / macros) is NOT covered by `grcov` and certain platform specific tools (like `cargo-apk`) generate additional code that also is NOT included in the coverage.**

Test-to-Code coverage status: [![codecov](https://codecov.io/gh/rust-multiplatform/Base-Project-Template/branch/main/graph/badge.svg?token=XpGvuQVirP)](https://codecov.io/gh/rust-multiplatform/Base-Project-Template)

Below are several charts showing/highlighting the distribution of **all platforms**.

### Sunburst

![Sunburst](https://codecov.io/gh/rust-multiplatform/Base-Project-Template/branch/main/graphs/sunburst.svg?token=XpGvuQVirP)

### Grid

![Grid](https://codecov.io/gh/rust-multiplatform/Base-Project-Template/branch/main/graphs/tree.svg?token=XpGvuQVirP)

### Icicle

![Icicle](https://codecov.io/gh/rust-multiplatform/Base-Project-Template/branch/main/graphs/icicle.svg?token=XpGvuQVirP)

## Contributing & Getting Help

We welcome any help we get and try to answer questions as good as possible!
Generally speaking, please open an [issue here on GitHub](issues/new) or contact me directly.
No matter the problem or question.

In case you've got an idea/request for an example/template, please do open an [issue here on GitHub](issues/new).

Want to add your own example/template project to the organization and use our CI's?
Please open an [issue here on GitHub](issues/new).
