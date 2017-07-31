all: run

build:
	cargo build --release
	cp ./target/release/libexample.so ./example.so

run: build
	python test.py

clean:
	cargo clean
	rm ./example.so
