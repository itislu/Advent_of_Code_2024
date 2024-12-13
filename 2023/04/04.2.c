#include "04.h"

int	free_all_and_exit(int **winning_numbers, int *winning_num_amount, int **my_numbers, int *my_num_amount, int card_amount)
{
    for (int i = 0; i < card_amount; i++)
	{
        free(winning_numbers[i]);
        free(my_numbers[i]);
    }
    free(winning_numbers);
    free(my_numbers);
    free(winning_num_amount);
    free(my_num_amount);

	exit (1);
}

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

int create_arrays(int ***winning_numbers, int **winning_num_amount, int ***my_numbers, int **my_num_amount)
{
	int		fd;
	char	*line;
	int		pos;
	int		card_amount = 0;

	*winning_numbers = NULL;
	*winning_num_amount = NULL;
	*my_numbers = NULL;
	*my_num_amount = NULL;

	fd = open("input.txt", O_RDONLY);
	line = get_next_line(fd);
	while (line)
	{
		pos = skip_beginning(line);

		*winning_num_amount = realloc(*winning_num_amount, (card_amount + 1) * sizeof(int));
		*winning_numbers = realloc(*winning_numbers, (card_amount + 1) * sizeof(int *));
		if (!*winning_num_amount || !*winning_numbers)
			free_all_and_exit(*winning_numbers, *winning_num_amount, *my_numbers, *my_num_amount, card_amount);

		(*winning_num_amount)[card_amount] = how_many_numbers(line, pos);
		(*winning_numbers)[card_amount] = populate_int_array((*winning_num_amount)[card_amount], line, &pos);
		if (!(*winning_numbers)[card_amount])
			free_all_and_exit(*winning_numbers, *winning_num_amount, *my_numbers, *my_num_amount, card_amount);

		while (!isdigit(line[pos]))
			pos++;

		*my_num_amount = realloc(*my_num_amount, (card_amount + 1) * sizeof(int));
		*my_numbers = realloc(*my_numbers, (card_amount + 1) * sizeof(int *));
		if (!*my_num_amount || !*my_numbers)
			free_all_and_exit(*winning_numbers, *winning_num_amount, *my_numbers, *my_num_amount, card_amount);

		(*my_num_amount)[card_amount] = how_many_numbers(line, pos);
		(*my_numbers)[card_amount] = populate_int_array((*my_num_amount)[card_amount], line, &pos);
		if (!(*my_numbers)[card_amount])
			free_all_and_exit(*winning_numbers, *winning_num_amount, *my_numbers, *my_num_amount, card_amount);

		card_amount++;
		line = get_next_line(fd);
	}
	close(fd);
	return (card_amount);
}

int	main(int argc, char *argv[])
{
	int		*copies;
	int		card_amount;
	int		result;
	int		dup;
	int		**winning_numbers;
	int		*winning_num_amount;
	int		**my_numbers;
	int		*my_num_amount;
	int		i;
	int		j;
	int		cur;

	card_amount = create_arrays(&winning_numbers, &winning_num_amount, &my_numbers, &my_num_amount);
	copies = (int *) malloc(card_amount * sizeof(int));
	if (!copies)
		free_all_and_exit(winning_numbers, winning_num_amount, my_numbers, my_num_amount, card_amount);
	for (i = 0; i < card_amount; i++)
		copies[i] = 1;

	result = 0;
	cur = 0;
	while (cur < card_amount)
	{
		dup = 0;
		i = 0;
		while (i < winning_num_amount[cur])
		{
			j = 0;
			while (j < my_num_amount[cur])
			{
				if (winning_numbers[cur][i] == my_numbers[cur][j])
					dup++;
				j++;
			}
			i++;
		}

		for (i = 1; i <= dup; i++)
			copies[cur + i]++;
		if (--copies[cur] == 0)
			cur++;
		result++;
	}

	printf("%d\n", result);

	free(copies);
	free_all_and_exit(winning_numbers, winning_num_amount, my_numbers, my_num_amount, card_amount);
}