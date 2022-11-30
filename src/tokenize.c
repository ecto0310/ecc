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

  for (char *p = source; *p != '\0';) {
    if (isspace(*p)) {
      p++;
      continue;
    }

    if (strchr("+-*/()", *p)) {
      current = new_token(TK_KEYWORD, current, p, 1);
      p += 1;
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

  new_token(TK_EOF, current, NULL, 0);
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
