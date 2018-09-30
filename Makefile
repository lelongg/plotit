run: client server
	./target/debug/noise | ./target/debug/server

client:
	(cd client && cargo web deploy)

server:
	(cd server && cargo build)

.PHONY: run client server