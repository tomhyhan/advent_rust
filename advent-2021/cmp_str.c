int cmp_str(char a[], char b[]) {
    while (*a != '\0' && *a == *b ) {
        ++a;
        ++b;
    }
    if (*a - *b == 0) {
        return 0;
    }
    return *a - *b > 0? 1 : -1;
}

int main(void) {
    char a[] = "AC";
    char b[] = "AB";
    cmp_str(a,b);
    return 0;
}

