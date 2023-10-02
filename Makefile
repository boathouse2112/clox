CC = clang
CFLAGS=-Wall -Wextra -Werror -std=c99 -g

.PHONY: all
all: clox

clox: clox.o chunk.o
	$(CC) $(CFLAGS) clox.o -o clox

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf clox
	rm -rf *.o
