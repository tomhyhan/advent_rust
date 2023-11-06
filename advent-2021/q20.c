#include "lib.h"

#define MARGIN (100)
#define IMAGE (100)
#define IMAGE_SIZE (IMAGE + MARGIN * 2)
#define DECODE_SIZE (512)

void parse_input(FILE *file, char decode[], char image[][IMAGE_SIZE])
{
    size_t i = 0, row = MARGIN, col = MARGIN;
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
                col = MARGIN;
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

size_t generate_decode_idx(size_t row, size_t col, char image[][IMAGE_SIZE], size_t l_bound, size_t h_bound, bool is_odd)
{
    int32_t i, j;
    size_t idx = 0;
    
    for (i = -1; i < 2; i++)
    {
        for (j = -1; j < 2; j++)
        {
            size_t nr = row + i, nc = col + j;

            if (is_odd && (nr <= l_bound || nr >= h_bound || nc <= l_bound || nc >= h_bound)) {
            idx = idx << 1 | 1;
            continue;
            }

            if (image[nr][nc] == '#') {
                idx = idx << 1 | 1;
            } else {
                idx = idx << 1 | 0;
            }
        }
    }
    assert(idx >= 0 && idx < 512);
    return idx;
}

void copy_image(char image_dest[][IMAGE_SIZE], char image_src[][IMAGE_SIZE]) {
    size_t i;
    for (i=0; i < IMAGE_SIZE; i ++) {
        memcpy(image_dest[i], image_src[i], sizeof(image_dest[i]));
    }
}

void enhance_image(char decode[], char image[][IMAGE_SIZE], size_t iteration)
{
    size_t i;
    for (i = 1; i <= iteration; i++) {
        size_t row, col;
        size_t l_bound, h_bound;
        char new_image[IMAGE_SIZE][IMAGE_SIZE];
        init_image(new_image);
        l_bound = MARGIN - i ;
        h_bound = MARGIN + IMAGE + i - 1 ;
        for (row = l_bound; row <= h_bound; row++) {
            for (col = l_bound; col <= h_bound; col++) {
                size_t idx = generate_decode_idx(row, col, image, l_bound, h_bound, i % 2 == 0);
                new_image[row][col] = decode[idx];
            }
        }
        copy_image(image, new_image);
    }
}

int64_t lit_pixels(char image[][IMAGE_SIZE]) {
    int64_t pixels = 0;
    size_t i,j;
    for (i = 0; i < IMAGE_SIZE; i++)
    {
        for (j = 0; j < IMAGE_SIZE; j++)
        {
            if (image[i][j] == '#') {
                pixels++;
            }
        }
    }
    return pixels;
}

void solution(FILE *file)
{
    char decode[DECODE_SIZE] = {0};
    int64_t pixels;
    char image[IMAGE_SIZE][IMAGE_SIZE];
    init_image(image);

    parse_input(file, decode, image);
    enhance_image(decode, image, 50);
    pixels = lit_pixels(image);
    printf("pixels: %lld\n", pixels);
}

AOC_MAIN_ONE("./inputs/q20.txt")
