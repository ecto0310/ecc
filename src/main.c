#include "file.h"
#include "parse.h"
#include "tokenize.h"

int main(int argc, char **argv) {
  char *input_file = argv[1];

  char *source = read_file(input_file);

  Token *token = tokenize(source);

  Node *node = parse(source, &token);

  return 0;
}
