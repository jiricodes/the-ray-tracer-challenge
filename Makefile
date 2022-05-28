all:
	cargo run --release -p chapter_ends > test.ppm
	open test.ppm