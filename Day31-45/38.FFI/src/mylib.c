#include <stdio.h>
#include <stdint.h>

// 1. Simple function
int c_add(int a, int b) {
    return a + b;
}

// 2. Function with pointer (String)
void c_print_hello(const char* name) {
    printf("Hello from C, %s!\n", name);
}

// 3. Struct manipulation
typedef struct {
    int x;
    int y;
} Point;

void c_move_point(Point* p, int dx, int dy) {
    p->x += dx;
    p->y += dy;
}
