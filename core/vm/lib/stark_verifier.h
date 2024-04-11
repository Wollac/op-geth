#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct VarLengthArray {
  const uint8_t *ptr;
  uintptr_t len;
} VarLengthArray;

/**
 * Verifies a receipt for a given image_id.
 * Returns 0 if the receipt is valid, 1 if the receipt did not deserialize, 2 if the receipt did not verify.
 */
uint8_t verify(const uint8_t (*image_id)[32],
               struct VarLengthArray receipt);
