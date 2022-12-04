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

  fprintf(fp, "\tpush rbp\n");
  fprintf(fp, "\tmov rbp, rsp\n");
  fprintf(fp, "\tsub rsp, 208\n");

  for (; node != NULL; node = node->next) {
    generate(fp, node);
    fprintf(fp, "\tpop rax\n\n");
  }

  fprintf(fp, "\tmov rsp, rbp\n");
  fprintf(fp, "\tpop rbp\n");
  fprintf(fp, "\tret\n");
}

void generate(FILE *fp, Node *node) {
  switch (node->kind) {
    case ND_NUM:
      fprintf(fp, "\tpush %d\n", node->value);
      return;
    case ND_LVAR:
      generate_lval(fp, node);
      fprintf(fp, "\tpop rax\n");
      fprintf(fp, "\tmov rax, [rax]\n");
      fprintf(fp, "\tpush rax\n");
      return;
    case ND_ASSIGN:
      generate_lval(fp, node->lhs);
      generate(fp, node->rhs);

      fprintf(fp, "\tpop rdi\n");
      fprintf(fp, "\tpop rax\n");
      fprintf(fp, "\tmov [rax], rdi\n");
      fprintf(fp, "\tpush rdi\n");
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
    case ND_EQ:
      fprintf(fp, "\tcmp rax, rdi\n");
      fprintf(fp, "\tsete al\n");
      fprintf(fp, "\tmovzb rax, al\n");
      break;
    case ND_NE:
      fprintf(fp, "\tcmp rax, rdi\n");
      fprintf(fp, "\tsetne al\n");
      fprintf(fp, "\tmovzb rax, al\n");
      break;
    case ND_LT:
      fprintf(fp, "\tcmp rax, rdi\n");
      fprintf(fp, "\tsetl al\n");
      fprintf(fp, "\tmovzb rax, al\n");
      break;
    case ND_LE:
      fprintf(fp, "\tcmp rax, rdi\n");
      fprintf(fp, "\tsetle al\n");
      fprintf(fp, "\tmovzb rax, al\n");
  }

  fprintf(fp, "\tpush rax\n");
}

void generate_lval(FILE *fp, Node *node) {
  if (node->kind != ND_LVAR) {
    error("代入の左辺値が変数ではありません");
  }

  fprintf(fp, "\tmov rax, rbp\n");
  fprintf(fp, "\tsub rax, %d\n", node->offset);
  fprintf(fp, "\tpush rax\n");
}
