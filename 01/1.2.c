#include <ctype.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include "../utils/get_next_line.h"
#include "../utils/get_next_line.c"
#include "../utils/get_next_line_utils.c"

int	main(int argc, char *argv[])
{
	int		first_digit;
	int		last_digit;
	int		i;
	int		j;
	int		result = 0;
	char	*line;
	int		fd;
	char	*strings[] = {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine", NULL};
	char	*intptr;
	char	*tmp;
	char	*tmp2;
	int		k;
	int		l;

	fd = open(argv[1], O_RDONLY);
	line = get_next_line(fd);
	while (line)
	{
		i = 0;
		while (!isdigit(line[i]) && line[i])
			i++;
		j = 0;
		k = 10;
		intptr = &line[i];
		while (strings[j])
		{
			tmp = strstr(line, strings[j]);
			j++;
			if (tmp && tmp < intptr)
			{
				intptr = tmp;
				k = j;
			}
		}
		if (intptr == &line[i])
		{
			first_digit = atoi(intptr);
			while (first_digit > 9)
				first_digit /= 10;
		}
		else
		{
			first_digit = k;
		}

		i = strlen(line);
		while (!isdigit(line[i]) && i > 0)
			i--;
		j = 0;
		k = 0;
		intptr = &line[i];
		while (strings[j])
		{
			tmp = strstr(intptr, strings[j]);
			tmp2 = tmp;
			l = 1;
			while (line[i + l])
			{
				tmp2 = strstr(&line[i + l], strings[j]);
				if (tmp2 > tmp)
					tmp = tmp2;
				l++;
			}
			if (tmp >= intptr)
			{
				intptr = tmp;
				k = j + 1;
			}
			j++;
		}
		if (k == 0)
		{
			last_digit = atoi(intptr);
			if (last_digit > 9)
				last_digit %= 10;
		}
		else
		{
			last_digit = k;
		}

		result += first_digit * 10 + last_digit;
		line = get_next_line(fd);
	}

	printf("%d\n", result);
	close(fd);
}
