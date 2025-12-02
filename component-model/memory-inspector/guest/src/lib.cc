#include "helloworld.h"
#include "memory-inspector/hack.h"

#include <iostream>
#include <string>
#include <cstdlib>
#include <cstdint>
#include <cstdio>

// 冗余的 __real_* 函数声明是必须的
extern "C" {
  void *__real_malloc(size_t);
  void __real_free(void *ptr);
}

void *__wrap_malloc(size_t size) {
  // 不能直接调用 malloc，否则递归导致栈溢出。
  auto p = __real_malloc(size);

  auto v = (uint64_t)p;
  printf("malloc(%ld)=0x%llx\n", size, v);

  return p;
}

void __wrap_free(void *ptr) {
  printf("free(%p)\n", ptr);
  __real_free(ptr);
}

void exports_helloworld_say_hello_again(helloworld_hello_request_t *req, helloworld_hello_reply_t *ret) {
  std::string message((const char*)(req->name.ptr), req->name.len);

  message = std::string("Hello again ") + message;

  helloworld_string_dup(&ret->message, message.c_str());
}


// ref: https://github.com/grpc/grpc-go/blob/master/examples/helloworld/greeter_server/main.go#L43
void exports_sammyne_helloworld_greeter_say_hello(exports_sammyne_helloworld_greeter_hello_request_t *req, exports_sammyne_helloworld_greeter_hello_reply_t *ret) {
  std::string message((const char*)(req->name.ptr), req->name.len);

  message = std::string("Hello ") + message;

  helloworld_string_dup(&ret->message, message.c_str());
}
