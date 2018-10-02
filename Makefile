run:build
	./target/debug/noise | ./target/debug/server

build: client server

client:
	(cd client && cargo web deploy)

server:
	(cd server && cargo build)

.PHONY: run build client server