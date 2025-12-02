#ifndef __MEMORY_INSPECTOR_HACK_H
#define __MEMORY_INSPECTOR_HACK_H

#ifdef __cplusplus
extern "C" {
#endif

void *__wrap_malloc(size_t size);

void __wrap_free(void *ptr);

#ifdef __cplusplus
}
#endif
#endif