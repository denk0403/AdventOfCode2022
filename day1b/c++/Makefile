asm:
	clang++ -S -O3 -masm=intel main.cpp -std=c++20

build:
	clang++ -O3 main.cpp -o ../main -std=c++20

run: build
	../main
