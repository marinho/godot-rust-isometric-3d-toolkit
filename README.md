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

1. Copy `toolkit.gdextension.template` as just `toolkit.gdextention` (or another name as you prefer) into the Godot project root folder.
2. Update file paths in `toolkit.gdextention`. If you placed this folder (where `README.md` is located) inside the `addons` folder of the Godot project, you don't have to modify them.

## Building

### Current platform

```shell
cargo build
```

### Multiple and Cross platforms

TBD

- https://rust-lang.github.io/rustup/cross-compilation.html
- https://doc.rust-lang.org/nightly/rustc/platform-support.html
