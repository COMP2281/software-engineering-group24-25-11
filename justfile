set dotenv-load
set dotenv-filename := ".env.rust"

PWD := shell("echo $PWD")
export PATH := shell("echo $PATH") + ":" + PWD + "/deps:" + PWD + "/deps/emsdk:" + PWD + "/deps/emsdk/upstream/emscripten"

default: rust-debug

godot:
    cd godot && godot -e & disown

dev: rust-debug

rust-debug:
    cd rust && cargo +nightly build -Zbuild-std
    cd rust && cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten

rust-release:
    cd rust && cargo +nightly build -Zbuild-std --release
    cd rust && cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten --release

release: release-web

# Creates all of the files needed for web
release-web: rust-release
    mkdir -p target/web
    cd godot && godot --export-release Web ../target/web/index.html

# TODO: add export presets for other platforms.


# Installs all of the dependencies needed for the project.
@setup: install-emscripten install-rust-toolchain install-blender install-godot
    echo -e "{{BOLD+GREEN}}Installed all dependencies successfully.{{NORMAL}}"

# Installs emscripten into ./deps/emsdk
[unix]
@install-emscripten:
    echo -e "{{BOLD+YELLOW}}Installing emscripten...\033{{NORMAL}}"
    rm -rf ./deps/emsdk
    mkdir -p ./deps
    cd ./deps && git clone https://github.com/emscripten-core/emsdk.git
    cd ./deps/emsdk && ./emsdk install 3.1.66
    cd ./deps/emsdk && ./emsdk activate 3.1.66
    echo -e "{{BOLD+YELLOW}}Installed emscripten successfully.{{NORMAL}}"

[windows]
@install-emscripten:
    echo -e "{{BOLD+YELLOW}}Installing emscripten...\033{{NORMAL}}"
    rm -rf ./deps/emsdk
    mkdir -p ./deps
    cd ./deps && git clone https://github.com/emscripten-core/emsdk.git
    cd ./deps/emsdk && ./emsdk.bat install 3.1.66
    cd ./deps/emsdk && ./emsdk.bat activate 3.1.66
    echo -e "{{BOLD+YELLOW}}Installed emscripten successfully.{{NORMAL}}"

[windows]
install-rust-toolchain:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p ./deps
    if command -v rustup 2>&1 >/dev/null; then
        echo -e "{{BOLD+YELLOW}}Rustup already installed, installing globally...\033{{NORMAL}}"
        rm -f .env.rust
        rustup toolchain install nightly
        rustup target add --toolchain nightly wasm32-unknown-emscripten
    else
        echo -e "{{BOLD+YELLOW}}Rust toolchain not found, installing to ./deps{{NORMAL}}"
        RUSTENV="PATH=\$PWD/deps/cargo/bin:\$PATH
    RUSTUP_HOME=\$PWD/deps/rustup
    CARGO_HOME=\$PWD/deps/cargo"
        tee .env.rust > /dev/null <<< $RUSTENV && source .env.rust
        curl -o ./deps/rustup-init.exe https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe
        ./deps/rustup-init.exe --default-toolchain nightly -t wasm32-unknown-emscripten --no-modify-path
        rustup component add --toolchain nightly rust-src
    fi
    echo -e "{{BOLD+YELLOW}}Rust toolchain installed successfully{{NORMAL}}"

[unix]
install-rust-toolchain:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p ./deps
    if command -v rustup 2>&1 >/dev/null; then
        echo -e "{{BOLD+YELLOW}}Rustup already installed, installing globally...\033{{NORMAL}}"
        rm -f .env.rust
        rustup toolchain install nightly
        rustup target add --toolchain nightly wasm32-unknown-emscripten
    else
        echo -e "{{BOLD+YELLOW}}Rust toolchain not found, installing to ./deps{{NORMAL}}"
        RUSTENV="PATH=\$PWD/deps/cargo/bin:\$PATH
    RUSTUP_HOME=\$PWD/deps/rustup
    CARGO_HOME=\$PWD/deps/cargo"
        tee .env.rust > /dev/null <<< $RUSTENV && source .env.rust
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -t wasm32-unknown-emscripten --no-modify-path
        rustup component add --toolchain nightly rust-src
    fi
    echo -e "{{BOLD+YELLOW}}Rust toolchain installed successfully.{{NORMAL}}"

[linux]
@install-blender:
    echo -e "{{BOLD+YELLOW}}Downloading blender 4.3 to ./deps/blender...{{NORMAL}}"
    mkdir -p ./deps/blender
    curl --progress-bar -Lo deps/blender-4.3.0-linux-x64.tar.xz https://download.blender.org/release/Blender4.3/blender-4.3.0-linux-x64.tar.xz
    tar xf ./deps/blender-4.3.0-linux-x64.tar.xz --directory ./deps/blender --strip-components=1
    rm ./deps/blender-4.3.0-linux-x64.tar.xz
    echo -e "{{BOLD+YELLOW}}Blender downloaded successfully.{{NORMAL}}"


[windows]
@install-blender:
    echo -e "{{BOLD+YELLOW}}Downloading blender 4.3 to ./deps/blender...{{NORMAL}}"
    mkdir -p ./deps
    rm -rf ./deps/blender
    curl --progress-bar -Lo deps/blender-4.3.0-windows-x64.zip https://download.blender.org/release/Blender4.3/blender-4.3.0-windows-x64.zip
    unzip ./deps/blender-4.3.0-windows-x64.zip -d ./deps
    mv ./deps/blender-4.3.0-windows-x64/ ./deps/blender
    rm ./deps/blender-4.3.0-windows-x64.zip
    echo -e "{{BOLD+YELLOW}}Blender downloaded successfully.{{NORMAL}}"

[windows]
@install-godot:
    echo -e "{{BOLD+YELLOW}}Downloading godot 4.3 to ./deps/godot...{{NORMAL}}"
    mkdir -p ./deps
    rm -rf ./deps/godot
    curl --progress-bar -Lo deps/Godot_v4.3-stable_win64.exe.zip https://github.com/godotengine/godot/releases/download/4.3-stable/Godot_v4.3-stable_win64.exe.zip
    unzip ./deps/Godot_v4.3-stable_win64.exe.zip -d ./deps
    mv ./deps/Godot_v4.3-stable_win64.exe ./deps/godot
    rm ./deps/Godot_v4.3-stable_win64_console.exe
    rm ./deps/Godot_v4.3-stable_win64.exe.zip
    echo -e "{{BOLD+YELLOW}}Godot downloaded successfully.{{NORMAL}}"
[unix]
@install-godot:
    echo -e "\n\n{{BOLD+BLUE}}Please install godot yourselves, and ensure that it is accessible on PATH (i.e. running \`godot --version\` works).{{NORMAL}}\n\n"
