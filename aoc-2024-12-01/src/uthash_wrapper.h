#pragma once
#include "uthash.h"
#include <stdint.h>
#include <stdlib.h>

// Hash table entry structure for uthash
struct hash_entry {
    int32_t key;        // the key (number from input)
    int32_t count;      // frequency count
    UT_hash_handle hh;  // makes this structure hashable
};

// Create and populate a hash table from an array
struct hash_entry* uthash_build_frequency_map(const int32_t* arr, size_t len);

// Look up a key in the hash table and return its count
int32_t uthash_lookup(struct hash_entry *hash_table, int32_t key);

// Free the entire hash table
void uthash_destroy(struct hash_entry *hash_table);
