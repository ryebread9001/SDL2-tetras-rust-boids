# SDL2 Tetras Boids

A small Rust arcade game built with Tetra and SDL2. The project uses sprite assets in the `resources/` folder, so it should be launched from the repository root.

## Requirements

- Rust toolchain
- A C/C++ build toolchain for native dependencies
- SDL2 development libraries for your platform

## Install SDL2 development dependencies

### Windows

1. Install the Rust MSVC toolchain:

```powershell
rustup default stable-x86_64-pc-windows-msvc
```

2. Install Visual Studio Build Tools with the workload for "Desktop development with C++".

3. Install SDL2 development libraries. The easiest common option is via vcpkg:

```powershell
git clone https://github.com/microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg install sdl2
```

4. Make sure the SDL2 binaries are on your `PATH`, or set `VCPKG_ROOT` appropriately for your shell.

### Linux (easier)

```bash
sudo apt update
sudo apt install build-essential pkg-config libsdl2-dev libasound2-dev libudev-dev
```

```bash
cargo run
```
