## Build fpga

```sh
yosys -s build.ys
```

## Run test

```sh
iverilog -o build/tb_latch d_latch.v d_latch_ts.v && ./build/tb_latch
```
