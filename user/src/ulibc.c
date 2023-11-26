#include "ulibc.h"
const int SYS_WRITE = 64;
const int SYS_EXIT = 93;


int write(int fd, const void *buf, int count) {
  return csyscall((void*)SYS_WRITE, (void*)fd, (void*)buf, (void*)count);
}

int exit(int xstate) {
  csyscall((void*)SYS_EXIT, (void*)xstate, 0, 0);
}
