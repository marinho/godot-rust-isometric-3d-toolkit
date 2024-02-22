watch_build:
	cargo watch -x "build"
watch_test:
	cargo watch -x "test"
build_release:
	cargo build --release
	#cross build --target x86_64-pc-windows-gnu --release
	cross build --target x86_64-unknown-linux-gnu --release
add_targets:
	cargo install cross
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-unknown-linux-gnu