CC = clang
CFLAGS=-Wall -Wextra -Werror -std=c99 -g

objects = clox.o chunk.o debug.o value.o memory.o

.PHONY: all
all: clox

clox: $(objects)
	$(CC) $(CFLAGS) $(objects) -o clox

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf clox
	rm -rf *.o
