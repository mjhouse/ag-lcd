# AG-LCD Examples

All examples assume an HD44780 two-line lcd screen connected to an Arduino Nano with the RW pin of the screen connected to 
ground, using a four-pin configuration (rather than eight). If you are using an eight pin configuration or have the RW pin
connected, you may need to uncomment lines or otherwise modify these examples to get them to work.

Pins should be connected as follows:

| Nano | LCD      |
|------|----------|
| d12  | RS       |
| d11  | E/Enable |
| d10  | RW       |
| d2   | D4       |
| d3   | D5       |
| d4   | D6       |
| d5   | D7       |

These examples require [ravedude](https://crates.io/crates/ravedude) to be installed. You can do that with `cargo install ravedude`.

## Autoscroll

Demos the autoscroll feature of HD44780 two-line lcd screens. Autoscroll "scrolls" the display as each character
is written, effectively pushing the displayed message out to the left as it is printed. The cursor doesn't move.

![Autoscroll Example Gif](../media/autoscroll_example.gif)

## Blink

Blinks the backround of the cursor.

![Blink Example Gif](../media/blink_example.gif)

## Character

Creates a custom character mapping in the CGRAM on the LCD and displays it (a sideways smiley face)

![Character Example Gif](../media/character_example.gif)

## Cursor

Blinks the cursor on and off.

![Cursor Example Gif](../media/cursor_example.gif)

## Display

Blinks the display on and off.

![Display Example Gif](../media/display_example.gif)

## Layout

Demos the text layout direction setting. Left-to-right is the standard layout direction for english, right-to-left
prints characters in reverse order and direction.

![Layout Example Gif](../media/layout_example.gif)

## Scroll

Scrolls the display left and right, given a direction and the number of positions to scroll by.

![Scroll Example Gif](../media/scroll_example.gif)