#ifndef RUMMAGE_H
#define RUMMAGE_H

#include <stdint.h>
#include <stddef.h> 

typedef struct {
    char* response;
    char** email_ids;
    size_t len;
} ChatResponse;

void* rummage_make();
void rummage_refresh(void*);
ChatResponse rummage_prompt(void*, char *);
void rummage_kill_llama(void*);
char* rummage_generate_daily_update(void*);
void rummage_free(void*);

#endif
