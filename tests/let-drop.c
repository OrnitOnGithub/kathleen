#include <stdlib.h>

int main() {
    int* myVariable = (int*)malloc(sizeof(int));
    *myVariable = 42;
    free(myVariable);
    return 0;
}
