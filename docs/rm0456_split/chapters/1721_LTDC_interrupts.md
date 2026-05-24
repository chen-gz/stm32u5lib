RM0456 Rev 6

RM0456

LCD-TFT display controller (LTDC)

Default color
Every layer can have a default color in the format ARGB which is used outside the defined
layer window or when a layer is disabled.
The default color is configured through the LTDC_LxDCCR register.
The blending is always performed between the two layers even when a layer is disabled. To
avoid displaying the default color when a layer is disabled, keep the blending factors of this
layer in the LTDC_LxBFCR register to their reset value.

Color keying
A color key (RGB) can be configured to be representative for a transparent pixel.
If the color keying is enabled, the current pixels (after format conversion and before CLUT
respectively blending) are compared to the color key. If they match for the programmed
RGB value, all channels (ARGB) of that pixel are set to 0.
The color key value can be configured and used at run-time to replace the pixel RGB value.
The color keying is enabled through the LTDC_LxCKCR register.
The color keying is configured through the LTDC_LxCKCR register. The programmed value
depends on the pixel format as it is compared to current pixel after pixel format conversion
to ARGB888.
Example: if the a mid-yellow color (50 % red + 50 % green) is used as the transparent color
key:

43.5

•

In RGB565, the mid-yellow color is 0x8400. Set the LTDC_LxCKCR to 0x848200.

•

In ARGB8888, the mid-yellow color is 0x808000. Set LTDC_LxCKCR to 0x808000.

•

In all CLUT-based color modes (L8, AL88, AL44), set one of the palette entry to the
mid-yellow color 0x808000 and set the LTDC_LxCKCR to 0x808000.

LTDC interrupts
The LTDC provides four maskable interrupts logically ORed to two interrupt vectors.
The interrupt sources can be enabled or disabled separately through the LTDC_IER
register. Setting the appropriate mask bit to 1 enables the corresponding interrupt.
The two interrupts are generated on the following events:
•

Line interrupt: generated when a programmed line is reached. The line interrupt
position is programmed in the LTDC_LIPCR register

•

Register reload interrupt: generated when the shadow registers reload is performed
during the vertical blanking period

•

FIFO underrun interrupt: generated when a pixel is requested from an empty layer
FIFO

•

Transfer error interrupt: generated when an AHB bus error occurs during data transfer

RM0456 Rev 6

<!-- pagebreak -->

