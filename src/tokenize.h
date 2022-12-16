#ifndef SRC_TOKENIZE_H_
#define SRC_TOKENIZE_H_

#include <stdbool.h>

typedef enum {
  TK_PUNC,  // punctuator
  TK_ID,    // identifier
  TK_NUM,   // number
  TK_EOF,   // end of file
} TokenKind;

typedef struct Token Token;
struct Token {
  TokenKind kind;
  Token *next;
  int val;
  char *str;
  int len;
};

Token *tokenize(char *source);
Token *new_token(TokenKind kind, Token *cur, char *str, int len);
bool startswith(char *p, char *q);

#endif  // SRC_TOKENIZE_H_
