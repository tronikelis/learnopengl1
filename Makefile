main: main.o
	./main.o

main.o: main.c
	clang main.c -o main.o -lm -lglfw -lOpenGL -lpthread 
