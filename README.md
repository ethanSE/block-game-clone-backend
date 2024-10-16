## block-game-clone-backend

This project contains the core game logic for https://github.com/ethanSE/block_game_clone implemented in Rust.

+ Implements game logic as a function of GameState + Action -> GameState

+ Implements greedy AI opponent

+ Build automation for compiling to WebAssembly

+ Build automation for generating TypeScript types from Rust types for TS <-> Rust (as Wasm) interop in browser as strings

### Building Package (from project root)
with Rust, Cargo, wasm-pack installed:

```shell
cargo run -p build_wasm_ts_module  //from project root
```

### Publish wasm module to NPM

+ bump version number in block-game-clone-backend/Cargo.toml

+ build

+ wasm-pack publish

## License

* MIT license (http://opensource.org/licenses/MIT)




# Architecture

```mermaid
       C4Component
    title Component diagram for block-game-clone-backend

    Component(webApp, "Web App", "React", "")

    Container_Boundary(backend, "Backend") {
        Component(wasmBundle, "WASM module","WASM + TS types", "Distributed as npm package")
        Component(buildAutomation, "Build Automation", "Rust", "Uses wasm-pack to build wasm module, ts-rs to generate typescript types")
        Component(gameLogic, "Core Game Logic", "Rust", "represents game state. A functional reducer from state, action pair -> new state")

        Rel(buildAutomation, wasmBundle, "builds")
        Rel(buildAutomation, gameLogic, "uses")
    }

    Rel(webApp, wasmBundle, "uses")   


    UpdateLayoutConfig($c4ShapeInRow="2", $c4BoundaryInRow="1")
    UpdateRelStyle(webApp, wasmBundle, $offsetX="20", $offsetY="30")
    UpdateRelStyle(buildAutomation, gameLogic, $offsetX="20", $offsetY="10")
    UpdateRelStyle(buildAutomation, wasmBundle, $offsetX="-10", $offsetY="20")

```


The goal here was to create a single single repository that contains the game logic. The idea is that this logic could be used anywhere including:
- in the current web app as compiled to web assembly
- on a server for a multiplayer mode
- in any other project (training a better AI opponent?)
