# vst_plugin

This is an example VST plugin written in Rust, using the
[vst-rs](https://github.com/RustAudio/vst-rs) crate. It is adapted from
`vst-rs`'s example gain plugin.

To build a `.dll` file for Windows, first install rust for Windows using
[RUSTUP-INIT.EXE](https://www.rust-lang.org/tools/install). Then simply run:

```
cargo build
```

Copy the `vst_plugin.dll` file from the `target/debug/` directory into your
64-bit VST2 directory configured in your DAW.

Tip: You can develop on Linux or WSL and only use Windows to compile the
`.dll`.