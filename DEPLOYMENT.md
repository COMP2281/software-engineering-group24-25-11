# Building
## Installing dependencies
The following dependencies are required:
- git
- clang >= v9.0
- just >= v1.38 (optional - highly recommended - makes it easy to setup, develop, and release)
The following are also required, but are provided by the setup command.
- blender (required)
- emsdk + emscripten v3.1.74 (required for web)
- rust nightly toolchain + rust-src component (required):
  - wasm32-unknown-emscripten (web)
  - x86_64-pc-windows-msvc (if on windows) (requires msvc - requires administrator - will be prompted by rustup to install)
  - x86_64-unknown-linux-gnu (if on linux)

### Windows
Git can be installed [from their website](https://git-scm.com/downloads/win).\
Just can be installed with `winget install -e --id Casey.Just`\
Clang can be installed [following these instructions](https://rust-lang.github.io/rust-bindgen/requirements.html)
You must also have the new powershell installed which can be done with `winget install --id Microsoft.PowerShell --source winget`.

### Other platforms
For linux use your package manager (ensure `just` is a compatible version), you can also add binaries to `.local/bin` if the required versions are not in your package manager.
For macos, we recommend install your dependencies using homebrew.

> [!NOTE]
> `just` is not required to build the project, however it handles setting environment variables and adding dependencies to PATH. If you wish you may install the dependencies manually and run the build commands (which can be found in the `justfile`) manually.

Since the primary focus of this project is the web, most of the `just` recipes assume that you are also building for WASM, and hence still requires `emscripten` and the `wasm32` rust toolchain and rust-src component.


You can see a full list of `just` recipes with `just --list`

We provide an easy way to install these dependencies within the project folder to simplify development of the game.
This will install all dependencies into .deps, with the only exception being rust, which will only be installed to `.deps` if there is no system wide installation (if you wish to prevent this, remove `rustup` from PATH when running the setup command).
This can be done through the following instructions.
- Ensure you have `clang >= 9.0` installed [(see this for instructions)](https://rust-lang.github.io/rust-bindgen/requirements.html)
- If running on windows, the installation of rust will require that you have `msvc` installed (requires administrator privilleges), if you do not, instructions will be provided during the setup command.
- If on a unix platform (e.g. linux & macos), ensure you have both `tar` and `unzip` installed.
- Finally run `just setup` and follow any instructions.

Please take a note of the following:

> [!NOTE]
> After launching godot for the first time, you will be prompted to find a blender executable, select the one in `${PROJECT_ROOT}/.deps/blender/` if using `just setup` otherwise select your own blender executable.

> [!NOTE]
> Building the project also **requires** the godot export templates to be downloaded, this can be done by clicking `Project -> Export... -> Web (runnable)`, after which you will see a red label (if you don't have the export templates installed), follow the instructions there to `Manage export templates -> Download and install`.


> [!NOTE]
> Many common actions have `dev-*` alternatives which will rebuild automatically when changes happen to the project whilst the commands are running, these require `watchexec` to be installed, which you can do manually or through cargo using `just install-watchexec`.
> These commands include:
> - just dev-all # builds rust library in debug and release (for web and current platform)
> - just dev-debug # also dev-release: builds rust library in specified mode (for web and current platform)
> - just dev-web $DIR # creates a web export at the given path or at target/web.

## Launching godot and your code editor
After the dependencies have been installed you **must** launch godot through `just godot` if you have installed the dependencies through `just setup`.

You may also open your code editor with the right environment (e.g. rust installation), through `just env "editor command"`.
For example with vscode this would be: `just env "code ."`
For example with neovim this would be: `just env "nvim ."`

## Running outside the browser
In order to ease development of trivial features, the game can be ran as an executable through godot. This is missing a VR implementation since it requires OpenXR instead of WebXR, however it is an easy way to do development without the complexity of the web.

To run it locally, the rust library needs to be compiled, which can be done with `just rust-debug rust-release`, or alternatively if `watchexec` is installed (which can be done with `just install-watchexec`), automatically re-compiled upon changes through `just dev-all`.

Once the rust library is compiled, you can simply press the play button in the top toolbar, which will open the game on your device. You may also export an executable through godot in `Project -> Export...`, though this remains mostly untested.

## Creating a web export
> [!NOTE]
> A web export is only required if you wish to host the project without godot, in a production environment, or wish to use the project from outside of the local device (i.e. not on localhost), this is typically required for VR devices.
> See [Godot remote deploy](#godot-remote-deploy) on how run an easy development server on localhost.

Creating a web export **requires** both the debug and release builds of the rust library to be made. This can be done with the following:
- `just rust-debug rust-release`

Following this, provided that the godot export templates have been installed, you can open godot and go to `Project -> Export -> Web` and then click `Export Project...`, unselecting `Export with debug` if checked. You can then customise where the export will be pushed to by changing the directory where the resulting `.html` file will be created.

To make this less cumbersome, you can use the following command to export the project:
- `just release-web ${DIRECTORY}`
The directory variable is optional, and if not provided, the export will be placed in a directory in the root of the project called target, in a subfolder called web.
Any relative path provided in place of `${DIRECTORY}` will start at the root of the godot directory: `./godot`.

Provided that `watchexec` has been installed (which can be done with `just install-watchexec`), you can also re-export on change using `just dev-web ${DIRECTORY}`.

Hosting the web export is not as simple as simply deploying the website. Instead it requires both HTTPS (a certificate), and headers to be set. See [Hosting the web export](#hosting-the-web-export) for more information.


# Deploying to the web
## Godot remote deploy
The simplest way to deploy the game to the web is through godot's builtin `Remote Deploy` function, this creates a local server on localhost, bypassing the need for a SSL certificate (since most browsers consider localhost a secure environment). You should read [Notes about deployment on localhost](#notes-about-deployment-on-localhost) if you wish to access the server from a different device (such as a VR headset).

Ensure that initially (and after any changes), the rust code is recompiled with `just rust-debug rust-release`, or if `watchexec` is installed (which can be done with `just install-watchexec`), using `just dev-all` to automatically re-compile.

To start the remote deployment, ensure you have the export templates installed, after which in the top toolbar, next to the play/pause/stop buttons, you should see an icon which looks like a monitor with a play icon. Clicking this and selecting `Run in browser`, starts godot's web server. After changes, make sure any rust code is recompiled, and then click the button again and select `Re-export Project`.

### Notes about deployment on localhost
This should be sufficient for testing in 3D, as for VR, there are further considerations since the web export requires a secure environment (which is provided through localhost), whereas on the target VR device, the export will likely not be localhost (in most network configurations). Therefore for testing on VR devices consult below.

## Hosting the web export
To host the web export on your own, there are several considerations, such as as mentioned previously in this document, outside of localhost of the machine hosting the webserver, a SSL certificate is required for HTTPS (therefore a domain - or your own certificate authority is required).

This document assumes that the export will be hosted on a Linux machine, though for other systems, a similar approach can be used.

There are a multitude of tutorials online for getting a SSL certificate for a domain (which is the focus of this section), if you wish to use your own certificate authority you are left to work that out on your own.

A simple way is through `certbot`, first create an A record on your domain pointing to the external IP of the hosting device, or the local IP if needed. Then run `certbot certonly --manual --preferred-challenges dns -d your.domain.com`. You may omit the preferred challenge if the record doesn't point to a local IP. Follow the instructions provided.

A sample NGINX configuration is provided here:
```nginx
server {
    listen       443 ssl;
    server_name  your.domain.com;

    # This should be the path to your SSL certificates for this server_name,
    # this is typically located at the provided directories.
    ssl_certificate      /etc/letsencrypt/live/your.domain.com/fullchain.pem;
    ssl_certificate_key  /etc/letsencrypt/live/your.domain.com/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;

    root   /var/www; # This should be the folder your web export is placed at.
    index index.html; # This should match the export html name.

    location / {
        add_header Cross-Origin-Opener-Policy same-origin;
        add_header Cross-Origin-Embedder-Policy require-corp;
        add_header Cross-Origin-Resource-Policy cross-origin;
        add_header Permissions-Policy "cross-origin-isolated=(self)";
        add_header X-Frame-Options DENY;
        add_header X-Content-Type-Options nosniff;
        # Enable HSTS (recommended)
        add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    }
}
```
> [!NOTE]
> It is important to ensure that the exported files are served with the provided headers, otherwise the game will fail to load.


# Github Action
The current github action creates a new release when the repository is tagged, this release contains a zip of a complete web export.

The action also attempts to deploy the web export to a **Netlify** site, which requires the following secrets to be set for the repository:
- `NETLIFY_SITE_ID`: The netlify site ID, found by going to a created site, and viewing the `Site Configuration` in the `Site Details`.
- `NETLIFY_AUTH_TOKEN`: A netlify authentication token. A personal access token which is sufficient can be created from `User settings` -> `Applications`.
