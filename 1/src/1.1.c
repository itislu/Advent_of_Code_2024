#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <fcntl.h>
#include "get_next_line.h"
#include "libft.h"

int	main(int argc, char *argv[])
{
	int	first_digit;
	int	last_digit;
	int	i;
	int	result = 0;
	char	*line;
	int	fd;

	fd = open(argv[1], O_RDONLY);
	line = get_next_line(fd);
	while (line)
	{
		i = 0;
		while (!isdigit(line[i]))
			i++;
		first_digit = atoi(&line[i]);
		while (first_digit > 9)
			first_digit /= 10;

		i = strlen(line);
		i--;
		while (!isdigit(line[i]))
			i--;
		last_digit = atoi(&line[i]);
		if (last_digit > 9)
			last_digit %= 10;

		result += first_digit * 10 + last_digit;
		line = get_next_line(fd);
	}

	printf("%d\n", result);
	close(fd);
}
