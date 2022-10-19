#ifndef SRC_PARSE_H_
#define SRC_PARSE_H_

#include <stdbool.h>

#include "tokenize.h"

typedef enum {
  ND_ADD,
  ND_SUB,
  ND_MUL,
  ND_DIV,
  ND_NUM,
} NodeKind;

typedef struct Node Node;
struct Node {
  NodeKind kind;
  Node *lhs;
  Node *rhs;
  int value;
};

Node *parse(char *source, Token **token);

bool consume_char(Token **token, char op);
void expect_char(char *source, Token **token, char op);
int expect_number(char *source, Token **token);

Node *new_node(NodeKind kind, Node *lhs, Node *rhs);
Node *new_node_number(int value);

bool is_next_token(Token **token);
bool next_token(Token **token);

Node *expr(char *source, Token **token);
Node *mul(char *source, Token **token);
Node *unary(char *source, Token **token);
Node *primary(char *source, Token **token);

#endif  // SRC_PARSE_H_
