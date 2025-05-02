all:
	make server
	make cli
server:
	./builder build --release --bin server
	cp target/release/server 02-configuration/files/server
cli:
	./builder build --release --bin cli
	cp target/release/cli assets/cli
