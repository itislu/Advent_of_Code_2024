#include "2.h"

int	max_red = 12;
int	max_green = 13;
int	max_blue = 14;

int	get_game(char **line)
{
	int	game;

	game = 0;
	while (**line && !isdigit(**line))
		(*line)++;
	game = atoi(*line);
	while (**line && isdigit(**line))
		(*line)++;
	return (game);
}

int	main(int argc, char *argv[])
{
	char	*line;
	int		fd;
	int		game;
	int		max;
	bool	possible;
	int		result;

	fd = open("input", O_RDONLY);
	// fd = open(argv[1], O_RDONLY);
	line = get_next_line(fd);
	result = 0;
	while (line)
	{
		possible = true;
		game = get_game(&line);

		while (*line)
		{
			while (*line && !isdigit(*line))
				line++;
			max = atoi(line);
			while (*line && isdigit(*line))
				line++;
			while (*line && isspace(*line))
				line++;
			if (*line == 'r')
			{
				if (max > max_red)
					possible = false;
			}
			else if (*line == 'g')
			{
				if (max > max_green)
					possible = false;
			}
			else if (*line == 'b')
			{
				if (max > max_blue)
					possible = false;
			}
		}
		if (possible)
			result += game;

		line = get_next_line(fd);
	}
	printf("%d\n", result);
}