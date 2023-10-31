#include "lib.h"

#define MARGIN (10)
#define IMAGE (5)
#define IMAGE_SIZE (IMAGE + MARGIN * 2)
#define DECODE_SIZE (512)

void parse_input(FILE *file, char decode[], char image[][IMAGE_SIZE])
{
    size_t i = 0, row = 2, col = 2;
    char ch;
    while (fscanf_s(file, "%c", &ch, 1) && ch != '\n')
    {
        if (ch == '.' || ch == '#')
        {
            decode[i++] = ch;
        }
    }

    while (fscanf_s(file, "%c", &ch, 1) != EOF)
    {
        if (ch == '.' || ch == '#')
        {
            image[row][col] = ch;
            col++;
            if (col == IMAGE + MARGIN)
            {
                row++;
                col = 2;
            }
        }
    }
}

void init_image(char image[][IMAGE_SIZE])
{
    size_t i, j;

    for (i = 0; i < IMAGE_SIZE; i++)
    {
        for (j = 0; j < IMAGE_SIZE; j++)
        {
            image[i][j] = '.';
        }
    }
}

void print_image(char image[][IMAGE_SIZE])
{
    size_t i, j;

    for (i = 0; i < IMAGE_SIZE; i++)
    {
        for (j = 0; j < IMAGE_SIZE; j++)
        {
            printf("%c", image[i][j]);
        }
        printf("\n");
    }
}

size_t generate_decode_idx(size_t row, size_t col, char image[][IMAGE_SIZE])
{
    int32_t i, j;
    size_t idx = 0;
    size_t offset = 8;
    for (i = -1; i < 2; i++)
    {
        for (j = -1; j < 2; j++)
        {
            size_t nr = row + i, nc = col + j;
            if (image[nr][nc] == '.')
            {
                idx = idx | 0 << offset;
            }
            else if (image[nr][nc] == '#')
            {
                idx = idx | 1 << offset;
            }
            offset--;
        }
    }
    printf("%d %d %d\n", row, col, idx);
    return idx;
}

void enhance_image(char decode[], char image[][IMAGE_SIZE], size_t iteration)
{
    size_t i;
    for (i = 1; i < iteration; i++)
    {
        size_t row, col;
        char new_image[IMAGE_SIZE][IMAGE_SIZE];
        init_image(new_image);

        for (row = iteration - i; row < iteration + IMAGE + i; row++)
        {
            for (col = iteration - i; col < iteration + IMAGE + i; col++)
            {
                size_t idx = generate_decode_idx(row, col, image);
                printf("decode: %c\n", decode[idx]);
                new_image[row][col] = decode[idx];
            }
        }
        print_image(new_image);
        break;
    }
}

void solution(FILE *file)
{
    char decode[512] = {0};
    char image[IMAGE_SIZE][IMAGE_SIZE];
    init_image(image);

    parse_input(file, decode, image);
    enhance_image(decode, image, 2);
}

AOC_MAIN_ONE("./inputs/q20.txt")
