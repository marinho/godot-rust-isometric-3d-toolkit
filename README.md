## Development

### Watch and Build

```shell
cargo watch -x "build"
```

### Watch and Test

```shell
cargo watch -x "test"
```

## Using in a Godot 4.x project

1. Copy `isometric_3d_toolkit.gdextension.template` as just `isometric_3d_toolkit.gdextention` (or another name as you prefer) into the Godot project root folder.
2. Update file paths in `isometric_3d_toolkit.gdextention`. If you placed this folder (where `README.md` is located) inside the `addons` folder of the Godot project, you don't have to modify them.

## Building

### Current platform

```shell
cargo build
```

### Multiple and Cross platforms

### Preparing on Windows

1. Instal WSL 2+
2. Install Podman or Docker
3. Install `cargo install cross`
4. Install platform targets:

```shell
rustup target add x86_64-unknown-linux-gnu
```

### Building

Podman or Docker must be running

```shell
cross build --target x86_64-unknown-linux-gnu
```

### References

- https://rust-lang.github.io/rustup/cross-compilation.html
- https://doc.rust-lang.org/nightly/rustc/platform-support.html
