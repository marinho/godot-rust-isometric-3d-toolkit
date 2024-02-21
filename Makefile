watch_build:
	cargo watch -x "build"
watch_test:
	cargo watch -x "test"
build_all:
	echo "TODO"
add_targets:
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-unknown-linux-gnu