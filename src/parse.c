#include "parse.h"

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "error.h"
#include "parse_util.h"
#include "tokenize.h"

Node *parse(char *source, Token **token) {
  Node *node = program(source, token);
  return node;
}

Node *program(char *source, Token **token) {
  Node head = {next : NULL};
  Node *current = &head;

  while (is_next_token(token)) {
    current->next = statement(source, token);
    current = current->next;
  }

  return head.next;
}

Node *statement(char *source, Token **token) {
  Node *node = expr(source, token);
  expect_char(source, token, ";");
  return node;
}

Node *expr(char *source, Token **token) {
  Node *node = assign(source, token);
  return node;
}

Node *assign(char *source, Token **token) {
  Node *node = equality(source, token);
  if (consume_char(token, "=") != NULL) {
    node = new_node(ND_ASSIGN, node, assign(source, token));
  }
  return node;
}

Node *equality(char *source, Token **token) {
  Node *node = relational(source, token);

  while (is_next_token(token)) {
    if (consume_char(token, "==") != NULL) {
      node = new_node(ND_EQ, node, relational(source, token));
    } else if (consume_char(token, "!=") != NULL) {
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
    if (consume_char(token, "<") != NULL) {
      node = new_node(ND_LT, node, add(source, token));
    } else if (consume_char(token, "<=") != NULL) {
      node = new_node(ND_LE, node, add(source, token));
    } else if (consume_char(token, ">") != NULL) {
      node = new_node(ND_LT, add(source, token), node);
    } else if (consume_char(token, ">=") != NULL) {
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
    if (consume_char(token, "+") != NULL) {
      node = new_node(ND_ADD, node, mul(source, token));
    } else if (consume_char(token, "-") != NULL) {
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
    if (consume_char(token, "*") != NULL) {
      node = new_node(ND_MUL, node, unary(source, token));
    } else if (consume_char(token, "/") != NULL) {
      node = new_node(ND_DIV, node, unary(source, token));
    } else {
      break;
    }
  }
  return node;
}

Node *unary(char *source, Token **token) {
  if (consume_char(token, "+") != NULL) {
    return primary(source, token);
  }
  if (consume_char(token, "-") != NULL) {
    return new_node(ND_SUB, new_node_number(0), unary(source, token));
  }
  return primary(source, token);
}

Node *primary(char *source, Token **token) {
  if (consume_char(token, "(") != NULL) {
    Node *node = expr(source, token);
    expect_char(source, token, ")");
    return node;
  }
  Token *now = consume_id(source, token);
  if (now != NULL) {
    return new_node_id(now->str);
  }
  now = expect_number(source, token);
  return new_node_number(now->val);
}
