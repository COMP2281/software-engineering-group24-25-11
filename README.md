# About

[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/vdJ2j5Ot)

# Installation

## Windows

Ensure you have `git` installed, this can downloaded [from their website](https://git-scm.com/downloads/win).

Next, we **highly** recommend installing `just`, a command runner to make it easier to build the project, this can be done with:
`winget install -e --id Casey.Just`
If you don't, it is up to you to source the dependencies, setup the environment variables, etc.

Finally, open a terminal inside the root project directory (i.e. the one which contains the folders mentioned in the structure).
This typically can be done through opening the file explorer, clicking on the address bar, and typing `wt` (windows terminal) or `cmd`

Now we need to install the rest of the dependencies, this can be done by running `just setup`, after which you should see a message indicating if everything was successful.

Now you can proceed to [Getting started](#getting-started) to learn how to start development.



## Linux
Please ensure that you have up to date packages, otherwise things may not work as expected.

Ensure you have the following packages installed (should be available in your package manager):
- `git`
- `just` (needs to be somewhat recent - if needed download from github releases and place at `~/.local/bin/just`)

Then we need to install the rest of the dependencies, this can be done by running `just setup`, after which you should see a message indicating if everything was successful.

Now you can proceed to [Getting started](#getting-started) to learn how to start development.


## MacOS (todo)
🚧 This is currently unsupported, please ask for help.

## Manual installation (any platform)

While we recommend following the guide above, the following programs are required to build the project.

- emsdk + emscripten v3.1.74 (required for web)
- blender (required)
- rust nightly toolchain + rust-src component (required):
  - wasm32-unknown-emscripten (web)
  - x86_64-pc-windows-msvc (if on windows)
  - x86_64-unknown-linux-gnu (if on linux)
- just (optional - highly recommended - makes it easy to setup, develop, and release)

You should ensure that these dependencies are in your PATH.

Then you can look at the `justfile` to find commands you may want to run.

# Development
## Project structure
├── godot: The base godot game project.\
├── rust: All rust code for the GDExtension for the Godot project, provides custom classes.\
├── docs: All required documentation, such as requirements etc...\
├── .deps: A local collection of dependencies needed for building/developing.
└── target (optional): Final files/executables for all build types...\\

## Getting started
To launch godot with the correct environment run: `just godot`
To launch an editor of your choice (e.g. vscode) with the correct environment run: `just env "code ."` (note: you need to pass the command as a string)

To compile rust extension (in debug mode) do `just` or alternatively (more verbose) `just rust-debug`.
To compile rust extension (in release mode) do `just rust-release`.

If you wish to automatically compile the rust extension when you make changes, see [automatic rebuilding](#automatic-rebuilding)


> [!NOTE]
> After launching godot for the first time, you will be prompted to find a blender executable, select the one in `${PROJECT_ROOT}/.deps/blender/`.
> You will also need to download the export templates, which you can do by clicking `Project -> Export... -> Web (runnable)`, after which you will see a red label (if you don't have the export templates installed), follow the instructions there.

## Automatic rebuilding
To automatically rebuild the rust extension whenever you make changes, you can start a terminal and run `just dev`, then it will rebuild the extension (in debug mode) automatically.

> [!NOTE]
> This requires `watchexec` to be installed, which can be done through `just install-watchexec`
