RM0456 Rev 6

RM0456

44.11.2

DSI Host (DSI)

Color coding
Table 443 shows the RGB components used.
Table 443. RGB components

44.11.3

White

Yellow

Cyan

Green

Magenta

Red

Blue

Black

R

High

High

Low

Low

High

High

Low

Low

G

High

High

High

High

Low

Low

Low

Low

B

High

Low

High

Low

High

Low

High

Low

BER testing pattern
The BER testing pattern simplifies conformance testing. This pattern tests the RX D-PHY
capability to receive the data correctly. The following data patterns are required:
•

X bytes of 0xAA (high-frequency pattern, inverted)

•

X bytes of 0x33 (mid-frequency pattern)

•

X bytes of 0xF0 (low-frequency pattern, inverted)

•

X bytes of 0x7F (lone 0 pattern)

•

X bytes of 0x55 (high-frequency pattern)

•

X bytes of 0xCC (mid-frequency pattern, inverted)

•

X bytes of 0x0F (low-frequency pattern)

•

Y bytes of 0x80 (lone 1 pattern).

In most cases, Y is equal to X. However, depending on line length and the color coding
used, Y may be different from X. With RGB888 color coding and horizontal resolution in
multiples of eight, the pattern shown in Figure 441 appears on the DSI display.
Figure 441. RGB888 BER testing pattern

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

44.11.4

RM0456

Video mode pattern generator resolution
Depending on the orientation, BER mode, and color coding, the smallest resolutions
accepted by the video mode pattern generator are:
•

BER mode: 8x8

•

horizontal color bar mode: 8x8

•

vertical color bar mode: 8x8.

Vertical pattern
The width of each color bar is determined by the division of horizontal resolution (pixels) for
eight test pattern colors. If the horizontal resolution is not divisible by eight, the last color
(black) is extended to fill the resolution.
In the example in Figure 442, the horizontal resolution is 103.
Figure 442. Vertical pattern (103x15)

Horizontal pattern
The width of each color bar is determined by the division of the number of vertical resolution
(lines) for eight test pattern colors. If the vertical resolution is not divisible by eight, the last
color (black) is extended to fill the resolution, as shown in Figure 443.
Figure 443. Horizontal pattern (103x15)

<!-- pagebreak -->

