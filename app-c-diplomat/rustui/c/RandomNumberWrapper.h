#ifndef RandomNumberWrapper_H
#define RandomNumberWrapper_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct RandomNumberWrapper RandomNumberWrapper;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

RandomNumberWrapper* RandomNumberWrapper_new(int32_t value);

int32_t RandomNumberWrapper_value(const RandomNumberWrapper* self);

int32_t RandomNumberWrapper_generate(const RandomNumberWrapper* self);
void RandomNumberWrapper_destroy(RandomNumberWrapper* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
