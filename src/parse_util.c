#include "parse_util.h"

#include <stdlib.h>
#include <string.h>

#include "error.h"

bool consume_char(Token **token, char *op) {
  if ((*token)->kind != TK_PUNC || strlen(op) != (*token)->len ||
      memcmp((*token)->str, op, (*token)->len)) {
    return false;
  }
  next_token(token);
  return true;
}

void expect_char(char *source, Token **token, char *op) {
  if ((*token)->kind != TK_PUNC || strlen(op) != (*token)->len ||
      memcmp((*token)->str, op, (*token)->len)) {
    error_at(source, (*token)->str, "'%s'ではありません", op);
  }
  next_token(token);
  return;
}

int expect_number(char *source, Token **token) {
  if ((*token)->kind != TK_NUM) {
    error_at(source, (*token)->str, "数ではありません");
  }
  int value = (*token)->val;
  next_token(token);
  return value;
}

Node *new_node(NodeKind kind, Node *lhs, Node *rhs) {
  Node *node = calloc(1, sizeof(Node));
  node->kind = kind;
  node->lhs = lhs;
  node->rhs = rhs;
  return node;
}

Node *new_node_number(int value) {
  Node *node = calloc(1, sizeof(Node));
  node->kind = ND_NUM;
  node->value = value;
  return node;
}

bool is_next_token(Token **token) { return (*token)->kind != TK_EOF; }

bool next_token(Token **token) {
  if (!is_next_token(token)) {
    return false;
  }
  *token = (*token)->next;
  return true;
}
