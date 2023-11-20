#ifndef A_1

#define A_1 6
#define A_2 7
#define A_3 8
#define A_4 9
#define A_5 10
#define A_6 11
#define A_7 12
#define A_8 13

#define D_1 18
#define D_2 19
#define D_3 20
#define D_4 21
#define D_5 26
#define D_6 27
#define D_7 28
#define D_8 29

#define SR_OE_N 2
#define SR_LATCH 3
#define SR_DATA 14
#define SR_CLOCK 15

#define MEM_WE 16
#define MEM_RE 17

void init_io_pins();
void set_addr_pins_dir(int);
void set_data_pins_dir(int);
void write_addr_pins(uint data);
void write_data_pins(uint data);
char read_data_pins();
void set_shiftreg_value(uint value);
void set_shiftreg_output_enabled(bool enabled);

#endif
