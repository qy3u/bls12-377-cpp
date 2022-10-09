ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

all: target/debug/libfr.$(EXT)
	g++ src/main.cpp -L ./target/debug/ -lfr -o run
	LD_LIBRARY_PATH=./target/debug/ ./run

target/debug/libfr.$(EXT): src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
	rm -rf run