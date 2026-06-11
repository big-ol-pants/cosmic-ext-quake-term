# COSMIC Quake Term

COSMIC Quake Term is an experimental quake-style terminal for the COSMIC desktop environment.

This project is based on [`cosmic-term`](https://github.com/pop-os/cosmic-term) and aims to add quake-like terminal behavior, such as quickly showing and hiding a terminal from the desktop.

> Status: early development

## About

COSMIC Quake Term builds on COSMIC Term, the COSMIC terminal emulator. COSMIC Term uses [`alacritty_terminal`](https://docs.rs/alacritty_terminal), provided by the Alacritty project, and uses a custom renderer based on [`cosmic-text`](https://github.com/pop-os/cosmic-text).

The goal of this fork is to keep the COSMIC-native terminal experience while adding quake-style workflow features.

## Planned Quake-Style Features

The project is intended to support features such as:

- Toggleable drop-down terminal behavior
- Keyboard-shortcut-driven show/hide workflow
- Top-of-screen terminal placement
- Configurable terminal height
- Persistent terminal session
- COSMIC-friendly integration
- Possible panel/applet integration in the future

Some of these features may still be experimental or incomplete.

## Current Project State

This repository is currently still very close to upstream COSMIC Term. Some package metadata still uses the upstream names:

- Cargo package name: `cosmic-term`
- App ID: `com.system76.CosmicTerm`
- Default binary name: `cosmic-term`

These names may change later as the project becomes more distinct from upstream COSMIC Term.

## Requirements

### System

This project is intended for Linux systems running the COSMIC desktop environment.

### Build Tools

You will need:

- Rust
- Cargo
- Git
- Common Linux build tools
- Development libraries required by COSMIC/libcosmic applications

On Pop!_OS or Ubuntu-based systems, you can start with:

```bash
sudo apt update
sudo apt install -y build-essential git pkg-config libxkbcommon-dev
```

Additional packages may be required depending on your distro and graphics stack.

## Install Rust

If Rust is not already installed, install it with rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then reload your shell environment:

```bash
source "$HOME/.cargo/env"
```

Verify the installation:

```bash
rustc --version
cargo --version
```

This project currently declares Rust `1.90` in `Cargo.toml`.

## Clone the Repository

```bash
git clone https://github.com/big-ol-pants/cosmic-quake-term.git
cd cosmic-quake-term
```

## Build

For a debug build:

```bash
cargo build
```

For a release build:

```bash
cargo build --release
```

You can also use the provided `justfile` if you have [`just`](https://github.com/casey/just) installed:

```bash
just build-debug
```

or:

```bash
just build-release
```

## Run

During development, run with Cargo:

```bash
cargo run
```

Or use the `justfile` run recipe:

```bash
just run
```

Because the package is currently still named `cosmic-term`, the release binary is expected at:

```bash
target/release/cosmic-term
```

Run it directly with:

```bash
./target/release/cosmic-term
```

## Install Locally

Build the release binary first:

```bash
cargo build --release
```

Then install using the `justfile`:

```bash
sudo just install
```

By default, the `justfile` installs under:

```text
/usr
```

and uses the current app ID:

```text
com.system76.CosmicTerm
```

To install under a different root or prefix, review the variables at the top of the `justfile`.

## Uninstall

If installed through the `justfile`, uninstall with:

```bash
sudo just uninstall
```

## Features

The default Cargo features are:

* `dbus-config`
* `wgpu`
* `wayland`
* `password_manager`

The `wgpu` feature enables GPU rendering. If GPU rendering is unavailable or disabled, COSMIC Term can fall back to software rendering.

To build without default features:

```bash
cargo build --no-default-features
```

To build with specific features:

```bash
cargo build --no-default-features --features wayland,wgpu
```

## Color Schemes

Custom color schemes can be imported from the application menu:

```text
View -> Color schemes...
```

Templates are available in the [`color-schemes`](color-schemes) directory.

## Development

Format the code:

```bash
cargo fmt
```

Run Clippy:

```bash
cargo clippy --all-features -- -W clippy::pedantic
```

Or use the `justfile`:

```bash
just check
```

Clean build artifacts:

```bash
cargo clean
```

or:

```bash
just clean
```

## Notes for Contributors

This project is currently transitioning from upstream COSMIC Term toward a quake-style terminal experience.

Useful areas to work on include:

* Quake-style show/hide behavior
* Window positioning
* Shortcut or IPC-based toggle behavior
* COSMIC panel/applet integration
* Package/app ID renaming
* Documentation
* Testing on COSMIC Wayland sessions

## Known Naming TODOs

The project currently still contains upstream COSMIC Term names in several places. These should eventually be reviewed and renamed if the project is intended to be distributed separately.

Likely places to update:

* `Cargo.toml`
* `justfile`
* `.desktop` metadata
* App ID
* Icons
* Metainfo/AppStream files
* Translation strings
* README references

## License

This project is licensed under GPL-3.0-only.

See [`LICENSE`](LICENSE) for details.

## Acknowledgements

This project is based on COSMIC Term by System76 and the COSMIC desktop project.

It uses terminal emulation technology from Alacritty and text rendering technology from COSMIC Text.
