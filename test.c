#include <stdio.h>
#include "rummage-lib/rummage.h"

int main() {
    void* ptr = rummage_make();
    
    rummage_refresh(ptr);
    ChatResponse res = rummage_prompt(ptr, "C is cool");

    printf("recieved response: %s\n", res.response);

    printf("recieved email ids: ");
    for (int i = 0; i < res.len; i++) {
        printf("%s ", res.email_ids[i]);
    }
    printf("\n");
    printf("daily update: %s\n", rummage_generate_daily_update(ptr));
    
    rummage_kill_llama(ptr);
    rummage_free(ptr);
    ptr = NULL;
}