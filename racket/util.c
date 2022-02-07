#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include "chezscheme.h"
#include "racketcs.h"
#include "util.h"

void default_boot(const char* exec_file,
                  const char* boot1_path,
                  const char* boot2_path,
                  const char* boot3_path) {
  racket_boot_arguments_t ba;
  puts("Hello!\n\n");

  memset(&ba, 0, sizeof ba);

  ba.boot1_path = boot1_path;
  ba.boot2_path = boot2_path;
  ba.boot3_path = boot3_path;
  ba.exec_file = exec_file;

  ba.collects_dir = "/Applications/Racket v8.3/collects\0/Users/qt/Library/Racket/8.3/collects\0";
  ba.config_dir = "/Applications/Racket v8.3/etc";
  racket_boot(&ba);
}


void run_something(const char* image) {
  racket_embedded_load_file(image, 1);
  racket_namespace_require(Sstring_to_symbol("racket"));
  static char buffer[500];

  memset(buffer, 0, sizeof buffer);

  ptr mod = Scons(Sstring_to_symbol("quote"),
                  Scons(Sstring_to_symbol("evaluator"),
                        Snil));
  ptr eval_sym = Sstring_to_symbol("try-eval");
  ptr try_eval = Scar(racket_dynamic_require(mod, eval_sym));
  /* racket_apply(racket_primitive("disable-interrupts"), Snil); */
  while (fgets(buffer, sizeof buffer, stdin)) {
    Slock_object(try_eval);
    ptr result = racket_apply(try_eval, Scons(Sstring(buffer), Snil));
    Sunlock_object(try_eval);
    ptr out = Scar(result);
    ptr data = Sbytevector_data(out);
    long length = Sbytevector_length(out);
    for (int i = 0; i < length; ++i) {
      printf("%c", ((char*)data)[i]);
    }
    memset(buffer, 0, sizeof buffer);
  }
}
