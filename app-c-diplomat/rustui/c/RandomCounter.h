#ifndef RandomCounter_H
#define RandomCounter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct RandomCounter RandomCounter;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

RandomCounter* RandomCounter_new();

int32_t RandomCounter_increment(RandomCounter* self);

int32_t RandomCounter_get_value(const RandomCounter* self);
void RandomCounter_destroy(RandomCounter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
