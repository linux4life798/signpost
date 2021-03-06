#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <gpio.h>
#include <tock.h>

#include "signbus_io_interface.h"
#include "storage_master.h"

#define BUFFER_SIZE 2048

static uint8_t request_buf[BUFFER_SIZE];
static uint8_t return_buf[BUFFER_SIZE];
static uint8_t message_buf[BUFFER_SIZE];
static uint8_t return_address;

static uint8_t rpc_pending = 0;

#define RPC_REQUEST 0x01
#define RPC_RETURN 0x02

int main (void) {
    signbus_io_init(0x19);
    storage_master_enable_edison();
    gpio_enable_output(2);
    gpio_set(2);

    while(1) {
        uint8_t src;
        int len;
        bool enc;
        int rc;
        len = signbus_io_recv(BUFFER_SIZE, message_buf, &enc, &src);
        if (len < 0) {
            printf(" - Err from signbus_io_recv %d\n", len);
            continue;
        }

        if(message_buf[0] == RPC_REQUEST) {
            //use len > 1 to make sure that someone isn't just setting
            //up the correct buffer
            if(!rpc_pending && len > 1) {
                gpio_clear(2);
                rpc_pending = 1;
                memcpy(request_buf, message_buf+1,len-1);
                rc = signbus_io_set_read_buffer(request_buf,len-1);
                if (rc < 0) {
                    printf(" - signbus_io_read_buffer error %d\n", rc);
                    continue;
                }
                return_address = src;
                storage_master_wakeup_edison();
            }

        } else if(message_buf[0] == RPC_RETURN) {
            gpio_set(2);
            memcpy(return_buf, message_buf+1,len-1);
            rpc_pending = 0;
            rc = signbus_io_send(return_address, 0, return_buf, len-1);
            if (rc < 0) {
                printf(" - signbus_io_send error %d\n", rc);
                continue;
            }
        }
    }
}

