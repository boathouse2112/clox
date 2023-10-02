CC = clang
CFLAGS=-Wall -Wextra -Werror -std=c99 -g

.PHONY: all
all: main

main: main.o
	$(CC) $(CFLAGS) main.o -o main

main.o: main.c
	$(CC) $(CFLAGS) -c main.c -o main.o

clean:
	rm main
	rm *.o
