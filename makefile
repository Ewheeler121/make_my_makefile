CC = gcc

SRC_FILES = 


                             OBJ_FILES = $(SRC_FILES:.c=.o)


                             EXECUTABLE = 
all: $EXECUTABLE


                             $(EXECUTABLE): $(OBJ_FILES)
	$(CC) $(CFLAGS) $(OBJ_FILES -o $(EXECUTABLE)


                             %.o %.c
	$(CC) $(CFLAGS) -c $< -o $@


                             clean:
	rm -f $(OBJ_FILES) $(EXECUTABLE)