#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

extern uint32_t addition(uint32_t, uint32_t);

// extern uint32_t actual_addition(uint32_t a, uint32_t b);

uint32_t actual_addition(uint32_t a, uint32_t b) { return a + b; }

int main(void) {
  uint32_t sum = addition(1, 2);
  printf("c -: %" PRIu32 "\n", sum);
}