#ifndef SRC_PARSE_H_
#define SRC_PARSE_H_

#include <stdbool.h>

#include "tokenize.h"

typedef enum NodeKind NodeKind;
typedef struct Node Node;
typedef struct Variable Variable;
typedef struct Program Program;

enum NodeKind {
  ND_ADD,     // +
  ND_SUB,     // -
  ND_MUL,     // *
  ND_DIV,     // /
  ND_EQ,      // ==
  ND_NE,      // !=
  ND_LT,      // <
  ND_LE,      // <=
  ND_ASSIGN,  // =
  ND_VAR,     // Variable
  ND_NUM,     // Integer
};

struct Node {
  NodeKind kind;
  Node *next;
  Node *lhs;
  Node *rhs;
  int value;
  Variable *variable;
};

struct Variable {
  Variable *next;
  char *name;
  int offset;
};

struct Program {
  Node *node;
  Variable *variable;
  int stack_size;
};

Program *parse(char *source, Token **token);
Program *program(char *source, Token **token);
Node *statement(char *source, Token **token, Variable **variable);
Node *expr(char *source, Token **token, Variable **variable);
Node *assign(char *source, Token **token, Variable **variable);
Node *equality(char *source, Token **token, Variable **variable);
Node *relational(char *source, Token **token, Variable **variable);
Node *add(char *source, Token **token, Variable **variable);
Node *mul(char *source, Token **token, Variable **variable);
Node *unary(char *source, Token **token, Variable **variable);
Node *primary(char *source, Token **token, Variable **variable);

#endif  // SRC_PARSE_H_
