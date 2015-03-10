#include <stdlib.h>
#include <regex.h>
#include <stdio.h>
#include <string.h>
#include <errno.h>

void scan_file(char *path, regex_t *re_ptr) {
  int i;

  size_t linebuf_size;
  ssize_t linebuf_read_size;
  char *linebuf;
  FILE *file;

  file = fopen(path, "r");
  if (file == NULL) {
    perror("fopen()");
    return;
  }

  linebuf_size = 0;
  linebuf = NULL;
  linebuf_read_size = 0;
  do {
    if (linebuf_read_size == 0) {
      continue;
    }
    if ((regexec(re_ptr, linebuf, 0, NULL, 0) == 0)) {
      printf("%s", linebuf);
    }
  } while ((linebuf_read_size = getline(&linebuf, &linebuf_size, file)) > 0);

  if (linebuf_read_size < 0 && errno != 0) {
    perror("getline()");
  }

  free(linebuf);
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
    fprintf(stderr, "regex(): %s\n", errbuf);
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

