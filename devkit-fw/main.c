#include <stdio.h>
#include "pico/stdlib.h"
#include "hardware/gpio.h"
#include "pico/binary_info.h"

const uint A_1 = 6;
const uint A_2 = 7;
const uint A_3 = 8;
const uint A_4 = 9;
const uint A_5 = 10;
const uint A_6 = 11;
const uint A_7 = 12;
const uint A_8 = 13;

const uint D_1 = 18;
const uint D_2 = 19;
const uint D_3 = 20;
const uint D_4 = 21;
const uint D_5 = 26;
const uint D_6 = 27;
const uint D_7 = 28;
const uint D_8 = 29;

void write_data();
void init_io_pins();
void set_addr_pins_dir(int);
void set_data_pins_dir(int);
void write_addr_pins(uint data);
void write_data_pins(uint data);
char read_data_pins();

int main() {
    stdio_init_all();

    init_io_pins();

    set_addr_pins_dir(GPIO_OUT);
    set_data_pins_dir(GPIO_IN);

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
    gpio_set_dir(D_1, dir);
    gpio_set_dir(D_2, dir);
    gpio_set_dir(D_3, dir);
    gpio_set_dir(D_4, dir);
    gpio_set_dir(D_5, dir);
    gpio_set_dir(D_6, dir);
    gpio_set_dir(D_7, dir);
    gpio_set_dir(D_8, dir);
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
