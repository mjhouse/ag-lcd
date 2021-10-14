# AG-LCD Examples

All examples assume an HD44780 two-line lcd screen connected to an Arduino Nano with the RW pin of the screen connected to 
ground, using a four-pin configuration (rather than eight). If you are using an eight pin configuration or have the RW pin
connected, you may need to uncomment lines or otherwise modify these examples to get them to work.

Pins should be connected as follows (Only required are uncommented in examples):

| Nano | LCD      | Required |
|------|----------|----------|
| d12  | RS       | YES      |
| d11  | E/Enable | YES      |
| d10  | RW       | NO       |
| d2   | D4       | YES      |
| d3   | D5       | YES      |
| d4   | D6       | YES      |
| d5   | D7       | YES      |
| d6   | D0       | NO       |
| d7   | D1       | NO       |
| d8   | D2       | NO       |
| d9   | D3       | NO       |

These examples require [ravedude](https://crates.io/crates/ravedude) to be installed. You can do that with `cargo install ravedude`.  

## Autoscroll

`cargo run --example autoscroll`  

Demos the autoscroll feature of HD44780 two-line lcd screens. Autoscroll "scrolls" the display as each character
is written, effectively pushing the displayed message out to the left as it is printed. The cursor doesn't move.

![Autoscroll Example Gif](../media/autoscroll_example.gif)

## Blink

`cargo run --example blink`  

Blinks the backround of the cursor.

![Blink Example Gif](../media/blink_example.gif)

## Character

`cargo run --example character`  

Creates a custom character mapping in the CGRAM on the LCD and displays it (a sideways smiley face)

![Character Example Gif](../media/character_example.gif)

## Cursor

`cargo run --example cursor`  

Blinks the cursor on and off.

![Cursor Example Gif](../media/cursor_example.gif)

## Display

`cargo run --example display`  

Blinks the display on and off.

![Display Example Gif](../media/display_example.gif)

## Layout

`cargo run --example layout`  

Demos the text layout direction setting. Left-to-right is the standard layout direction for english, right-to-left
prints characters in reverse order and direction.

![Layout Example Gif](../media/layout_example.gif)

## Scroll

`cargo run --example scroll`  

Scrolls the display left and right, given a direction and the number of positions to scroll by.

![Scroll Example Gif](../media/scroll_example.gif)