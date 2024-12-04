set dotenv-load

default: dev

setup:
    cd rust && rm -rf ./emsdk
    gcd rust && it clone https://github.com/emscripten-core/emsdk.git
    cd rust/emsdk && ./emsdk install 3.1.66
    cd rust/emsdk && ./emsdk activate 3.1.66

dev:
    cd rust && cargo +nightly build -Zbuild-std
    cd rust && cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten
    # cd godot && godot

