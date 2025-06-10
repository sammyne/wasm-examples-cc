#include "helloworld.h"

#include <iostream>
#include <string>

// ref: https://github.com/grpc/grpc-go/blob/master/examples/helloworld/greeter_server/main.go#L43
void exports_sammyne_helloworld_greeter_say_hello(exports_sammyne_helloworld_greeter_hello_request_t *req, exports_sammyne_helloworld_greeter_hello_reply_t *ret) {
  std::string message((const char*)(req->name.ptr), req->name.len);

  message = std::string("Hello ") + message;

  // char* p = (char*) malloc(1024*1024*1024);
  // if(p)

  helloworld_string_dup(&ret->message, message.c_str());
}
