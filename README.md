# Mindustry Mods

Checkout the website here: https://simonwoodburyforget.github.io/mindustry-mods/

Add mods to listing here: https://github.com/SimonWoodburyForget/mindustry-mods/blob/master/CONTRIBUTING.md

## Development

Requirements:
- Python 3.8: 
  - [`requirements.txt`](requirements.txt)
- Rust 1.42.0:
  - [`wasm-pack`](wasm-pack)
  - [`cargo-make`](cargo-make)
- npm:
  - gh-pages 2.2.0
- Github personal access token at `~/.github-token` to increase request limit 
  from 500 to 5,000.

Testing: 
- `cargo test`
- `cargo make dist-test` (pushes to test repo in another directory, to test the gh-pages output)

Building:
- `cargo make dist-release` (pushes to main repo with gh-pages)

Executing: 
- `cargo run -- -iph`

Notes: `cargo make dist-*` *related commands require repo access,
relative paths and urls are hardcoded to make that work.*

[requirements]: https://github.com/SimonWoodburyForget/mindustry-mods/blob/master/scripts/requirements.txt
[wasm-pack]: https://github.com/rustwasm/wasm-pack
[cargo-make]: https://github.com/sagiegurari/cargo-make
[rustup]: https://rustup.rs/
