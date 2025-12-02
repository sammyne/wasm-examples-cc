#include "helloworld.h"
#include "memory-inspector/hack.h"

#include <iostream>
#include <string>
#include <cstdlib>
#include <cstdint>
#include <cstdio>

// 冗余的 __real_* 函数声明是必须的
extern "C" {
  void *__real_calloc(size_t nmemb, size_t size);
  void __real_free(void *ptr);
  void *__real_malloc(size_t);
  void *__real_realloc(void *ptr, size_t size);
  void *__real_reallocarray(void *ptr, size_t nmemb, size_t size);
}

void *__wrap_calloc(size_t nmemb, size_t size) {
  return __real_calloc(nmemb, size);
}

void __wrap_free(void *ptr) {
  printf("free(%p)\n", ptr);
  __real_free(ptr);
}

void *__wrap_malloc(size_t size) {
  // 不能直接调用 malloc，否则递归导致栈溢出。
  auto p = __real_malloc(size);

  printf("malloc(%zu)=%p\n", size, p);

  return p;
}

void *__wrap_realloc(void *ptr, size_t size) {
  // 不能直接调用 realloc，否则递归导致栈溢出。
  auto p = __real_realloc(ptr, size);

  // TODO: 排查清楚加这行代码会导致失败的原因
  // printf("realloc(%zu)=%p\n", size, p);

  return p;
}

void *__wrap_reallocarray(void *ptr, size_t nmemb, size_t size) {
  return __real_reallocarray(ptr, nmemb, size);
}

// ref: https://github.com/grpc/grpc-go/blob/master/examples/helloworld/greeter_server/main.go#L43
void exports_sammyne_helloworld_greeter_say_hello(exports_sammyne_helloworld_greeter_hello_request_t *req, exports_sammyne_helloworld_greeter_hello_reply_t *ret) {
  printf("request.name=%p\n", req->name.ptr);

  std::string message((const char*)(req->name.ptr), req->name.len);
  printf("message.c_str=%p\n", message.c_str());

  message = std::string("Hello ") + message;
  printf("message.c_str=%p\n", message.c_str());

  helloworld_string_dup(&ret->message, message.c_str());
  printf("ret->message.ptr=%p\n", ret->message.ptr);
}
