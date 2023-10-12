#include "lib.h"
#define PAPER_SIZE 1000

typedef struct {
  bool dots[PAPER_SIZE][PAPER_SIZE];
  uint16_t width, height;
  char* ch;
} paper_t;

int main(void) {
    paper_t *paper = (paper_t*)malloc(sizeof(paper_t));
    paper->width = PAPER_SIZE;
    paper->ch = "aasdf";
    printf("%d\n", paper -> width);
    printf("%s\n", paper -> ch);
    return 0;
}
