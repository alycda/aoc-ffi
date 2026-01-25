# Makefile for C++ qsort vs std::sort comparison

CXX = g++
CXXFLAGS = -std=c++17 -O3 -Wall -Wextra

# Targets
all: main

main: main.cpp
	$(CXX) $(CXXFLAGS) -o main main.cpp

run: main
	./main

clean:
	rm -f main

# Compare with Rust version
rust-run:
	cargo run --release

compare: main rust-run
	@echo "\nNow running C++ version:"
	./main

.PHONY: all run clean rust-run compare
