// Rust's alternative to:
// let v: Vec<_> = v.iter().filter(|&x| !is_odd(x)).collect();

#include <stdlib.h>
#include <stdbool.h>
#include <stddef.h>

// Assume this exists, matching your Rust predicate
static inline bool is_odd(int x) { return (x & 1) != 0; }

/**
 * Filters `in` of length `n`, keeping elements where !is_odd(x).
 * Returns a newly-allocated array in *out with length *out_n.
 * Caller must free(*out).
 */
bool filter_not_odd_new(const int *in, size_t n, int **out, size_t *out_n) {
    if (!out || !out_n) return false;

    // Pass 1: count how many will be kept
    size_t keep = 0;
    for (size_t i = 0; i < n; ++i) {
        if (!is_odd(in[i])) keep++;
    }

    int *buf = malloc(keep ? keep * sizeof *buf : 1); // malloc(0) is implementation-defined
    if (!buf && keep) return false;

    // Pass 2: copy the kept elements
    size_t j = 0;
    for (size_t i = 0; i < n; ++i) {
        if (!is_odd(in[i])) buf[j++] = in[i];
    }

    *out = buf;
    *out_n = keep;
    return true;
}
