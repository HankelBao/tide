run:
	cargo build --release
	./target/release/tide test.py
