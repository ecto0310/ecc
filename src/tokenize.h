#ifndef SRC_TOKENIZE_H_
#define SRC_TOKENIZE_H_

typedef enum {
  TK_KEYWORD,
  TK_NUM,
  TK_EOF,
} TokenKind;

typedef struct Token Token;
struct Token {
  TokenKind kind;
  Token *next;
  int val;
  char *str;
};

Token *tokenize(char *source);
Token *new_token(TokenKind kind, Token *cur, char *str);

#endif  // SRC_TOKENIZE_H_
