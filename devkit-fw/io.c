#include <stdio.h>
#include "hardware/gpio.h"

#include "io.h"

int data_dir = -999;

void write_addr_pins(uint data) {
    gpio_put(A_1, data & 1);
    gpio_put(A_2, (data & 2) >> 1);
    gpio_put(A_3, (data & 4) >> 2);
    gpio_put(A_4, (data & 8) >> 3);
    gpio_put(A_5, (data & 16) >> 4);
    gpio_put(A_6, (data & 32) >> 5);
    gpio_put(A_7, (data & 64) >> 6);
    gpio_put(A_8, (data & 128) >> 7);
}

void write_data_pins(uint data) {
    set_data_pins_dir(GPIO_OUT);

    gpio_put(D_1, data & 1);
    gpio_put(D_2, (data & 2) >> 1);
    gpio_put(D_3, (data & 4) >> 2);
    gpio_put(D_4, (data & 8) >> 3);
    gpio_put(D_5, (data & 16) >> 4);
    gpio_put(D_6, (data & 32) >> 5);
    gpio_put(D_7, (data & 64) >> 6);
    gpio_put(D_8, (data & 128) >> 7);
}

char read_data_pins() {
    set_data_pins_dir(GPIO_IN);

    return gpio_get(D_1)
         | (gpio_get(D_2) << 1)
	 | (gpio_get(D_3) << 2)
	 | (gpio_get(D_4) << 3)
         | (gpio_get(D_5) << 4)
         | (gpio_get(D_6) << 5)
         | (gpio_get(D_7) << 6)
         | (gpio_get(D_8) << 7);
}

void set_addr_pins_dir(int dir) {
    gpio_set_dir(A_1, dir);
    gpio_set_dir(A_2, dir);
    gpio_set_dir(A_3, dir);
    gpio_set_dir(A_4, dir);
    gpio_set_dir(A_5, dir);
    gpio_set_dir(A_6, dir);
    gpio_set_dir(A_7, dir);
    gpio_set_dir(A_8, dir);
}

void set_data_pins_dir(int dir) {
    if (data_dir == dir) {
        return;
    }

    gpio_set_dir(D_1, dir);
    gpio_set_dir(D_2, dir);
    gpio_set_dir(D_3, dir);
    gpio_set_dir(D_4, dir);
    gpio_set_dir(D_5, dir);
    gpio_set_dir(D_6, dir);
    gpio_set_dir(D_7, dir);
    gpio_set_dir(D_8, dir);
    data_dir = dir;
}

void init_io_pins() {
    gpio_init(A_1);
    gpio_init(A_2);
    gpio_init(A_3);
    gpio_init(A_4);
    gpio_init(A_5);
    gpio_init(A_6);
    gpio_init(A_7);
    gpio_init(A_8);

    gpio_init(D_1);
    gpio_init(D_2);
    gpio_init(D_3);
    gpio_init(D_4);
    gpio_init(D_5);
    gpio_init(D_6);
    gpio_init(D_7);
    gpio_init(D_8);
}
