size_t str_len(const char* s) {
    const char* p = s;

    while (*p != '\0') {
        p++;
    }
    return p-s;
}

int main(void) {
    char str[] = "as234523452345";
    str_len(str);
    return 0;
}
