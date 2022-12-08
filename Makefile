build:
	cargo build --release

run:
	./target/release/hello_rust

test:
	cd hello_world/ ; cargo test

doc:
	cargo doc --open