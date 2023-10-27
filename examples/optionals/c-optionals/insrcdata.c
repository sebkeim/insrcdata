// generated by insrcdata version 0.2.0

#include "insrcdata.h"
#include <string.h>
#include <stdio.h>
#include <stdlib.h>

static unsigned const ADHOC_TABLE_COUNT = 2;
static const adhoc_t ADHOC_TABLE[ADHOC_TABLE_COUNT] = {
   {100.0, 42, },
   {-1.0, 0, },
};

const adhoc_t* adhoc_from_adhocs(adhocs_t label) {
    return &ADHOC_TABLE[label];
}
adhocs_t adhoc_adhocs(const adhoc_t *s) {
    return (adhocs_t)(s-ADHOC_TABLE);
}

static unsigned const SCORE_TABLE_COUNT = 1;
static const score_t SCORE_TABLE[SCORE_TABLE_COUNT] = {
   {100.0, },
};

static unsigned const COUNT_TABLE_COUNT = 1;
static const count_t COUNT_TABLE[COUNT_TABLE_COUNT] = {
   {42, },
};

static unsigned const OPTJOIN_TABLE_COUNT = 2;
static const optjoin_t OPTJOIN_TABLE[OPTJOIN_TABLE_COUNT] = {
   {1, 1, },
   {0, 0, },
};

const optjoin_t* optjoin_from_optjoins(optjoins_t label) {
    return &OPTJOIN_TABLE[label];
}
optjoins_t optjoin_optjoins(const optjoin_t *s) {
    return (optjoins_t)(s-OPTJOIN_TABLE);
}

bool optjoin_score_join(const optjoin_t* s, const score_t** ptr) {
    if( s->score_join_) {
        *ptr = &SCORE_TABLE[s->score_join_-1];
        return true;
    }
    return false;
}
bool optjoin_count_join(const optjoin_t* s, const count_t** ptr) {
    if( s->count_join_) {
        *ptr = &COUNT_TABLE[s->count_join_-1];
        return true;
    }
    return false;
}
