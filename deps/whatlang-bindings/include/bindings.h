#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct CInfo {
  int32_t lang;
  int32_t script;
  double confidence;
};

extern "C" {

int32_t detect(const char *ptr, CInfo *cinfo);

int32_t detect_lang(const char *ptr);

int32_t detect_script(const char *ptr);

} // extern "C"
