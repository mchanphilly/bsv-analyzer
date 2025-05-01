# bsv-analyzer

My master's thesis is on adapting Rust Analyzer to work with Bluespec SystemVerilog. This is the artifact. It has some bugs and doesn't support many features, but it's mostly error-resilient and is full-featured enough for classroom use.

Because it's still in-progress, I haven't yet packaged this up nicely as a VS Code extension.

To install from source, clone this repo and run `cargo xtask install`, installing whatever dependencies you need. Then to use, make sure you have VS Code[^editors] installed, and open any workspace with `.bsv` files. The project discovery works by searhing for `.bsv` files within a limited number of layers (but more than you probably need) downstream of your workspace, and dumping them all into a pool. This approach precludes some project layouts, but works for the common case.

Feel free to reach out with any issues to martinch@mit.edu. I'll publish more details when I'm closer to finishing, since I'm actively writing my thesis.

[^editors]: You can use other editors, but I'm not super familiar with the process. See [Rust Analyzer's documentation](https://rust-analyzer.github.io/book/other_editors.html) for details.

---

rust-analyzer is a modular compiler frontend for the Rust language.
It is a part of a larger rls-2.0 effort to create excellent IDE support for Rust.

## License

bsv-analyzer (like rust-analyzer) is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
