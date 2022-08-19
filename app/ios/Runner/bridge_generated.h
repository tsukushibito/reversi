#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define BOARD_SIZE 8

typedef struct wire_int_32_list {
  int32_t *ptr;
  int32_t len;
} wire_int_32_list;

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

void wire_search_game_tree(int64_t port_,
                           struct wire_int_32_list *squares,
                           int32_t turn_depth,
                           int32_t color,
                           int32_t search_depth);

struct wire_int_32_list *new_int_32_list_0(int32_t len);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_search_game_tree);
    dummy_var ^= ((int64_t) (void*) new_int_32_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}