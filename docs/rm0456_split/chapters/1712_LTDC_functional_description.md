1741

LCD-TFT display controller (LTDC)

RM0456

43.3

LTDC functional description

43.3.1

LTDC block diagram
Figure 407. LTDC block diagram
Pixel clock domain

AHB clock domain
32-bit
AHB bus

AHB
master

ltdc_hclk

Layer 1
FIFO

PFC(1)

Layer 2
FIFO

PFC

LCD_R[7:0]
Blending
unit

Dithering
unit

LCD_G[7:0]
LCD_B[7:0]
LCD_CLK

ltdc_ker_ck

ltdc_it
ltdc_err_it
ltdc_li

Timing
generator

APB clock domain

LCD_VSYNC
LCD_HSYNC
LCD_DE

Configuration
and status
registers

ltdc_pclk

32-bit
APB bus

APB
slave

(1) PFC: pixel format converter, performing the pixel format conversion from the selected input pixel format of a layer to words.

43.3.2

MSv66004V1

LTDC pins and internal signals
The table below summarizes the LTDC signal interface.
Table 428. LTDC external pins
LCD-TFT signals

Signal type

Description

LCD_CLK

Output

Clock output

LCD_HSYNC

Output

Horizontal synchronization

LCD_VSYNC

Output

Vertical synchronization

LCD_DE

Output

Not data enable

LCD_R[7:0]

Output

8-bit Red data

LCD_G[7:0]

Output

8-bit Green data

LCD_B[7:0]

Output

8-bit Blue data

The LTDC pins must be configured by the user application. The unused pins can be used for
other purposes.
For LTDC outputs up to 24 bits (RGB888), if less than 8 bpp are used to output for example
RGB565 or RGB666 to interface on 16- or 18-bit displays, the RGB display data lines must
be connected to the MSB of the LTDC RGB data lines.
As an example, in the case of an LTDC interfacing with a RGB565 16-bit display, the LTDC
display R[4:0], G[5:0] and B[4:0] data lines pins must be connected to the LCD_R[7:3],
LCD_G[7:2] and LCD_B[7:3] pins.

<!-- pagebreak -->

