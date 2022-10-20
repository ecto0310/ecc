#include "file.h"

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "error.h"

char *read_file(char *path) {
  FILE *fp = fopen(path, "r");
  if (fp == NULL) {
    error("Can't open %s: %s", path, strerror(errno));
  }

  if (fseek(fp, 0, SEEK_END) == -1) {
    error("Failed to seek %s's end: %s", path, strerror(errno));
  }
  long size = ftell(fp);
  if (fseek(fp, 0, SEEK_SET) == -1) {
    error("Failed to seek %s's begin: %s", path, strerror(errno));
  }

  char *buf = calloc(1, size + 2);
  fread(buf, size, 1, fp);

  if (size == 0 || buf[size - 1] != '\n') {
    buf[size++] = '\n';
  }
  buf[size] = '\0';
  fclose(fp);
  return buf;
}
