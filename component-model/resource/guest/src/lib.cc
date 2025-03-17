#include <iostream>
#include <string>

#include "helloworld.h"

void exports_sammyne_helloworld_greeter_say_hello(
    exports_sammyne_helloworld_greeter_borrow_context_t ctx,
    exports_sammyne_helloworld_greeter_hello_request_t *req,
    exports_sammyne_helloworld_greeter_hello_reply_t *ret) {
  auto request_id = sammyne_helloworld_types_method_context_request_id(ctx);
  printf("request-id = %lld\n", request_id);

  std::string message((const char *)(req->name.ptr), req->name.len);

  message = std::string("Hello ") + message + std::string(" with request-id=") +
            std::to_string(request_id);

  helloworld_string_dup(&ret->message, message.c_str());
}