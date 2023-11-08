#include <stdlib.h>

int main() {
    int* myVariable = (int*)malloc(sizeof(int));
    free(myVariable);
    return 0;
}
