#include <ctype.h>
#include <fcntl.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include "../utils/get_next_line.h"
#include "../utils/get_next_line.c"
#include "../utils/get_next_line_utils.c"

#define R 0
#define G 1
#define B 2

void	skip_game(char **line)
{
	while (**line && !isdigit(**line))
		(*line)++;
	while (**line && isdigit(**line))
		(*line)++;
}

int	main(int argc, char *argv[])
{
	char	*line;
	int		fd;
	int		result;
	int		min[3];
	int		num;

	fd = open("input", O_RDONLY);
	// fd = open(argv[1], O_RDONLY);
	line = get_next_line(fd);
	result = 0;
	while (line)
	{
		skip_game(&line);
		min[R] = 1;
		min[G] = 1;
		min[B] = 1;

		while (*line)
		{
			while (*line && !isdigit(*line))
				line++;
			num = atoi(line);
			while (*line && isdigit(*line))
				line++;
			while (*line && isspace(*line))
				line++;
			if (*line == 'r')
			{
				if (num > min[R])
					min[R] = num;
			}
			else if (*line == 'g')
			{
				if (num > min[G])
					min[G] = num;
			}
			else if (*line == 'b')
			{
				if (num > min[B])
					min[B] = num;
			}
		}
		result += min[R] * min[G] * min[B];

		line = get_next_line(fd);
	}
	printf("%d\n", result);
}