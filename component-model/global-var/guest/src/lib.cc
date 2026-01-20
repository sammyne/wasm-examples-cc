#include "helloworld.h"
#include "global-var/hack.h"

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

std::string dummy(const char *tag) {
  auto out = std::string("dummy");
  printf("[dummy-%s] addr(out)=%p, addr(out.c_str())=%p)\n", tag, &out, out.c_str());
  return out;
}

static std::string HELLO_WORLD = dummy("static");

std::string HELLO_WORLD2 = dummy("non-static");

// ref: https://github.com/grpc/grpc-go/blob/master/examples/helloworld/greeter_server/main.go#L43
void exports_sammyne_helloworld_greeter_say_hello(exports_sammyne_helloworld_greeter_hello_request_t *req, exports_sammyne_helloworld_greeter_hello_reply_t *ret) {
  std::string message((const char*)(req->name.ptr), req->name.len);

  message = std::string("Hello ") + message;

  printf("addr(HELLO_WORLD)=%p, HELLO_WORLD='%s'\n", HELLO_WORLD.c_str(), HELLO_WORLD.c_str());
  HELLO_WORLD = message + " :)";

  helloworld_string_dup(&ret->message, message.c_str());
}
