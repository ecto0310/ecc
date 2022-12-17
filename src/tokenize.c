#include "tokenize.h"

#include <ctype.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

#include "error.h"

Token *tokenize(char *source) {
  Token head = {next : NULL};
  Token *current = &head;

  char *p = source;
  for (; *p != '\0';) {
    if (isspace(*p)) {
      p++;
      continue;
    }

    if (startswith(p, "==") || startswith(p, "!=") || startswith(p, "<=") ||
        startswith(p, ">=")) {
      current = new_token(TK_PUNC, current, p, 2);
      p += 2;
      continue;
    }

    if (strchr("+-*/()<>=;", *p)) {
      current = new_token(TK_PUNC, current, p, 1);
      p += 1;
      continue;
    }

    if (isalpha(*p)) {
      char *p_tmp = p;
      p += 1;
      while (isalnum(*p)) p += 1;
      current = new_token(TK_ID, current, p_tmp, p - p_tmp);
      continue;
    }

    if (isdigit(*p)) {
      current = new_token(TK_NUM, current, p, 0);
      char *p_tmp = p;
      current->val = strtol(p, &p, 10);
      current->len = p - p_tmp;
      continue;
    }

    error_at(source, p, "invalid token");
  }

  new_token(TK_EOF, current, p, 0);
  return head.next;
}

Token *new_token(TokenKind kind, Token *current, char *str, int len) {
  Token *token = calloc(1, sizeof(Token));
  token->kind = kind;
  token->str = str;
  token->len = len;
  current->next = token;
  return token;
}

bool startswith(char *p, char *q) { return strncmp(p, q, strlen(q)) == 0; }
