#include "code_generate.h"

#include <errno.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "error.h"

void code_generate(char *path, Node *node) {
  FILE *fp = fopen(path, "w");
  if (fp == NULL) {
    error("Can't open %s: %s", path, strerror(errno));
  }
  fprintf(fp, ".intel_syntax noprefix\n");
  fprintf(fp, ".globl main\n");
  fprintf(fp, "main:\n");

  generate(fp, node);

  fprintf(fp, "\tpop rax\n");
  fprintf(fp, "\tret\n");
}

void generate(FILE *fp, Node *node) {
  if (node->kind == ND_NUM) {
    fprintf(fp, "\tpush %d\n", node->value);
    return;
  }

  generate(fp, node->lhs);
  generate(fp, node->rhs);

  fprintf(fp, "\tpop rdi\n");
  fprintf(fp, "\tpop rax\n");

  switch (node->kind) {
    case ND_ADD:
      fprintf(fp, "\tadd rax, rdi\n");
      break;
    case ND_SUB:
      fprintf(fp, "\tsub rax, rdi\n");
      break;
    case ND_MUL:
      fprintf(fp, "\timul rax, rdi\n");
      break;
    case ND_DIV:
      fprintf(fp, "\tcqo\n");
      fprintf(fp, "\tidiv rdi\n");
      break;
  }

  fprintf(fp, "\tpush rax\n");
}
