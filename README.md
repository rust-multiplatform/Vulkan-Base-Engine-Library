<<<<<<< .merge_file_Njs5g2
# Rust Multi-Platform: Base Project Template

[![Multiplatform Build](https://github.com/rust-multiplatform/Base-Project-Template/actions/workflows/multiplatform-build.yml/badge.svg)](https://github.com/rust-multiplatform/Base-Project-Template/actions/workflows/multiplatform-build.yml)
=======
# Rust Multi-Platform: Base Engine Library
>>>>>>> .merge_file_KMOBqx

> THIS IS A LIBRARY.

<<<<<<< .merge_file_Njs5g2
This project is supported by the following platforms:

- ✅ Platform: Windows
- ✅ Platform: Linux
- ✅ Platform: macOS
- ✅ Platform: Android
- ✅ Platform: iOS
- ✅ Platform: WebAssembly

To use this project simply fork it (button in top right corner) into your own namespace.
If you need some more advanced setup check out the [forking guide](FORKING.md).
We also include a checklist for [what to do after forking](FORKING.md#what-is-important-to-do-after-forking).
=======
This project is supported by **all** Rust platforms and is to be used by other projects of the Rust Multi-Platform and may or may not be useless outside the project scope.
>>>>>>> .merge_file_KMOBqx

## Licenses

This project is dual licensed in Rust's fashion:

- [MIT License](https://spdx.org/licenses/MIT.html)
- [Apache License 2.0](https://spdx.org/licenses/Apache-2.0.html)

For your own project you can chose whichever fits you better.
For templates/examples we recommend to also dual-licensing.

<<<<<<< .merge_file_Njs5g2
## Project layout

| Folder                                         | Description                                                                                                                         |
| ---------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| ./                                             | Workspace root; `Cargo.toml` contains all project folders (internal crates)                                                         |  |
| [platform/](platform/)                         | Platform projects root. Contains every platform this demonstration is supported on incl. instructions on how to build and use them. |
| [platform/android/](platform/android/)         | Contains the Android platform project and instructions on how to build this project for Android and run it.                         |
| [platform/ios/](platform/ios/)                 | Contains the iOS platform project and instructions on how to build this project for iOS and run it.                                 |
| [platform/linux/](platform/linux/)             | Contains the Linux platform project and instructions on how to build this project for Linux and run it.                             |
| [platform/macos/](platform/macos/)             | Contains the macOS platform project and instructions on how to build this project for macOS and run it.                             |
| [platform/windows/](platform/windows/)         | Contains the Windows platform project and instructions on how to build this project for Windows and run it.                         |
| [platform/webassembly/](platform/webassembly/) | Contains the WebAssembly platform project and instructions on how to build this project for Websites and run it.                    |
| [shared/](shared/)                             | Contains the **shared** code between **all** projects.                                                                              |

To break this down:
The [shared/](shared/) folder contains our **cross/multi-platform code**.
99.9% of what we do in this project will happen there.

Each of the projects inside [platform/](platform/) are representing a **platform specific project**.
In most cases, like for Windows, Linux and macOS, there is nothing else to do but call our shared code and compile a **binary**.
However, on certain platforms, like Android and iOS, we have to use some special commands and tools to get an e.g. .APK (Android) or .APP (iOS) file.
Furthermore, on those systems resources/assets may need to be specially loaded and their `Cargo.toml` will be much more extensive.
Simply said: For those special platforms we will use some cargo extensions which will automatically generate a native project in the background. Values from `Cargo.toml` will be used to generate those.

**Please check the `README.md` inside each platform to see how each platform is build, run and what you will need to do so.**

=======
>>>>>>> .merge_file_KMOBqx
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

<<<<<<< .merge_file_Njs5g2
[![Multiplatform Build](https://github.com/rust-multiplatform/Base-Project-Template/actions/workflows/multiplatform-build.yml/badge.svg)](https://github.com/rust-multiplatform/Base-Project-Template/actions/workflows/multiplatform-build.yml)

This project utilizes the GitHub Actions CI (= Continuous Integration) to showcase how to build for all platforms.
For most platforms we just need a runner on the target platform (Windows, Linux or macOS) and install Rust.
This can be simply done by following [rustup.rs](https://rustup.rs/) (check the [other install options](https://rust-lang.github.io/rustup/installation/other.html) for automatically installing inside an CI).
Something like:

```shell
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable --profile full -y
```

should work for most platforms.

Note that we _may_ need more tools installed depending on the CI provider and platform.
Check the host <-> target matrix at [Building & Running](#Building-&-Running)

Additionally, often we have to `source` the profile changes. Something like:

```shell
source $HOME/.cargo/env
```

Furthermore, we have templates for many other CI platforms if you prefer those:

- [Rust-Multiplatform / CircleCI Example](https://github.com/rust-multiplatform/CI-Example-CircleCI)
- [Rust Multiplatform / TeamCity Example](https://github.com/rust-multiplatform/CI-Example-TeamCity)
- [Rust Multiplatform / GitLab Example](https://github.com/rust-multiplatform/CI-Example-GitLab-CI)
- [Rust Multiplatform / BItBucket Example](https://github.com/rust-multiplatform/CI-Example-BitBucket-Pipelines)
- [Rust Multiplatform / Travis Example](https://github.com/rust-multiplatform/CI-Example-Travis)
- [Rust Multiplatform / Azure DevOps Example](https://github.com/rust-multiplatform/CI-Example-Azure-Pipelines)
- [Rust Multiplatform / AppVeyor Example](https://github.com/rust-multiplatform/CI-Example-AppVeyor)
=======
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

[![Build status](https://ci.appveyor.com/api/projects/status/oa83imlxgqueuyqk?svg=true)](https://ci.appveyor.com/project/Sakul6499/base-engine-library)

### CI Platform: Jenkins

[![Build Status](https://jenkins.sakul-flee.de/job/github_rust_multiplatform/job/Base-Engine-Library/job/main/badge/icon)](https://jenkins.sakul-flee.de/job/github_rust_multiplatform/job/Base-Engine-Library/job/main/)
>>>>>>> .merge_file_KMOBqx

## Coverage

[![codecov](https://codecov.io/github/rust-multiplatform/Base-Engine-Library/branch/main/graph/badge.svg?token=9bK3wW9oNA)](https://codecov.io/github/rust-multiplatform/Base-Engine-Library)

### Sunburst

![Sunburst](https://codecov.io/gh/rust-multiplatform/Base-Engine-Library/branch/main/graphs/sunburst.svg?token=9bK3wW9oNA)

### Grid

![Grid](https://codecov.io/gh/rust-multiplatform/Base-Engine-Library/branch/main/graphs/tree.svg?token=9bK3wW9oNA)

### Icicle

![Icicle](https://codecov.io/gh/rust-multiplatform/Base-Engine-Library/branch/main/graphs/icicle.svg?token=9bK3wW9oNA)

## Contributing & Getting Help

We welcome any help we get and try to answer questions as good as possible!
Generally speaking, please open an [issue here on GitHub](issues/new) or contact me directly.
No matter the problem or question.

In case you've got an idea/request for an example/template, please do open an [issue here on GitHub](issues/new).

Want to add your own example/template project to the organization and use our CI's?
Please open an [issue here on GitHub](issues/new).
