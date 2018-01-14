/* From https://youtu.be/zmtHaZG7pPc?t=22m19s */
struct pyfrac_error {
    char *message;
    int failed;
    int code;
};

void pyfrac_init(void);

char *pyfrac_repeated(const unsigned char * num, unsigned long num_len,
                      const unsigned char * den, unsigned long den_len,
                      unsigned long base, unsigned long min_exp,
                      struct pyfrac_error *);

void pyfrac_free(char *);
