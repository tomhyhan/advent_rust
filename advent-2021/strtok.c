#include <stdio.h>
#include <string.h>

char* strtok1(char* string, const char* delimiter) {
    static char* line = NULL;
    char* token = NULL;
    
    if (string != NULL) {
        line = string;
    }
    
    if (*line == '\0') {
        return NULL;
    } 

    line += strspn(line, delimiter);

    token = line;   
    for (; *line != '\0'; line++) {
        const char* current_d = delimiter; 
        for (; *current_d != '\0'; current_d++) {
            if (*line == *current_d) {
                *line = '\0';
                line++;
                return token;
            }
        }
    }   
    return token;
} 

int main(void) {
    char str[] = "Hello, world!";
    char delimiter[] = ", ";
    char* token = strtok1(str, delimiter);

    while (token != NULL) {
        printf("%s\n", token);
        token = strtok1(NULL, delimiter);
    }

    return 0;
}
