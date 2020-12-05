#include "./binaryen/src/binaryen-c.h"


char* BinaryenRustWrite(BinaryenModuleRef module, size_t len) {
    char buffer[len];
    BinaryenModuleWrite(module, buffer, len);
    return buffer;
}