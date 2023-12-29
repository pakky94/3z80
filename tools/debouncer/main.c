#include <stdio.h>
#include "pico/stdlib.h"
#include "hardware/gpio.h"
#include "hardware/timer.h"
#include "pico/binary_info.h"

int INPUTS[4] = { 18, 19, 20, 21 };
int OUTPUTS[4] = { 26, 27, 28, 29 };

void init_io_pins() {
    for (int i=0; i < 4; i++) {
        gpio_init(INPUTS[i]);
        gpio_init(OUTPUTS[i]);
        gpio_set_dir(INPUTS[i], GPIO_IN);
        gpio_set_dir(OUTPUTS[i], GPIO_OUT);
    }
}


int main() {
    stdio_init_all();
    init_io_pins();

    absolute_time_t x = get_absolute_time();

    absolute_time_t lastChange[4] = { x, x, x, x };
    uint status[4] = { 0, 0, 0, 0 };

    while (true) {
        sleep_ms(15);
        absolute_time_t now = get_absolute_time();
        for (int i=0; i < 4; i++) {
            //uint8 curr = gpio_get(input[4]);
            gpio_put(OUTPUTS[i], gpio_get(INPUTS[i]));
        }
    }
}