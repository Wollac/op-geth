#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum ErrorCode {
  Ok = 0,
  VerifyError = 1,
  InvalidSeal = 2,
};
typedef uint8_t ErrorCode;

typedef struct VarLengthArray {
  const uint8_t *ptr;
  uintptr_t len;
} VarLengthArray;

/**
 * Verifies the given succinct proof.
 */
ErrorCode verify(const uint8_t (*pre_state)[32],
                 const uint8_t (*post_state)[32],
                 const uint8_t (*input)[32],
                 const uint8_t (*journal)[32],
                 struct VarLengthArray seal);
