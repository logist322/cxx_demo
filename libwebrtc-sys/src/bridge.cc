#include <memory>
#include <string>

extern "C++" {
    std::string hello_world();
}

namespace bridge {
    std::unique_ptr<std::string> bridge_hello_world() {
        return std::make_unique<std::string>(hello_world());
    }
}
