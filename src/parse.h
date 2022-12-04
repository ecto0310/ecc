#ifndef SRC_PARSE_H_
#define SRC_PARSE_H_

#include <stdbool.h>

#include "tokenize.h"

typedef enum {
  ND_ADD,  // +
  ND_SUB,  // -
  ND_MUL,  // *
  ND_DIV,  // /
  ND_EQ,   // ==
  ND_NE,   // !=
  ND_LT,   // <
  ND_LE,   // <=
  ND_NUM,  // Integer
} NodeKind;

typedef struct Node Node;
struct Node {
  NodeKind kind;
  Node *next;
  Node *lhs;
  Node *rhs;
  int value;
};

Node *parse(char *source, Token **token);
Node *program(char *source, Token **token);
Node *statement(char *source, Token **token);
Node *expr(char *source, Token **token);
Node *equality(char *source, Token **token);
Node *relational(char *source, Token **token);
Node *add(char *source, Token **token);
Node *mul(char *source, Token **token);
Node *unary(char *source, Token **token);
Node *primary(char *source, Token **token);

#endif  // SRC_PARSE_H_
