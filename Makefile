CC = clang
CFLAGS=-Wall -Wextra -Werror -std=c99 -g

source = clox.c chunk.c debug.c value.c memory.c vm.c compiler.c scanner.c
objects = ${patsubst %.c,build/%.o,${source}}

.PHONY: all
all: clox

clox: $(objects)
	$(CC) $(CFLAGS) $(objects) -o build/clox

build/%.o: %.c
	mkdir -p build
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf build
