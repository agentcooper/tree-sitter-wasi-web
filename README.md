Example setup for compiling Rust code that depends on an unmodified [tree-sitter](https://crates.io/crates/tree-sitter) crate to the web via WebAssembly (and WASI).

https://agentcooper.github.io/tree-sitter-wasi-web/

# Install

Install [WASI SDK](https://github.com/WebAssembly/wasi-sdk) to `/opt/wasi-sdk`.

# Run (web)

Run `make web-start` to start a local dev server.

Or create `web/dist` directory ready for use with any static web server:

```bash
make web-build
python3 -m http.server -d web/dist
```

# Run (local)

```bash
cargo run
```
