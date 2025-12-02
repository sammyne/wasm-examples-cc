#include "helloworld.h"

#include <iostream>
#include <string>

void exports_sammyne_helloworld_greeter_say_hello(sammyne_helloworld_types_hello_request_t *req, sammyne_helloworld_types_hello_reply_t *ret) {
  // std::string message((const char*)(req->name.ptr), req->name.len);

  // message = std::string("Hello again ") + message;
  sammyne_hellohost_api_new_greeting(&req->name, &ret->message);
  // helloworld_string_dup(&ret->message, message.c_str());
}
