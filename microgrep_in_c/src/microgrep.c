#include <stdlib.h>
#include <regex.h>
#include <stdio.h>
#include <string.h>
#include <errno.h>

void scan_file(char *path, regex_t *re_ptr) {
  int i;

  size_t filebuf_size, line_size;
  char *filebuf, *line;
  char curchar;
  unsigned int line_i, read_count, last_newline_i;
  FILE *file;

  file = fopen(path, "r");
  if (file == NULL) {
    fprintf(stderr, "ERROR(fopen): %s\n", strerror(errno));
    return;
  }
  filebuf_size = line_size = 512;
  filebuf = malloc(filebuf_size);
  if (filebuf == NULL) {
    fprintf(stderr, "ERROR(malloc): no memory for filebuf (%zd bytes)\n", filebuf_size);
    return;
  }
  line = malloc(line_size);
  if (line == NULL) {
    fprintf(stderr, "ERROR(malloc): no memory for line (%zd bytes)\n", line_size);
    return;
  }
  line_i = 0;
  last_newline_i = 0;

  while ((read_count = fread(filebuf, sizeof *filebuf, filebuf_size, file))) {
    for (i = 0; i < read_count; i++) {
      curchar = filebuf[i];
      if (curchar == '\n') {
        line[line_i] = '\0';
        if ((regexec(re_ptr, line, 0, NULL, 0) == 0)) {
          printf("%s\n", line);
        }
        line_i = 0;
      } else {
        if (line_i == line_size) {
          line_size *= 2;
          if ((line = realloc(line, line_size)) == NULL) {
            fprintf(stderr, "ERROR(realloc): no memory for line (%zd bytes)\n", line_size);
            return;
          }
        }
        line[line_i++] = curchar;
      }
    }
  }

  free(line);
  free(filebuf);
}

void teardown_re(regex_t *re_ptr) {
  regfree(re_ptr);
}

regex_t *setup_re() {
  regex_t *re_ptr;
  int retcode;
  size_t errbuf_size;
  char *errbuf;

  re_ptr = malloc(sizeof *re_ptr);
  retcode = regcomp(re_ptr, "[0-9][0-9]\\.[0-9]ms)", REG_NOSUB);
  if (retcode != 0) {
    errbuf_size = regerror(retcode, re_ptr, NULL, 0);
    errbuf = malloc(errbuf_size);
    regerror(retcode, re_ptr, errbuf, errbuf_size);
    fprintf(stderr, "ERROR(regex): %s\n", errbuf);
    free(errbuf);
    teardown_re(re_ptr);
    return NULL;
  }

  return re_ptr;
}

int main(int argc, char **argv) {
  int i;
  regex_t *re_ptr;

  re_ptr = setup_re();
  for (i = 1; i < argc; i++) {
    scan_file(argv[i], re_ptr);
  }
  teardown_re(re_ptr);
}

