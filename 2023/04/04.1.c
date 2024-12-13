#include "04.h"

int	*populate_int_array(int winning_num_amount, char *line, int *pos)
{
	int	*winning_numbers;
	int	i = 0;

	winning_numbers = (int *) malloc((winning_num_amount * sizeof(int)));
	if (!winning_numbers)
		return (NULL);
	while (i < winning_num_amount)
	{
		winning_numbers[i++] = atoi(&line[*pos]);
		while (isdigit(line[*pos]))
			(*pos)++;
		while (isspace(line[*pos]))
			(*pos)++;
	}
	return (winning_numbers);
}

int	how_many_numbers(char *line, int pos)
{
	int	count = 0;

	while (isdigit(line[pos]))
	{
		count++;
		while (isdigit(line[pos]))
			pos++;
		while (isspace(line[pos]))
			pos++;
	}
	return (count);
}

int	skip_beginning(char *line)
{
	int	pos = 0;

	while (line[pos] && line[pos] != ':')
		pos++;
	pos++;
	while (isspace(line[pos]))
		pos++;
	return (pos);
}

int	main(int argc, char *argv[])
{
	int		fd;
	int		pos;
	char	*line;
	int		result = 0;
	int		*winning_numbers;
	int		winning_num_amount;
	int		*my_numbers;
	int		my_num_amount;
	int		i;
	int		j;
	int		value;

	fd = open("input.txt", O_RDONLY);
	line = get_next_line(fd);
	while (line)
	{
		pos = skip_beginning(line);
		winning_num_amount = how_many_numbers(line, pos);
		winning_numbers = populate_int_array(winning_num_amount, line, &pos);
		if (!winning_numbers)
			return (1);
		while (!isdigit(line[pos]))
			pos++;
		my_num_amount = how_many_numbers(line, pos);
		my_numbers = populate_int_array(my_num_amount, line, &pos);
		if (!my_numbers)
			return (1);	// No time to code freeing...

		value = 0;
		i = 0;
		while (i < winning_num_amount)
		{
			j = 0;
			while (j < my_num_amount)
			{
				if (winning_numbers[i] == my_numbers[j])
				{
					if (!value)
						value = 1;
					else
						value *= 2;
				}
				j++;
			}
			i++;
		}

		result += value;
		line = get_next_line(fd);
	}

	printf("%d\n", result);
	close(fd);
}