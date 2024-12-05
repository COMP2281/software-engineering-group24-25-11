# About

[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/vdJ2j5Ot)

# Project structure

├── godot: The base godot game project.\
├── rust: All rust code for the GDExtension for the Godot project, provides custom classes.\
├── docs: All required documentation, such as requirements etc...\
├── deps: A local collection of dependencies needed for building/developing.
└── target (optional): Final files/executables for all build types...\\

# Getting started

## Windows

Ensure you have `git` installed, this can either be gotten [from their website](https://git-scm.com/downloads/win), or downloaded using winget:
`winget install -e --id Git.Git --source winget`

Next, ensure you have `godot` installed, this can either be gotten [from their website](https://godotengine.org/), or downloaded using winget:
`winget install -e --id=GodotEngine.GodotEngine -v "4.3"`

Next, we **highly** recommend installing `just`, a command runner to make it easier to build the project, this can be done with:
`winget install -e --id Casey.Just`

Finally, open **git bash** inside the root project directory (i.e. the one which contains the folders mentioned in the structure).
This typically can be done through opening the file explorer, right clicking -> more options -> git bash here

Now we need to install the rest of the dependencies, this can be done by running `just setup`, after which you should see a message indicating if everything was successful.

> [!IMPORTANT]
> You should launch godot through the command `just godot`, this is because to compile for the web emscripten is required, and hence needs to be in the path.
> You may get around this by manually adding the following paths to your system PATH environment variable:
> `{PROJECT_DIR}/deps/emsdk`
> `{PROJECT_DIR}/deps/emsdk/upstream/emscripten`

## Linux

Ensure you have the following packages installed (should be available in your package manager):
- `git`
- `just`

Then we need to install the rest of the dependencies, this can be done by running `just setup`, after which you should see a message indicating if everything was successful.

Then to run godot, use `just godot`.

To compile rust changes (in debug mode) do `just dev`.
To compile rust changes (in release mode) do `just rust-release`.

## MacOS (todo)
This is currently
This is currently todo since I don't have a Mac

## Manual installation (any platform)

While we recommend following the guide above, the following programs are required to build the project.

- emsdk + emscripten v3.1.66 (required for web)
- blender (required)
- rust nightly toolchain + rust-src component (required):
  - wasm32-unknown-emscripten (web)
  - x86_64-pc-windows-msvc (windows)
  - x86_64-unknown-linux-gnu (linux)
- just (optional - highly recommended - makes it easy to setup, develop, and release)

You should ensure that these dependencies are in your PATH.

Then you can look at the top of the `justfile` to find commands you may want to run.
