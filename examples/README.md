# AG-LCD Examples

All examples assume: 

* A HD44780 two-line LCD screen 
* An Arduino Nano for examples in directory `nano/`, and an Arduino Uno for examples in directory `uno/`
* RW pin of the LCD display is connected to GND
* D4-D7 pins on LCD are connected to Arduino
* RS and E (Enable) are connected to Arduino

If you are using an eight pin configuration or have the RW pin connected, you may need to 
uncomment lines or otherwise modify these examples to get them to work.  

Pins should be connected as follows (optional pins are commented in examples):

| Arduino | LCD      | Required |
|---------|----------|----------|
| d12     | RS       | YES      |
| d11     | RW       | NO       |
| d10     | E/Enable | YES      |
| d9      | D0       | NO       |
| d8      | D1       | NO       |
| d7      | D2       | NO       |
| d6      | D3       | NO       |
| d5      | D4       | YES      |
| d4      | D5       | YES      |
| d3      | D6       | YES      |
| d2      | D7       | YES      |

# Running

* You'll need [ravedude](https://crates.io/crates/ravedude) installed. You can do that by following their installation instructions for your system.
* Edit the `.cargo/config.toml` file for the example you want to run and add the `--port` argument with the correct device id for your hardware USB cable.
* Open a terminal in the example directory and run with `cargo run`

# Troubleshooting
* If your using a Nano board and recieve `avrdude: stk500_getsync() attempt 1 of 10: not in sync: resp=0x00` upon running `cargo run`, it might be because your board was manufactured after January 2018. In that case you'll need the new boot loader, which is achieved by changing `nano` to `nano-new` for `runner` in `.cargo/config.toml` in the example directory.
* Regarding Arduino Uno: only setups using required pins have been tested to work; some users have reported issues with initializing the LCD (see [#4](https://github.com/mjhouse/ag-lcd/issues/4))