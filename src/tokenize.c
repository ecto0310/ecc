#include "tokenize.h"

#include <ctype.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

#include "error.h"

Token *tokenize(char *source) {
  Token head = {next : NULL};
  Token *current = &head;

  for (char *p = source; *p != '\0'; p++) {
    if (isspace(*p)) {
      continue;
    }

    if (strchr("+-*/()", *p)) {
      current = new_token(TK_KEYWORD, current, p);
      continue;
    }

    if (isdigit(*p)) {
      current = new_token(TK_NUM, current, p);
      current->val = strtol(p, &p, 10);
      p--;
      continue;
    }

    error_at(source, p, "invalid token");
  }

  new_token(TK_EOF, current, NULL);
  return head.next;
}

Token *new_token(TokenKind kind, Token *current, char *str) {
  Token *token = calloc(1, sizeof(Token));
  token->kind = kind;
  token->str = str;
  current->next = token;
  return token;
}
