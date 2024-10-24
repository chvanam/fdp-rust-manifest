#include <stdio.h>
#include "rustui/c/diplomat_runtime.h"
#include "rustui/c/RandomCounter.h"

int main() {
    printf("RandomCounter example:\n");

    RandomCounter *counter = RandomCounter_new();
    
    printf("Initial value: %d\n", RandomCounter_get_value(counter));

    for (int i = 0; i < 3; i++) {
        int value = RandomCounter_increment(counter);
        printf("Counter value after increment: %d\n", value);
    }

    RandomCounter_destroy(counter);

    return 0;
}