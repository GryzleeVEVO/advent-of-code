#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const char *prohibited[] = {"ab", "cd", "pq", "xy"};

int main() {
    FILE *file = fopen("input.txt", "r");

    if (file == NULL) {
        fprintf(stderr, "Couldn't open input");
        return EXIT_FAILURE;
    }

    int nice_strings_1 = 0;
    int nice_strings_2 = 0;

    char *line = NULL;
    size_t n = 0;
    int len = 0;

    while ((len = getline(&line, &n, file)) != -1) {
        int vowels = 0;
        bool repeated = false;
        bool naughty_biword = false;

        bool biword_twice = false;
        bool separated = false;

        for (int i = 0; i < len; i++) {
            // Part 1

            // Vowels
            switch (line[i]) {
            case 'a':
            case 'e':
            case 'i':
            case 'o':
            case 'u':
                vowels++;
            default:
                break;
            }

            if (i > 0) {
                char biword[2] = {line[i - 1], line[i]};

                // Does it contain a letter twice back-to-back?
                if (biword[0] == biword[1]) repeated = true;

                // Does it have a prohibited string?
                for (size_t i = 0; i < 4 && !naughty_biword; i++) {
                    naughty_biword = biword[0] == prohibited[i][0] &&
                                     biword[1] == prohibited[i][1];
                }
            }

            // Part 2

            // Check character two places ahead if possible
            if (!separated) {
                separated = (i + 2 < len && line[i] == line[i + 2]);
            }

            // Check biwords ahead
            if (!biword_twice) {
                if (i + 3 < len) {
                    char biword_1[] = {line[i], line[i + 1]};

                    for (int j = i + 2; j < len - 1 && !biword_twice; j++) {
                        char biword_2[] = {line[j], line[j + 1]};

                        biword_twice = (biword_1[0] == biword_2[0] &&
                                        biword_1[1] == biword_2[1]);
                    }
                }
            }
        }

        if (vowels >= 3 && repeated && !naughty_biword) nice_strings_1++;
        if (biword_twice && separated) nice_strings_2++;
    }

    printf("Part 1: %d\n", nice_strings_1);
    printf("Part 2: %d\n", nice_strings_2);

    if (line) free(line);

    fclose(file);

    return EXIT_SUCCESS;
}