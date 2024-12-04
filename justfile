set dotenv-load

default: dev

[unix]
toolchain:
    mkdir -p deps
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -t wasm32-unknown-emscripten --no-modify-path
    
[unix]
blender:
    mkdir -p deps
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -t wasm32-unknown-emscripten --no-modify-path
    

[windows]
toolchain:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -t wasm32-unknown-emscripten
    

setup:
    cd rust && rm -rf ./emsdk
    cd rust && git clone https://github.com/emscripten-core/emsdk.git
    cd rust/emsdk && ./emsdk install 3.1.66
    cd rust/emsdk && ./emsdk activate 3.1.66

dev:
    cd rust && cargo +nightly build -Zbuild-std
    cd rust && cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten

release:
    cd rust && cargo +nightly build -Zbuild-std --release
    cd rust && cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten --release

package: release
    cd godot && godot
