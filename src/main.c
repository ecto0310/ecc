#include "file.h"

int main(int argc, char **argv) {
  char *input_file = argv[1];

  char *source = read_file(input_file);

  return 0;
}
