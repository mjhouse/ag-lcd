# AG-LCD Examples

All examples assume: 

* A HD44780 two-line LCD screen 
* An Arduino Nano 
* RW pin of the LCD display is connected to GND
* At D4-D7 pins on LCD are connected to Nano

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

These examples require [ravedude](https://crates.io/crates/ravedude) to be installed. You can do that with `cargo install ravedude`.  

## Autoscroll ([code](autoscroll.rs))

`cargo run --example autoscroll`  

Autoscroll "scrolls" the display as each character is written, effectively pushing the displayed 
message out to the left as it is printed so that the cursor doesn't move.

![Autoscroll Example Gif](../media/autoscroll_example.gif)

## Blink ([code](blink.rs))

`cargo run --example blink`  

Blinks the backround of the cursor.

![Blink Example Gif](../media/blink_example.gif)

## Character ([code](character.rs))

`cargo run --example character`  

Creates a custom character mapping in the CGRAM on the LCD and displays it (a sideways smiley face)

![Character Example Gif](../media/character_example.gif)

## Cursor ([code](cursor.rs))

`cargo run --example cursor`  

Blinks the cursor on and off.

![Cursor Example Gif](../media/cursor_example.gif)

## Display ([code](display.rs))

`cargo run --example display`  

Blinks the display on and off.

![Display Example Gif](../media/display_example.gif)

## Layout ([code](layout.rs))

`cargo run --example layout`  

Demos the text layout direction setting. Left-to-right is the standard layout direction for english, right-to-left
prints characters in reverse order and direction.

![Layout Example Gif](../media/layout_example.gif)

## Scroll ([code](scroll.rs))

`cargo run --example scroll`  

Scrolls the display left and right, given a direction and the number of positions to scroll by.

![Scroll Example Gif](../media/scroll_example.gif)