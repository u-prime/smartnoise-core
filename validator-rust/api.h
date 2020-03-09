/* Generated with cbindgen:0.9.1 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int64_t len;
  uint8_t *data;
} ByteBuffer;

ByteBuffer accuracy_to_privacy_usage(const uint8_t *request_ptr, int32_t request_length);

ByteBuffer compute_privacy_usage(const uint8_t *request_ptr, int32_t request_length);

ByteBuffer expand_graph(const uint8_t *request_ptr, int32_t request_length);

ByteBuffer generate_report(const uint8_t *request_ptr, int32_t request_length);

ByteBuffer privacy_usage_to_accuracy(const uint8_t *request_ptr, int32_t request_length);

ByteBuffer validate_analysis(const uint8_t *request_ptr, int32_t request_length);