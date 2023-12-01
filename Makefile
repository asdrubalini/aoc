bench:
	cargo bench

bench-gen:
	cargo criterion --message-format=json | criterion-table > BENCHMARKS.md

test:
	cargo test --release
