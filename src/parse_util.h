#ifndef SRC_PARSE_UTIL_H_
#define SRC_PARSE_UTIL_H_

#include <stdbool.h>

#include "parse.h"
#include "tokenize.h"

Token *consume_char(Token **token, char *op);
Token *expect_char(char *source, Token **token, char *op);
Token *expect_number(char *source, Token **token);
Token *consume_id(char *source, Token **token);

Node *new_node(NodeKind kind, Node *lhs, Node *rhs);
Node *new_node_number(int value);
Node *new_node_id(char *id);

bool is_next_token(Token **token);
bool next_token(Token **token);

#endif  // SRC_PARSE_UTIL_H_
