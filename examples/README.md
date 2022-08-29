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

| Nano | LCD      | Required |
|------|----------|----------|
| d12  | RS       | YES      |
| d11  | RW       | NO       |
| d10  | E/Enable | YES      |
| d9   | D0       | NO       |
| d8   | D1       | NO       |
| d7   | D2       | NO       |
| d6   | D3       | NO       |
| d5   | D4       | YES      |
| d4   | D5       | YES      |
| d3   | D6       | YES      |
| d2   | D7       | YES      |

Note, that in the case of Arduino Uno, only setups using required pins have been tested to work.

These examples require [ravedude](https://crates.io/crates/ravedude) to be installed. You can do that with `cargo install ravedude`.  