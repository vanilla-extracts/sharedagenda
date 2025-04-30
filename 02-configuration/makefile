all:
	make server
	make cli
server:
	cargo podman build --release --bin server
cli:
	cargo build --release --bin cli
