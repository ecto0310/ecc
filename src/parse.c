#include "parse.h"

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "error.h"
#include "parse_util.h"
#include "tokenize.h"

Program *parse(char *source, Token **token) {
  Program *programs = program(source, token);
  return programs;
}

Program *program(char *source, Token **token) {
  Program *program = calloc(1, sizeof(Program));

  Node node = {next : NULL};
  Node *node_tail = &node;

  Variable *variable = NULL;

  while (is_next_token(token)) {
    node_tail->next = statement(source, token, &variable);
    node_tail = node_tail->next;
  }

  int offset = 0;
  for (Variable *now = variable; now != NULL; now = now->next, offset += 8) {
    now->offset = offset;
  }

  program->node = node.next;
  program->variable = variable;
  program->stack_size = offset;
  return program;
}

Node *statement(char *source, Token **token, Variable **variable) {
  Node *node = expr(source, token, variable);
  expect_char(source, token, ";");
  return node;
}

Node *expr(char *source, Token **token, Variable **variable) {
  Node *node = assign(source, token, variable);
  return node;
}

Node *assign(char *source, Token **token, Variable **variable) {
  Node *node = equality(source, token, variable);
  if (consume_char(token, "=") != NULL) {
    node = new_node(ND_ASSIGN, node, assign(source, token, variable));
  }
  return node;
}

Node *equality(char *source, Token **token, Variable **variable) {
  Node *node = relational(source, token, variable);

  while (is_next_token(token)) {
    if (consume_char(token, "==") != NULL) {
      node = new_node(ND_EQ, node, relational(source, token, variable));
    } else if (consume_char(token, "!=") != NULL) {
      node = new_node(ND_NE, node, relational(source, token, variable));
    } else {
      break;
    }
  }
  return node;
}

Node *relational(char *source, Token **token, Variable **variable) {
  Node *node = add(source, token, variable);

  while (is_next_token(token)) {
    if (consume_char(token, "<") != NULL) {
      node = new_node(ND_LT, node, add(source, token, variable));
    } else if (consume_char(token, "<=") != NULL) {
      node = new_node(ND_LE, node, add(source, token, variable));
    } else if (consume_char(token, ">") != NULL) {
      node = new_node(ND_LT, add(source, token, variable), node);
    } else if (consume_char(token, ">=") != NULL) {
      node = new_node(ND_LE, add(source, token, variable), node);
    } else {
      break;
    }
  }
  return node;
}

Node *add(char *source, Token **token, Variable **variable) {
  Node *node = mul(source, token, variable);

  while (is_next_token(token)) {
    if (consume_char(token, "+") != NULL) {
      node = new_node(ND_ADD, node, mul(source, token, variable));
    } else if (consume_char(token, "-") != NULL) {
      node = new_node(ND_SUB, node, mul(source, token, variable));
    } else {
      break;
    }
  }
  return node;
}

Node *mul(char *source, Token **token, Variable **variable) {
  Node *node = unary(source, token, variable);

  while (is_next_token(token)) {
    if (consume_char(token, "*") != NULL) {
      node = new_node(ND_MUL, node, unary(source, token, variable));
    } else if (consume_char(token, "/") != NULL) {
      node = new_node(ND_DIV, node, unary(source, token, variable));
    } else {
      break;
    }
  }
  return node;
}

Node *unary(char *source, Token **token, Variable **variable) {
  if (consume_char(token, "+") != NULL) {
    return primary(source, token, variable);
  }
  if (consume_char(token, "-") != NULL) {
    return new_node(ND_SUB, new_node_number(0), unary(source, token, variable));
  }
  return primary(source, token, variable);
}

Node *primary(char *source, Token **token, Variable **variable) {
  if (consume_char(token, "(") != NULL) {
    Node *node = expr(source, token, variable);
    expect_char(source, token, ")");
    return node;
  }
  Token *now = consume_id(source, token);
  if (now != NULL) {
    Variable *target_variable = find_variable(now, variable);
    if (target_variable == NULL) {
      target_variable = push_variable(now, variable);
    }
    return new_node_variable(target_variable);
  }
  now = expect_number(source, token);
  return new_node_number(now->val);
}
