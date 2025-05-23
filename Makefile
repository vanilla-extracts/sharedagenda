podman_all:
	make podman_server
	make podman_cli
podman_server:
	./builder build --release --bin server
	cp target/release/server 02-configuration/files/server
	cp target/release/server 04-vps/files/server
podman_cli:
	./builder build --release --bin cli
	cp target/release/cli assets/cli
all:
	cargo build --release
cli:
	cargo build --release --bin cli
server:
	cargo build --release --bin server
