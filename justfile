set dotenv-load
set dotenv-filename := ".env.rust"

set windows-shell := ["pwsh", "-NoLogo", "-NoProfileLoadTime", "-Command"]


PATH_SEP := if os_family() == "windows" { ";" } else { ":" }

ENV_PATH := join(justfile_directory(), ".env.rust")
DEPS_PATH := join(justfile_directory(), ".deps")

BLENDER_PATH := join(DEPS_PATH, "blender")
GODOT_PATH := join(DEPS_PATH, "godot")
RUST_PATH := join(DEPS_PATH, "cargo", "bin")
EMSDK_PATH := join(DEPS_PATH, "emsdk")
EMSCRIPTEN_PATH := join(EMSDK_PATH, "upstream", "emscripten")

LOCAL_RUSTUP_PATH := join(DEPS_PATH, "rustup")
LOCAL_CARGO_PATH := join(DEPS_PATH, "cargo")

export PATH := env("PATH") + PATH_SEP + BLENDER_PATH + PATH_SEP + EMSDK_PATH + PATH_SEP + EMSCRIPTEN_PATH + PATH_SEP + GODOT_PATH + PATH_SEP + RUST_PATH

WEB_EXPORT_PATH:= join(justfile_directory(), "target", "web")

default: rust-debug rust-release

# launches the godot editor
[unix]
godot:
    cd godot && godot -e

# launches the godot editor
[windows]
godot:
    cd godot; Godot_v4.4-stable_win64.exe -e

# runs the provided argument as an executable with the correct environment variables, this should be used to launch your code editor.
@env $BIN_NAME:
    cd godot; $BIN_NAME

# calls `rust-debug` on changes to the rust library
@dev-debug:
    watchexec -r -w rust just rust-debug

# calls `rust-release` on changes to the rust library
@dev-release:
    watchexec -r -w rust just rust-release

# calls both `rust-debug` and `rust-release` on changes to the rust library
@dev-all:
    watchexec -r -w rust just rust-debug rust-release

# automatically calls release-web on changes to the project (rust or godot)
@dev-web dir=WEB_EXPORT_PATH:
    watchexec -r -w rust -w godot just release-web {{dir}}

# creates the debug version of the library for the current platform and wasm
rust-debug:
    cd rust; cargo +nightly build -Zbuild-std
    cd rust; cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten

# creates the release version of the library for the current platform and wasm
rust-release:
    cd rust; cargo +nightly build -Zbuild-std --release
    cd rust; cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten --release

# creates a web export at target/web or the provided path (relative to the ./godot directory)
[unix]
@release-web dir=WEB_EXPORT_PATH: rust-release rust-debug
    mkdir -p {{dir}}
    cd godot; godot --headless --export-release Web {{join(dir, "index.html")}}

# creates a web export at target/web or the provided path (relative to the ./godot directory)
[windows]
@release-web dir=WEB_EXPORT_PATH: rust-release rust-debug
    New-Item -ItemType Directory -Path {{dir}} -Force | Out-Null
    cd godot; Godot_v4.4-stable_win64.exe --headless --export-release Web {{join(dir, "index.html")}}



# installs all of the dependencies needed for the project.
[unix]
@setup: install-emscripten install-rust-toolchain install-blender install-godot
    echo -e "{{BOLD+GREEN}}Installed all dependencies successfully.{{NORMAL}}"


# Installs all of the dependencies needed for the project.
[windows]
@setup: install-emscripten install-rust-toolchain install-blender install-godot
    Write-Host "Installed all dependencies successfully." -ForegroundColor Green

# dont use: for usage with ci/cd to install export templates and rust with no user interaction
[unix]
@_setup-ci-cd: install-emscripten _install-rust-toolchain-noconfirm install-blender install-godot _install-godot-export-templates
    echo -e "{{BOLD+GREEN}}Installed all dependencies successfully.{{NORMAL}}"

# installs emscripten into .deps/emsdk
[unix]
@install-emscripten:
    echo -e "{{BOLD+YELLOW}}Installing emscripten...\033{{NORMAL}}"
    rm -rf "{{EMSDK_PATH}}"
    mkdir -p "{{DEPS_PATH}}"
    cd {{DEPS_PATH}} && git clone https://github.com/emscripten-core/emsdk.git
    cd {{EMSDK_PATH}} && ./emsdk install 3.1.74
    cd {{EMSDK_PATH}} && ./emsdk activate 3.1.74
    echo -e "{{BOLD+YELLOW}}Installed emscripten successfully.{{NORMAL}}"

# installs emscripten into .deps/emsdk
[windows]
@install-emscripten:
    Write-Host "Installing emscripten..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Path "{{DEPS_PATH}}" -Force | Out-Null
    git clone https://github.com/emscripten-core/emsdk.git "{{EMSDK_PATH}}"
    {{join(EMSDK_PATH, "emsdk.ps1")}} install 3.1.74
    {{join(EMSDK_PATH, "emsdk.ps1")}} activate 3.1.74
    Write-Host "Installed emscripten successfully." -ForegroundColor Yellow

# installs rust into .deps if rustup is not found
[windows]
@install-rust-toolchain:
    #!pwsh
    Write-Host "Installing rustup..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Path "{{DEPS_PATH}}" -Force | Out-Null
    if (Get-Command rustup -ErrorAction SilentlyContinue) {
        Write-Host "Rustup already installed, installing toolchains globally..." -ForegroundColor Yellow
        rustup toolchain install nightly
        rustup target add --toolchain nightly wasm32-unknown-emscripten
        rustup target add --toolchain nightly x86_64-pc-windows-msvc
        rustup component add --toolchain nightly rust-src
    } else {
        Write-Host "Rustup not installed, installing to {{DEPS_PATH}}" -ForegroundColor Yellow
        Set-Content "{{ENV_PATH}}" 'RUSTUP_HOME="{{replace(LOCAL_RUSTUP_PATH, "\\", "\\\\")}}"'
        Add-Content "{{ENV_PATH}}" 'CARGO_HOME="{{replace(LOCAL_CARGO_PATH, "\\", "\\\\")}}"'
        Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "{{join(DEPS_PATH, "rustup-init.exe")}}"
        $env:RUSTUP_HOME = "{{LOCAL_RUSTUP_PATH}}"
        $env:CARGO_HOME = "{{LOCAL_CARGO_PATH}}"
        {{join(DEPS_PATH, "rustup-init.exe")}} --default-toolchain nightly --no-modify-path
        rustup target add --toolchain nightly wasm32-unknown-emscripten
        rustup target add --toolchain nightly x86_64-pc-windows-msvc
        rustup component add --toolchain nightly rust-src
    }
    Write-Host "Rust toolchains installed successfully." -ForegroundColor Yellow

# installs rust into .deps if rustup is not found
[unix]
@install-rust-toolchain:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p "{{DEPS_PATH}}"
    if command -v rustup 2>&1 >/dev/null; then
        echo -e "{{BOLD+YELLOW}}Rustup already installed, installing globally...{{NORMAL}}"
        rustup toolchain install nightly
        rustup target add --toolchain nightly wasm32-unknown-emscripten
        rustup target add --toolchain nightly x86_64-unknown-linux-gnu
        rustup component add --toolchain nightly rust-src
    else
        echo -e "{{BOLD+YELLOW}}Rust toolchain not found, installing to {{DEPS_PATH}}{{NORMAL}}"
        echo 'RUSTUP_HOME="{{LOCAL_RUSTUP_PATH}}"' > {{ENV_PATH}}
        echo "CARGO_HOME={{LOCAL_CARGO_PATH}}" >> {{ENV_PATH}}
        export RUSTUP_HOME="{{LOCAL_RUSTUP_PATH}}"
        export CARGO_HOME="{{LOCAL_CARGO_PATH}}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly --no-modify-path
        rustup target add --toolchain nightly wasm32-unknown-emscripten
        rustup component add --toolchain nightly rust-src
    fi
    echo -e "{{BOLD+YELLOW}}Rust toolchain installed successfully.{{NORMAL}}"

# dont use: for use within ci/cd to install rust without user interaction.
[unix]
@_install-rust-toolchain-noconfirm:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p "{{DEPS_PATH}}"
    if command -v rustup 2>&1 >/dev/null; then
        echo -e "{{BOLD+YELLOW}}Rustup already installed, installing globally...{{NORMAL}}"
        rustup toolchain install nightly
        rustup target add --toolchain nightly wasm32-unknown-emscripten
        rustup target add --toolchain nightly x86_64-unknown-linux-gnu
        rustup component add --toolchain nightly rust-src
    else
        echo -e "{{BOLD+YELLOW}}Rust toolchain not found, installing to {{DEPS_PATH}}{{NORMAL}}"
        echo 'RUSTUP_HOME="{{LOCAL_RUSTUP_PATH}}"' > {{ENV_PATH}}
        echo "CARGO_HOME={{LOCAL_CARGO_PATH}}" >> {{ENV_PATH}}
        export RUSTUP_HOME="{{LOCAL_RUSTUP_PATH}}"
        export CARGO_HOME="{{LOCAL_CARGO_PATH}}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly --no-modify-path -y
        rustup target add --toolchain nightly wasm32-unknown-emscripten
        rustup component add --toolchain nightly rust-src
    fi
    echo -e "{{BOLD+YELLOW}}Rust toolchain installed successfully.{{NORMAL}}"

# installs watchexec using cargo (requires a rust installation - e.g. from "just setup"), required for dev-* recipes.
@install-watchexec:
    cargo install watchexec-cli

# installs blender to .deps
[linux]
@install-blender:
    echo -e "{{BOLD+YELLOW}}Downloading blender 4.3 to {{BLENDER_PATH}}...{{NORMAL}}"
    mkdir -p "{{BLENDER_PATH}}"
    curl --progress-bar -Lo '{{join(BLENDER_PATH, "blender-4.3.0-linux-x64.tar.xz")}}' https://download.blender.org/release/Blender4.3/blender-4.3.0-linux-x64.tar.xz
    tar xf '{{join(BLENDER_PATH, "blender-4.3.0-linux-x64.tar.xz")}}' --directory {{BLENDER_PATH}} --strip-components=1
    rm '{{join(BLENDER_PATH, "blender-4.3.0-linux-x64.tar.xz")}}'
    echo -e "{{BOLD+YELLOW}}Blender downloaded successfully.{{NORMAL}}"

# installs blender using homebrew
[macos]
@install-blender:
    brew install blender


# installs blender to .deps
[windows]
@install-blender:
    Write-Host "Downloading blender 4.3 to {{BLENDER_PATH}}" -ForegroundColor Yellow
    New-Item -ItemType Directory -Path "{{BLENDER_PATH}}" -Force | Out-Null
    Invoke-WebRequest -Uri "https://download.blender.org/release/Blender4.3/blender-4.3.0-windows-x64.zip" -OutFile '{{join(DEPS_PATH, "blender-4.3.0-windows-x64.zip")}}'
    Expand-Archive -Path '{{join(DEPS_PATH, "blender-4.3.0-windows-x64.zip")}}' -DestinationPath "{{BLENDER_PATH}}"
    Remove-Item '{{join(DEPS_PATH, "blender-4.3.0-windows-x64.zip")}}'
    Write-Host "Blender downloaded successfully" -ForegroundColor Yellow

# installs godot to .deps
[windows]
@install-godot:
    Write-Host "Downloading godot 4.4 to {{GODOT_PATH}}" -ForegroundColor Yellow
    New-Item -ItemType Directory -Path "{{GODOT_PATH}}" -Force | Out-Null
    Invoke-WebRequest -Uri "https://github.com/godotengine/godot/releases/download/4.4-stable/Godot_v4.4-stable_win64.exe.zip" -OutFile '{{join(DEPS_PATH, "Godot_v4.4-stable_win64.exe.zip")}}'
    Expand-Archive -Path '{{join(DEPS_PATH, "Godot_v4.4-stable_win64.exe.zip")}}' -DestinationPath "{{GODOT_PATH}}"
    Remove-Item '{{join(DEPS_PATH, "Godot_v4.4-stable_win64.exe.zip")}}'
    Write-Host "Godot 4.4 downloaded successfully" -ForegroundColor Yellow


# installs godot using homebrew
[macos]
@install-godot:
    brew install godot
    echo -e "{{BOLD+YELLOW}}Please ensure that godot is pinned to version v4.4 using homebrew.{{NORMAL}}"

# installs godot to .deps
[linux]
@install-godot:
    echo -e "{{BOLD+YELLOW}}Downloading godot 4.4 to {{GODOT_PATH}}...{{NORMAL}}"
    mkdir -p "{{GODOT_PATH}}"
    curl --progress-bar -Lo '{{join(DEPS_PATH, "Godot_v4.4-stable_linux.x86_64.zip")}}' https://github.com/godotengine/godot/releases/download/4.4-stable/Godot_v4.4-stable_linux.x86_64.zip
    unzip '{{join(DEPS_PATH, "Godot_v4.4-stable_linux.x86_64.zip")}}' -d "{{GODOT_PATH}}"
    mv '{{join(GODOT_PATH, "Godot_v4.4-stable_linux.x86_64")}}' '{{join(GODOT_PATH, "godot")}}'
    rm '{{join(DEPS_PATH, "Godot_v4.4-stable_linux.x86_64.zip")}}'
    echo -e "{{BOLD+YELLOW}}Godot downloaded successfully.{{NORMAL}}"

# dont use: for use in ci/cd, installs godot export templates to default location
[linux]
@_install-godot-export-templates:
    echo -e "{{BOLD+YELLOW}}Downloading godot 4.4 export templates to .local/share/godot"
    mkdir -p "$HOME/.local/share/godot/export_templates/4.4.stable/"
    curl --progress-bar -Lo '{{join(DEPS_PATH, "Godot_v4.4-stable_export_templates.tpz")}}' "https://github.com/godotengine/godot-builds/releases/download/4.4-stable/Godot_v4.4-stable_export_templates.tpz"
    unzip '{{join(DEPS_PATH, "Godot_v4.4-stable_export_templates.tpz")}}' -d '{{GODOT_PATH}}'
    mv '{{join(GODOT_PATH, "templates")}}'/* "$HOME/.local/share/godot/export_templates/4.4.stable/"
    rm '{{join(DEPS_PATH, "Godot_v4.4-stable_export_templates.tpz")}}'
    echo -e "{{BOLD+YELLOW}}Godot export templates downloaded successfully.{{NORMAL}}"
