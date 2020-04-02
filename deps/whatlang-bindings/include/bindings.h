#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct CInfo {
  int32_t lang;
  double confidence;
};

extern "C" {

uint8_t detect(const char *ptr, CInfo *cinfo);

} // extern "C"
