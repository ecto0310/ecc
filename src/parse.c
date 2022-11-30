#include "parse.h"

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "error.h"
#include "tokenize.h"

Node *parse(char *source, Token **token) {
  Node *node = expr(source, token);
  if (is_next_token(token)) {
    error_at(source, (*token)->str, "不明なトークンです");
  }
  return node;
}

bool consume_char(Token **token, char *op) {
  if ((*token)->kind != TK_KEYWORD || strlen(op) != (*token)->len ||
      memcmp((*token)->str, op, (*token)->len)) {
    return false;
  }
  next_token(token);
  return true;
}

void expect_char(char *source, Token **token, char *op) {
  if ((*token)->kind != TK_KEYWORD || strlen(op) != (*token)->len ||
      memcmp((*token)->str, op, (*token)->len)) {
    error_at(source, (*token)->str, "'%c'ではありません", op);
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


Node *expr(char *source, Token **token) {
  Node *node = equality(source, token);
  return node;
}

Node *equality(char *source, Token **token) {
  Node *node = relational(source, token);

  while (is_next_token(token)) {
    if (consume_char(token, "==")) {
      node = new_node(ND_EQ, node, relational(source, token));
    } else if (consume_char(token, "!=")) {
      node = new_node(ND_NE, node, relational(source, token));
    } else {
      break;
    }
  }
  return node;
}

Node *relational(char *source, Token **token) {
  Node *node = add(source, token);

  while (is_next_token(token)) {
    if (consume_char(token, "<")) {
      node = new_node(ND_LT, node, add(source, token));
    } else if (consume_char(token, "<=")) {
      node = new_node(ND_LE, node, add(source, token));
    } else if (consume_char(token, ">")) {
      node = new_node(ND_LT, add(source, token), node);
    } else if (consume_char(token, ">=")) {
      node = new_node(ND_LE, add(source, token), node);
    } else {
      break;
    }
  }
  return node;
}

Node *add(char *source, Token **token) {
  Node *node = mul(source, token);

  while (is_next_token(token)) {
    if (consume_char(token, "+")) {
      node = new_node(ND_ADD, node, mul(source, token));
    } else if (consume_char(token, "-")) {
      node = new_node(ND_SUB, node, mul(source, token));
    } else {
      break;
    }
  }
  return node;
}

Node *mul(char *source, Token **token) {
  Node *node = unary(source, token);

  while (is_next_token(token)) {
    if (consume_char(token, "*")) {
      node = new_node(ND_MUL, node, unary(source, token));
    } else if (consume_char(token, "/")) {
      node = new_node(ND_DIV, node, unary(source, token));
    } else {
      break;
    }
  }
  return node;
}

Node *unary(char *source, Token **token) {
  if (consume_char(token, "+")) {
    return primary(source, token);
  }
  if (consume_char(token, "-")) {
    return new_node(ND_SUB, new_node_number(0), unary(source, token));
  }
  return primary(source, token);
}

Node *primary(char *source, Token **token) {
  if (consume_char(token, "(")) {
    Node *node = expr(source, token);
    expect_char(source, token, ")");
    return node;
  }
  return new_node_number(expect_number(source, token));
}
