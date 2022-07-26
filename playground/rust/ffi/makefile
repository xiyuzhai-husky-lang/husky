test-integers:
	cargo build
	gcc --std=c11 csrc/ffi-integers/main.c -L target/debug/ -lffi_integers
	LD_LIBRARY_PATH=target/debug/ ./a.out
	LD_LIBRARY_PATH=target/debug/ python pysrc/ffi-integers/main.py

test-integers-mirror:
	cargo build
	gcc --std=c11 csrc/ffi-integers-mirror/main.c -L target/debug/ -lffi_integers_mirror
	LD_LIBRARY_PATH=target/debug/ ./a.out
	LD_LIBRARY_PATH=target/debug/ python pysrc/ffi-integers-mirror/main.py || echo expect to fail

test-string-arguments:
	cargo build
	gcc --std=c11 csrc/ffi-string-arguments/main.c -L target/debug/ -lffi_string_arguments
	LD_LIBRARY_PATH=target/debug/ ./a.out
	LD_LIBRARY_PATH=target/debug/ python pysrc/ffi-string-arguments/main.py