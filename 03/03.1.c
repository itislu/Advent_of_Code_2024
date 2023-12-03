#include "03.h"

bool	is_symbol(char c)
{
	if (strchr(SYMBOLS, c))
		return (true);
	else
		return (false);
}

bool	is_bottom_symbol(char **map, int y, int x, int lines)
{
	if (++y != lines)
		if (is_symbol(map[y][x]))
			return (true);
	return (false);
}

bool	is_top_symbol(char **map, int y, int x, int lines)
{
	if (--y != -1)
		if (is_symbol(map[y][x]))
			return (true);
	return (false);
}

bool	check_triple(char **map, int y, int x, int lines)
{
	if (x != -1 && map[y][x] && !isdigit(map[y][x]))
	{
		if (is_symbol(map[y][x]))
			return (true);
		if (is_top_symbol(map, y, x, lines))
			return (true);
		if (is_bottom_symbol(map, y, x, lines))
			return (true);
	}
	return (false);
}

bool	is_part(char **map, int y, int x, int lines)
{
	if (check_triple(map, y, x - 1, lines))
		return (true);
	while (isdigit(map[y][x]))
	{
		if (is_top_symbol(map, y, x, lines))
			return (true);
		if (is_bottom_symbol(map, y, x, lines))
			return (true);
		x++;
	}
	if (check_triple(map, y, x, lines))
		return (true);
	return (false);
}

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

	*map = NULL;
	fd = open("input.txt", O_RDONLY);
	if (fd == -1)
		return (0);
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
	char	**map;
	int		lines;
	int		x;
	int		y;
	char	*nptr;
	int		result;

	lines = populate_map(&map);
	result = 0;

	y = 0;
	while (y < lines)
	{
		x = 0;
		while (map[y][x])
		{
			if (isdigit(map[y][x]))
			{
				nptr = &map[y][x];
				if (is_part(map, y, x, lines))
					result += atoi(nptr);
				while (isdigit(map[y][x]))
					x++;
			}
			else
				x++;
		}
		y++;
	}
	free_map(&map, lines);
	printf("%d\n", result);
}