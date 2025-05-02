all:
	make server
	make cli
server:
	./builder build --release --bin server
cli:
	./builder build --release --bin cli
