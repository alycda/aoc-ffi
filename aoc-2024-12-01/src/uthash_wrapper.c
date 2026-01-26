#include "uthash_wrapper.h"

struct hash_entry* uthash_build_frequency_map(const int32_t* arr, size_t len) {
    struct hash_entry *hash_table = NULL;

    for (size_t i = 0; i < len; i++) {
        int32_t key = arr[i];
        struct hash_entry *entry = NULL;

        // Look up existing entry
        HASH_FIND_INT(hash_table, &key, entry);

        if (entry) {
            // Key exists, increment count
            entry->count++;
        } else {
            // Key doesn't exist, create new entry
            entry = (struct hash_entry*)malloc(sizeof(struct hash_entry));
            entry->key = key;
            entry->count = 1;
            HASH_ADD_INT(hash_table, key, entry);
        }
    }

    return hash_table;
}

int32_t uthash_lookup(struct hash_entry *hash_table, int32_t key) {
    struct hash_entry *entry = NULL;
    HASH_FIND_INT(hash_table, &key, entry);
    return entry ? entry->count : 0;
}

void uthash_destroy(struct hash_entry *hash_table) {
    struct hash_entry *current, *tmp;
    HASH_ITER(hh, hash_table, current, tmp) {
        HASH_DEL(hash_table, current);
        free(current);
    }
}
