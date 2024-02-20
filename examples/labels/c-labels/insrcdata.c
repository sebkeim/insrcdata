// generated by insrcdata version 0.3.0

#include "insrcdata.h"
#include <string.h>
#include <stdio.h>
#include <stdlib.h>

const label_t LABEL_TABLE[LABEL_TABLE_COUNT] = {
   {"FOO", },
   {"BAR", },
   {"UpperCamelCase", },
   {"lowerCamelCase", },
   {"snake_case", },
   {"kebab-case", },
   {"SHOUTY_SNAKE_CASE", },
   {"Title Case", },
   {"SHOUTY-KEBAB-CASE", },
   {"Train-Case", },
};

const label_t* label_from_labels(labels_t label) {
    return &LABEL_TABLE[label];
}
labels_t label_labels(const label_t *s) {
    return (labels_t)(s-LABEL_TABLE);
}

