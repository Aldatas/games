CC = gcc
CFLAGS = -Wall -Wextra -g
LIBS = $(shell pkg-config --cflags --libs raylib) -lm

game: main.c
	$(CC) $(CFLAGS) main.c -o game $(LIBS)

run: game
	./game

.PHONY: run
