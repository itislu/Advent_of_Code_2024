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
	int		fd;
	int		floor;
	char	*line;
	int		i;

	fd = open(argv[1], O_RDONLY);
	line = get_next_line(fd);
	floor = 0;
	i = 0;
	while (line[i])
	{
		if (line[i] == '(')
			floor++;
		else if (line[i] == ')')
			floor--;
		i++;
	}
	printf("%d\n", floor);
}