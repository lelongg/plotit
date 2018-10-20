run:build
	./target/release/noise | ./target/release/plotit

build: client server

client:
	(cd client && cargo web deploy --release)

server:
	(cd server && cargo build --release)

.PHONY: run build client server