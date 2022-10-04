#ifndef SRC_ERROR_H_
#define SRC_ERROR_H_

void error_at(char *source, char *location, char *message, ...);
void error(char *fmt, ...);

#endif  // SRC_ERROR_H_
