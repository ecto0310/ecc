#ifndef SRC_CODE_GENERATE_H_
#define SRC_CODE_GENERATE_H_

#include <stdio.h>

#include "parse.h"

void code_generate(char *path, Node *node);
void generate(FILE *fp, Node *node);
void generate_lval(FILE *fp, Node *node);

#endif  // SRC_CODE_GENERATE_H_
