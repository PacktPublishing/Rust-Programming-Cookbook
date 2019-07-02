#include <stdio.h>

// A function with that name is expected to be linked to the project
extern char* digest(char *str);

// This function is exported under the name pre_digest
extern void pre_digest() {
    printf("pre_digest called\n");
}

int main() {
    char *result = digest("Hello World");
    printf("SHA digest of \"Hello World\": %s", result);
    return 0;
}
