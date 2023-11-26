#include <stdio.h>
#include "pico/stdlib.h"
#include "hardware/gpio.h"
#include "pico/binary_info.h"

#include "io.h"

void output_shiftregister();
char read_mem_addr();
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
            case 'r':
                read_mem_addr();
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

void output_shiftregister() {
    uint val;
    scanf("%d", &val);
    set_shiftreg_value(val);
    printf("s: '%d'\n", val);
}

char read_mem_addr() {
    char bank, addr_high, addr_low;
    scanf("%c", &bank);
    scanf("%c", &addr_high);
    scanf("%c", &addr_low);

    set_mem_read(false);
    set_mem_write(false);

    set_shiftreg_value(((uint)bank << 8) || (uint)addr_high);
    set_shiftreg_output_enabled(true);

    set_data_pins_dir(GPIO_IN);
    set_addr_pins_dir(GPIO_OUT);
    write_addr_pins(addr_low);

    set_mem_read(true);
    char val = read_data_pins();

    set_addr_pins_dir(GPIO_IN);
    set_shiftreg_output_enabled(false);
    set_mem_read(false);

    printf("r: '%c'\n", val);
    return val;
}

void write_256() {
    char bank, addr_high;
    scanf("%c", &bank);
    scanf("%c", &addr_high);
    char* data[256];
    for (int i=0; i<256; i++) {
        scanf("%c", &data[i]);
    }

    set_mem_read(false);
    set_mem_write(false);

    set_data_pins_dir(GPIO_OUT);
    set_addr_pins_dir(GPIO_OUT);

    set_shiftreg_value(((uint)bank << 8) || (uint)addr_high);
    set_shiftreg_output_enabled(true);

    for (int i=0; i<256; i++) {
        write_addr_pins(i);
        write_data_pins(*data[i]);
        set_mem_write(true);
        set_mem_write(false);
    }

    set_data_pins_dir(GPIO_IN);
    set_addr_pins_dir(GPIO_IN);
    set_shiftreg_output_enabled(false);

    printf("a\n");
}