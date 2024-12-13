#include "05.h"

/* INIT AND POPULATE */

// Count how many seeds
// Malloc seed array
// Populate seed array

// Need: dest[map_amount], src[map_amount], range[map_amount]

// Have size_of_map int array
// While get_next_line
	// get_next_line with realloc into file_array
	// Search for "map" in current line with strstr and if found, increase map_amount
		// While line starts with digit, increase size_of_map[map_amount - 1]

// While file array
	// While file_array[i] and line does not start with \n
		// atol first number into dest[], second into src[], third into range[]


/* CALCULATE */

// Lowest seed = first seed

// For each seed in seed array

	// While maps

		// While map

			// If number >= src && number <= src + range

				// number += dest - src;
				// break;

	// If number less than lowest seed, lowest number is new seed

// Return lowest seed


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