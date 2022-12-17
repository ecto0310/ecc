#include "parse_util.h"

#include <stdlib.h>
#include <string.h>

#include "error.h"

Token *consume_char(Token **token, char *op) {
  Token *now = *token;
  if (now->kind != TK_PUNC || strlen(op) != now->len ||
      strncmp(now->str, op, now->len)) {
    return NULL;
  }
  next_token(token);
  return now;
}

Token *expect_char(char *source, Token **token, char *op) {
  Token *now = *token;
  if (now->kind != TK_PUNC || strlen(op) != now->len ||
      strncmp(now->str, op, now->len)) {
    error_at(source, now->str, "'%s'ではありません", op);
  }
  next_token(token);
  return now;
}

Token *expect_number(char *source, Token **token) {
  Token *now = *token;
  if (now->kind != TK_NUM) {
    error_at(source, now->str, "数ではありません");
  }
  next_token(token);
  return now;
}

Token *consume_id(char *source, Token **token) {
  Token *now = *token;
  if (now->kind != TK_ID) {
    return NULL;
  }
  next_token(token);
  return now;
}

Node *new_node(NodeKind kind, Node *lhs, Node *rhs) {
  Node *node = calloc(1, sizeof(Node));
  node->kind = kind;
  node->next = NULL;
  node->lhs = lhs;
  node->rhs = rhs;
  return node;
}

Node *new_node_number(int value) {
  Node *node = calloc(1, sizeof(Node));
  node->kind = ND_NUM;
  node->next = NULL;
  node->value = value;
  return node;
}

Node *new_node_variable(Variable *variable) {
  Node *node = calloc(1, sizeof(Node));
  node->kind = ND_VAR;
  node->next = NULL;
  node->variable = variable;
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

Variable *find_variable(Token *token, Variable **variable) {
  for (Variable *now = (*variable); now != NULL; now = now->next) {
    if (strlen(now->name) == token->len &&
        !memcmp(token->str, now->name, token->len))
      return now;
  }
  return NULL;
}

Variable *push_variable(Token *token, Variable **variable) {
  Variable *new_variable = calloc(1, sizeof(Variable));
  new_variable->next = (*variable);
  new_variable->name = strndup(token->str, token->len);
  *variable = new_variable;
  return new_variable;
}
