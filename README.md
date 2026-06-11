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