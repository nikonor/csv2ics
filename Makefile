run:
	cargo run 2025-09-29 ./test.csv ./test.ics

test:
	cargo test

error:
	cargo run 2025-09-30 ./test.csv ./test.ics

all:
	cargo test && cargo run 2025-09-29 ./test.csv ./test.ics
