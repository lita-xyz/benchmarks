#include "keccak-tiny.h"
typedef unsigned char byte;

#ifdef __VALIDA__
const unsigned EOF = 0xFFFFFFFF;
#else
#include <stdio.h>
#endif

static inline byte read_byte_from_stdin() {
#ifdef __VALIDA__
  return __builtin_valida_read_advice();
#else
  return getc(stdin);
#endif
}

int read_stdin_into(byte *buf, int buflen) {
  unsigned len = 0;
  while (len < buflen) {
    byte c = read_byte_from_stdin();
    if (c == EOF) {
      return len;
    } else {
      buf[len] = c;
      len++;
    }
  }
  return -1;
}

static inline void write_char_to_stdout(char c) {
#ifdef __VALIDA__
  __builtin_valida_write(c);
#else
  putc(c, stdout);
#endif
}

static void dump32_to_hex(byte *buf) {
  const char hex_chars[] = "0123456789abcdef";
  write_char_to_stdout('0');
  write_char_to_stdout('x');
  for (unsigned i = 0; i < 32; i++) {
    unsigned bi = (unsigned)buf[i];
    write_char_to_stdout(hex_chars[(bi >> 4) & 0xF]);
    write_char_to_stdout(hex_chars[bi & 0xF]);
  }
  write_char_to_stdout('\n');
}

#define INPUT_MAX_LEN 256
#define DIGEST_SIZE 32

int main() {

  // byte input[INPUT_MAX_LEN];
  // int len = read_stdin_into(input, INPUT_MAX_LEN);
  // if (len == -1) {
  //     return -1;
  // }
  byte input[32] = {5};
  int len = 32;

  byte digest[DIGEST_SIZE];

  keccak_256(digest, DIGEST_SIZE, input, len);

  // // Do 9999 other iterations so that keccak overhead dwarfs initialization
  // and output serialization for (int i = 0; i < 999; i+=1) {
  //     keccak_256(digest, DIGEST_SIZE, digest, DIGEST_SIZE);
  // }

  // Comment out when benchmarking proving, or bench 10k+ keccak applications.
  dump32_to_hex(digest);

  return 0;
}
