#ifndef SRC_CODE_GENERATE_H_
#define SRC_CODE_GENERATE_H_

#include <stdio.h>

#include "parse.h"

void code_generate(char *path, Program *program);
void generate(FILE *fp, Node *node);
void generate_variable(FILE *fp, Node *node);

void load_memory();
void store_memory();

#endif  // SRC_CODE_GENERATE_H_
