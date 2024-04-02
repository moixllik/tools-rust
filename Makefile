run:
	cargo run

serve:
	http-server docs

rust-config: cargo-install
	rustup target add wasm32-unknown-unknown

cargo-install:
	cargo install http-server