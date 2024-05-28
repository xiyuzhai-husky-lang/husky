#include <stdio.h>
#include <stdlib.h>

int x = 0;

int y() { return x + 1; }

int y_with_x(int new_x) {
    int old_x = new_x;
    x = new_x;
    int y_value = y();
    x = old_x;
    return y_value;
}

int main() {
    printf("y equals %d\n", y());
    printf("y with x = 2 equals %d\n", y_with_x(2));
    printf("y equals %d\n", y());
}