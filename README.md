## block-game-clone-backend

This project contains the core game logic for https://github.com/ethanSE/block_game_clone implemented in Rust.

+ Implements game logic as a function of GameState + Action -> GameState

+ Implements greedy AI opponent

+ Build automation for compiling to WebAssembly

+ Build automation for generating TypeScript types from Rust types for TS <-> Rust (as Wasm) interop in browser as strings

+ messing around with runnning on microcontroller
### Building Package (from project root)

```shell
cargo run -p build_automation  //from project root
```

### Publish wasm module to NPM

+ bump version number in block-game-clone-backend/Cargo.toml

+ build

+ wasm-pack publish

## License

* MIT license (http://opensource.org/licenses/MIT)
