fuzzing:
	cargo fuzz run fuzz_fixed -j $(shell nproc) -- -max_len=32

fuzzing-ignore-crashes:
	cargo fuzz run fuzz_fixed -j $(shell nproc) -- -max_len=32  -ignore_crashes=1

stop:
	pkill -f fuzz