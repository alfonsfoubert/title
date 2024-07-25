build:
	cargo build --release

install: build
	mv ./target/release/title ~/.local/bin/

clean:
	rm ~/.local/bin/title
