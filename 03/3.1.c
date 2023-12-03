#include "3.h"

void	free_map(char ***map, int lines)
{
	for (int i = 0; i < lines; i++)
		free((*map)[i]);
	free(*map);
	*map = NULL;
}

int	populate_map(char ***map)
{
	int		fd;
	int		lines;
	char	*line;
	char	**tmp;

	fd = open("input.txt", O_RDONLY);
	*map = NULL;
	lines = 0;
	while ((line = get_next_line(fd)))
	{
		lines++;
		tmp = realloc(*map, lines * sizeof(char *));
		if (!tmp)
		{
			free_map(map, lines);
			return (0);
		}

		tmp[lines - 1] = line;
		*map = tmp;
	}
	close(fd);
	return (lines);
}

int	main(int argc, char *argv[])
{
	int		lines;
	char	**map;
	int		result;

	lines = populate_map(&map);
	result = 0;

	free_map(&map, lines);
}