#include "code_generate.h"
#include "file.h"
#include "parse.h"
#include "tokenize.h"

int main(int argc, char **argv) {
  char *input_file = argv[1];

  char *source = read_file(input_file);

  Token *token = tokenize(source);

  Node *node = parse(source, &token);

  code_generate(argv[2], node);

  return 0;
}
