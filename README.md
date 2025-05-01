My master's thesis is on adapting Rust Analyzer to work with Bluespec SystemVerilog. This is the artifact. It has some bugs and doesn't support many features, but it's mostly error-resilient and is full-featured enough for classroom use.

Because it's still in-progress, I haven't yet packaged this up nicely. To install from source, clone this repo and run `cargo xtask install`, installing whatever dependencies you need. Then to use, make sure you have VS Code installed, and open any workspace with `.bsv` files. The project discovery works by searhing for `.bsv` files within a limited number of directories (but more than you probably need) downstream of your workspace, and dumping them all into a pool. This approach precludes some project layouts, but works for the common case.

Feel free to reach out with any issues to martinch@mit.edu. I'll publish more details when I'm closer to finishing, since I'm actively writing my thesis.

---

<p align="center">
  <img
    src="https://raw.githubusercontent.com/rust-analyzer/rust-analyzer/master/assets/logo-wide.svg"
    alt="rust-analyzer logo">
</p>

rust-analyzer is a modular compiler frontend for the Rust language.
It is a part of a larger rls-2.0 effort to create excellent IDE support for Rust.

## Quick Start

https://rust-analyzer.github.io/manual.html#installation

## Documentation

If you want to **contribute** to rust-analyzer check out the [CONTRIBUTING.md](./CONTRIBUTING.md) or
if you are just curious about how things work under the hood, check the [./docs/dev](./docs/dev)
folder.

If you want to **use** rust-analyzer's language server with your editor of
choice, check [the manual](https://rust-analyzer.github.io/manual.html) folder.
It also contains some tips & tricks to help you be more productive when using rust-analyzer.

## Security and Privacy

See the corresponding sections of [the manual](https://rust-analyzer.github.io/manual.html#security).

## Communication

For usage and troubleshooting requests, please use "IDEs and Editors" category of the Rust forum:

https://users.rust-lang.org/c/ide/14

For questions about development and implementation, join rust-analyzer working group on Zulip:

https://rust-lang.zulipchat.com/#narrow/stream/185405-t-compiler.2Frust-analyzer

## Quick Links

* Website: https://rust-analyzer.github.io/
* Metrics: https://rust-analyzer.github.io/metrics/
* API docs: https://rust-lang.github.io/rust-analyzer/ide/
* Changelog: https://rust-analyzer.github.io/thisweek

## License

rust-analyzer is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
