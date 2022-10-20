#include "error.h"

#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

void error_at(char *source, char *location, char *message, ...) {
  va_list ap;
  va_start(ap, message);

  char *line_begin = location;
  while (source < line_begin && line_begin[-1] != '\n') {
    line_begin--;
  }

  char *line_end = location;
  while (*line_end != '\n') {
    line_end++;
  }

  int line = 1;
  for (char *p = source; p < line_begin; p++) {
    if (*p == '\n') {
      line++;
    }
  }

  int indent = fprintf(stderr, "%d: ", line);
  fprintf(stderr, "%.*s\n", (int)(line_end - line_begin), line_begin);

  int position = location - line_begin + indent;
  fprintf(stderr, "%*s", position, "");
  fprintf(stderr, "^ ");
  vfprintf(stderr, message, ap);
  fprintf(stderr, "\n");
  exit(1);
}

void error(char *fmt, ...) {
  va_list ap;
  va_start(ap, fmt);
  vfprintf(stderr, fmt, ap);
  fprintf(stderr, "\n");
  exit(1);
}
