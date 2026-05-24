RM0456 Rev 6

RM0456

LCD-TFT display controller (LTDC)
some cases when displaying a 24-bit data on 18-bit display. Thus the dithering technique is
used to round data which is different from one frame to the other.
The dithering pseudo-random technique is the same as comparing LSBs against a
threshold value and adding a 1 to the MSB part only, if the LSB part is ≥ the threshold. The
LSBs are typically dropped once dithering was applied.
The width of the added pseudo-random value is two bits for each color channel: two bits for
red, two bits for green and two bits for blue.
Once the LTDC is enabled, the LFSR starts running with the first active pixel and it is kept
running even during blanking periods and when dithering is switched off. If the LTDC is
disabled, the LFSR is reset.
The dithering can be switched on and off on the fly through the LTDC_GCR register.

Reload shadow registers
Some configuration registers are shadowed. The shadow registers values can be reloaded
immediately to the active registers when writing to these registers or at the beginning of the
vertical blanking period following the configuration in the LTDC_SRCR register. If the
immediate reload configuration is selected, the reload must be activated only when all new
registers have been written.
The shadow registers must not be modified again before the reload is done. Reading from
the shadow registers returns the actual active value. The new written value can only be read
after the reload has taken place.
A register reload interrupt can be generated if enabled in the LTDC_IER register.
The shadowed registers are all Layer1 and Layer2 registers except LTDC_LxCLUTWR.

Interrupt generation event
Refer to Section 43.5: LTDC interrupts for the interrupt configuration.

43.4.2

Layer programmable parameters
Up to two layers can be enabled, disabled and configured separately. The layer display
order is fixed and it is bottom up. If two layers are enabled, the layer2 is the top displayed
window.

Windowing
Every layer can be positioned and resized and it must be inside the active display area.
The window position and size are configured through the top-left and bottom-right X/Y
positions and the internal timing generator that includes the synchronous, back porch size
and the active data area. Refer to LTDC_LxWHPCR and LTDC_WVPCR registers.
The programmable layer position and size defines the first/last visible pixel of a line and the
first/last visible line in the window. It allows to display either the full image frame or only a
part of the image frame (see Figure 409):
• The first and the last visible pixel in the layer are set by configuring the WHSTPOS[11:0]
and WHSPPOS[11:0] in the LTDC_LxWHPCR register.
• The first and the last visible lines in the layer are set by configuring the WVSTPOS[10:0]
and WVSPPOS[10:0] in the LTDC_LxWVPCR register.

RM0456 Rev 6

<!-- pagebreak -->

