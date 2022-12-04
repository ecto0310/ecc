#ifndef SRC_PARSE_UTIL_H_
#define SRC_PARSE_UTIL_H_

#include <stdbool.h>

#include "tokenize.h"
#include "parse.h"

bool consume_char(Token **token, char *op);
void expect_char(char *source, Token **token, char *op);
int expect_number(char *source, Token **token);

Node *new_node(NodeKind kind, Node *lhs, Node *rhs);
Node *new_node_number(int value);

bool is_next_token(Token **token);
bool next_token(Token **token);

#endif  // SRC_PARSE_UTIL_H_
