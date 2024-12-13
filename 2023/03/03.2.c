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

int	check_and_get_part_number(char **map, int y, int x, int lines)
{
	int	value;

	while (x - 1 != -1 && isdigit(map[y][x - 1]))
		x--;
	if (is_part(map, y, x, lines))
		value = atoi(&map[y][x]);
	else
		value = -1;
	return (value);
}

bool	is_right_digit(char **map, int y, int x, int lines)
{
	if (isdigit(map[y][++x]))
		return (true);
	return (false);
}

bool	is_left_digit(char **map, int y, int x, int lines)
{
	if (--x != -1)
		if (isdigit(map[y][x]))
			return (true);
	return (false);
}

bool	is_bottom_digit(char **map, int y, int x, int lines)
{
	if (++y != lines && x != -1)
		if (isdigit(map[y][x]))
			return (true);
	return (false);
}

bool	is_top_digit(char **map, int y, int x, int lines)
{
	if (--y != -1 && x != -1)
		if (isdigit(map[y][x]))
			return (true);
	return (false);
}

int	two_adjacent_part_numbers(char **map, int y, int x, int lines)
{
	int	count;
	int	tmp;
	int	parts[2];

	count = 0;

	/* Check top */
	if (is_top_digit(map, y, x, lines))
	{
		tmp = check_and_get_part_number(map, y - 1, x, lines);
		if (tmp != -1)
		{
			parts[count] = tmp;
			count++;
		}
	}
	else	// Check top diagonals
	{
		if (is_top_digit(map, y, x - 1, lines))
		{
			tmp = check_and_get_part_number(map, y - 1, x - 1, lines);
			if (tmp != -1)
			{
				parts[count] = tmp;
				count++;
			}
		}
		if (is_top_digit(map, y, x + 1, lines))
		{
			tmp = check_and_get_part_number(map, y - 1, x + 1, lines);
			if (tmp != -1)
			{
				parts[count] = tmp;
				count++;
			}
		}
	}

	/* Check bottom */
	if (is_bottom_digit(map, y, x, lines))
	{
		tmp = check_and_get_part_number(map, y + 1, x, lines);
		if (tmp != -1)
		{
			if (count == 2)
			{
				map[y][x] = '+';
				return (0);
			}
			parts[count] = tmp;
			count++;
		}
	}
	else	// Check bottom diagonals
	{
		if (is_bottom_digit(map, y, x - 1, lines))
		{
			tmp = check_and_get_part_number(map, y + 1, x - 1, lines);
			if (tmp != -1)
			{
				if (count == 2)
				{
					map[y][x] = '+';
					return (0);
				}
				parts[count] = tmp;
				count++;
			}
		}
		if (is_bottom_digit(map, y, x + 1, lines))
		{
			tmp = check_and_get_part_number(map, y + 1, x + 1, lines);
			if (tmp != -1)
			{
				if (count == 2)
				{
					map[y][x] = '+';
					return (0);
				}
				parts[count] = tmp;
				count++;
			}
		}
	}

	/* Check left */
	if (is_left_digit(map, y, x, lines))
	{
		tmp = check_and_get_part_number(map, y, x - 1, lines);
		if (tmp != -1)
		{
			if (count == 2)
			{
				map[y][x] = '+';
				return (0);
			}
			parts[count] = tmp;
			count++;
		}
	}

	/* Check right */
	if (is_right_digit(map, y, x, lines))
	{
		tmp = check_and_get_part_number(map, y, x + 1, lines);
		if (tmp != -1)
		{
			if (count == 2)
			{
				map[y][x] = '+';
				return (0);
			}
			parts[count] = tmp;
			count++;
		}
	}

	if (count == 2)
		return (parts[0] * parts[1]);
	else
	{
		map[y][x] = '-';
		return (0);
	}
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
	long	result;

	lines = populate_map(&map);
	result = 0;

	y = 0;
	while (y < lines)
	{
		x = 0;
		while (map[y][x])
		{
			if (map[y][x] == '*')
				result += two_adjacent_part_numbers(map, y, x, lines);
			x++;
		}
		y++;
	}
	free_map(&map, lines);
	printf("%ld\n", result);
}