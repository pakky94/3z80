#include <stdio.h>
#include "pico/stdlib.h"
#include "hardware/gpio.h"
#include "pico/binary_info.h"

#include "io.h"

void write_data();
void output_shiftregister();
void write_256();

int main() {
    stdio_init_all();

    init_io_pins();

    set_addr_pins_dir(GPIO_OUT);
    set_data_pins_dir(GPIO_OUT);
    set_shiftreg_output_enabled(true);

    char command;

    while (true) {
        scanf("%c", &command);

        switch (command) {
            case 100:
            write_data();
            break;

            case 'r':
                printf("r: '%d'\n", read_data_pins());
                break;

            case 's':
                output_shiftregister();
                break;

            case 'W':
                write_256();
                break;

            default:
                printf(" - %c\n", command);
                break;
        }
    }
}

void write_data() {
    char addr_high, addr_low;
    scanf("%c", &addr_high);
    scanf("%c", &addr_low);
    char* data[256];
    for (int i=0; i<256; i++) {
        scanf("%c", &data[i]);
    }

    printf("addr: %c%c\n", addr_high, addr_low);
    printf("addr: %c%c\n", addr_high, addr_low);

    for (int i=0; i<256; i++) {
        printf("%c", data[i]);
    }
    printf("\n");
}

void output_shiftregister() {
    uint val;
    scanf("%d", &val);
    set_shiftreg_value(val);
    printf("s: '%d'\n", val);
}

void write_256() {
    char addr_high, addr_low;
    scanf("%c", &addr_high);
    scanf("%c", &addr_low);
    char* data[256];
    for (int i=0; i<256; i++) {
        scanf("%c", &data[i]);
    }

    set_data_pins_dir(GPIO_OUt);
    set_addr_pins_dir(GPIO_OUt);

    set_shiftreg_value(((uint)addr_high << 8) || (uint)addr_low);
    set_shiftreg_output_enabled(true);

    for (int i=0; i<256; i++) {
        write_addr_pins(i);
        write_data_pins(data[i]);
        // TODO: write sequence
    }

    printf("a\n");
}