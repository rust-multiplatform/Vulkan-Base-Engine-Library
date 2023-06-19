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

Furthermore, we have templates for many other CI platforms if you prefer those.

> Please note, that some CI platforms only showcase how to build a single platform (such as `platform_linux`). This is done to save on costs and time.
> 
> Furthermore, that if just one CI platform is failing, it is most likely linked to a plan or build credit issue on said platform.
> GitHub Actions is **NOT** limited for open source repositories.
> If GitHub Actions succeed, all other CI platforms _should_ succeed as well.

List of CI Examples/Templates:

- [Rust-Multiplatform / CircleCI Example](https://github.com/rust-multiplatform/CI-Example-CircleCI)
- [Rust Multiplatform / TeamCity Example](https://github.com/rust-multiplatform/CI-Example-TeamCity)
- [Rust Multiplatform / GitLab Example](https://github.com/rust-multiplatform/CI-Example-GitLab-CI)
- [Rust Multiplatform / BItBucket Example](https://github.com/rust-multiplatform/CI-Example-BitBucket-Pipelines)
- [Rust Multiplatform / Travis Example](https://github.com/rust-multiplatform/CI-Example-Travis)
- [Rust Multiplatform / Azure DevOps Example](https://github.com/rust-multiplatform/CI-Example-Azure-Pipelines)
- [Rust Multiplatform / AppVeyor Example](https://github.com/rust-multiplatform/CI-Example-AppVeyor)

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
