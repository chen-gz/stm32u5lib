1.

Enable the LTDC clock in the RCC register.

2.

Configure the required pixel clock following the panel datasheet.

3.

Configure the synchronous timings: VSYNC, HSYNC, vertical and horizontal back
porch, active data area and the front porch timings following the panel datasheet as
described in the Section 43.4.1.

4.

Configure the synchronous signals and clock polarity in the LTDC_GCR register.

5.

If needed, configure the background color in the LTDC_BCCR register.

6.

Configure the needed interrupts in the LTDC_IER and LTDC_LIPCR register.

7.

Configure the layer1/2 parameters by:
–

programming the layer window horizontal and vertical position in the
LTDC_LxWHPCR and LTDC_WVPCR registers. The layer window must be in the
active data area.

–

programming the pixel input format in the LTDC_LxPFCR register

–

programming the color frame buffer start address in the LTDC_LxCFBAR register

–

programming the line length and pitch of the color frame buffer in the
LTDC_LxCFBLR register

–

programming the number of lines of the color frame buffer in the
LTDC_LxCFBLNR register

–

if needed, loading the CLUT with the RGB values and its address in the
LTDC_LxCLUTWR register

–

If needed, configuring the default color and the blending factors respectively in the
LTDC_LxDCCR and LTDC_LxBFCR registers
RM0456 Rev 6

RM0456

LCD-TFT display controller (LTDC)
8.

Enable layer1/2 and if needed the CLUT in the LTDC_LxCR register.

9.

If needed, enable dithering and color keying respectively in the LTDC_GCR and
LTDC_LxCKCR registers. They can be also enabled on the fly.

10. Reload the shadow registers to active register through the LTDC_SRCR register.
11. Enable the LTDC controller in the LTDC_GCR register.
12. All layer parameters can be modified on the fly except the CLUT. The new configuration
must be either reloaded immediately or during vertical blanking period by configuring
the LTDC_SRCR register.
Note:

All layer’s registers are shadowed. Once a register is written, it must not be modified again
before the reload has been done. Thus, a new write to the same register overrides the
previous configuration if not yet reloaded.

43.7

LTDC registers

43.7.1

LTDC synchronization size configuration register (LTDC_SSCR)
Address offset: 0x008
Reset value: 0x0000 0000
This register defines the number of horizontal synchronization pixels minus 1 and the
number of vertical synchronization lines minus 1. Refer to Figure 408 and Section 43.4 for
an example of configuration.

31

30

29

28

Res.

Res.

Res.

Res.

27

26

25

24

23

22

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

Res.

Res.

Res.

Res.

Res.

21

20

19

18

17

16

rw

rw

rw

rw

rw

rw

5

4

3

2

1

0

rw

rw

rw

rw

rw

HSW[11:0]

VSH[10:0]
rw

rw

rw

rw

rw

rw

Bits 31:28 Reserved, must be kept at reset value.
Bits 27:16 HSW[11:0]: Horizontal synchronization width (in units of pixel clock period)
This bitfield defines the number of Horizontal Synchronization pixel minus 1.
Bits 15:11 Reserved, must be kept at reset value.
Bits 10:0 VSH[10:0]: Vertical synchronization height (in units of horizontal scan line)
This bitfield defines the vertical Synchronization height minus 1. It represents the number of
horizontal synchronization lines.

43.7.2

LTDC back porch configuration register (LTDC_BPCR)
Address offset: 0x00C
Reset value: 0x0000 0000
This register defines the accumulated number of horizontal synchronization and back porch
pixels minus 1 (HSYNC width + HBP - 1) and the accumulated number of vertical

RM0456 Rev 6

<!-- pagebreak -->

