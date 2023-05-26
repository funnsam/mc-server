run:
	cargo r -r -j 16
build:
	cargo b -r -j 16
	- rm mc-server
	cp target/release/mc-server .
