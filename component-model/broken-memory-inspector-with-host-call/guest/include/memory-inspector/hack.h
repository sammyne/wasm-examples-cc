#ifndef __MEMORY_INSPECTOR_HACK_H
#define __MEMORY_INSPECTOR_HACK_H

#ifdef __cplusplus
extern "C" {
#endif

// man 3 malloc 输出中的 SYNOPSIS 小节列出的函数应该都需要包装。

void *__wrap_calloc(size_t nmemb, size_t size);

void __wrap_free(void *ptr);

void *__wrap_malloc(size_t size);

void *__wrap_realloc(void *ptr, size_t size);

void *__wrap_reallocarray(void *ptr, size_t nmemb, size_t size);

#ifdef __cplusplus
}
#endif
#endif