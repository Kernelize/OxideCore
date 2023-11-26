#ifndef __ULIBC_H
#define __ULIBC_H

extern int csyscall(void* fd, void* a1, void* a2, void* a3);

int write(int fd, const void *buf, int count);
int exit(int xstate) __attribute__((noreturn));

#endif
