RM0456 Rev 6

RM0456

44.6

DSI Host (DSI)

Functional description: adapted command mode on LTDC
interface
The adapted command mode, enables the system to input a stream of pixel from the LTDC
that is conveyed by DSI Host using the command mode transmission (using the DCS
packets). The adapted command mode also supports pixel input control rate signaling and
tearing effect report mechanism.
The adapted command mode makes it possible to send large amounts of data through the
memory_write_start (WMS) and memory_write_continue (WMC) DCS commands. It helps
in delivering a wider data bandwidth for the memory write operations sent in command
mode to MIPI® displays and to refresh large areas of pixels in high resolution displays. If
additional commands such as display configuration commands, read back commands, and
tearing effect initialization must be transferred, then the APB register interface must be used
to complement the adapted command mode functionality.
Adapted command mode of operation supports 16 bpp, 18 bpp, and 24 bpp RGB.
To transmit the image data in adapted command mode:
•

Set command mode (CMDM) bit of the DSI Host mode configuration register
(DSI_MCR) to 1.

•

Set DSI mode (DSIM) bit in the DSI Wrapper configuration register (DSI_WCFGR) to 1.

To transmit the image data, follow these steps:
•

Define the image area to be refreshed, by using the set_column_address and
set_page_address DCS commands. The image area needs to be defined only once
and remains effective until different values are defined.

•

Define the pixel color coding to be used by using the color coding (COLC) field in the
DSI Host LTDC color coding register (DSI_LCOLCR).

•

Define the virtual channel ID of the LTDC interface generated packets using the virtual
channel ID (VCID) field in the DSI Host LTDC VCID register (DSI_LVCIDR). These also
need to be defined only once.

•

Start transmitting the data from the LTDC setting the LTDC enable (LTDCEN) bit of the
DSI_WCR register.

Figure 417 shows the adapted command mode usage flow.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456
Figure 417. Adapted command mode usage flow

Video engine

DSI controller

genIF:

set_co

Display

lumn_a

ddress

DCS: s

et_colu

genIF:

set_pa

mn_ad

dress

ge_add

ress
DCS: s

et_pag

LTDCIF

: vsync

= 1, dp

idataen

e_addre

ss

=1
DCS: w

rite_me

DCS: w

mory_s

rite_me

tart

mory_c

ontinue

DCS: w

1

rite_me

LTDCIF

: vsync

= 0, dp

idataen

DCS: w
=0

rite_me

mory_c

mory_c

ontinue

2

ontinue

3

MSv35860V1

When the command mode (CMDM) bit of the DSI Host mode configuration register
(DSI_CFGR) is set to 1, the LTDC interface assumes the behavior corresponding to the
adapted command mode.
In this mode, the host processor can use the LTDC interface to transmit a continuous
stream of pixels to be written in the local frame buffer of the peripheral. It uses a pixel input
bus to receive the pixels and controls the flow automatically to limit the stream of continuous
pixels. When the first pixel is received, the current value of the command size (CMDSIZE)
field of the DSI Host LTDC command configuration register (DSI_LCCR), is shadowed to the
internal interface function. The interface increments a counter on every valid pixel that is
input through the interface. When this pixel counter reaches command size (CMDSIZE), a
command is written into the command FIFO, and the packet can be transmitted through the
DSI link.
If the last pixel arrives before the counter reaches the value of shadowed command size
(CMDSIZE), a WMS command is issued to the command FIFO with word count (WC) set to
the number of bytes corresponding to the counter value. If more than CMDSIZE pixels are
received (shadowed value), a WMS command is sent to the command FIFO with WC set to
the number of bytes corresponding to the command size (CMDSIZE) and the counter is
restarted.
After the first WMS command has been written to the FIFO, the circuit behaves in a similar
way, but issues WMC commands instead of WMS commands. The process is repeated until
the last pixel of the image is received. The core automatically starts sending a new packet

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)
when the last pixel of the image is received, falls, or command size (CMDSIZE) limit is
reached.

Synchronization with the LTDC
The DSI Wrapper performs the synchronization of the transfer process by:
•

controlling the start/halt of the LTDC

•

making the data flow control between LTDC and DSI Host.

The transfer to refresh the display frame buffer can be triggered
•

manually, setting the LTDC enable (LTDCEN) bit of the DSI Wrapper control register
(DSI_WCR)

•

automatically when a tearing effect (TEIF) event occurs, and automatic refresh (AR) is
enabled.

The selection between manual and automatic mode is done through the automatic refresh
(AR) bit of the DSI Wrapper configuration register (DSI_WCFGR).
Once the transfer of one frame is done whatever in manual or automatic refresh mode, the
DSI Wrapper halts the TFT display controller (LTDC), resetting the LTDC enable (LTDCEN)
bit of the DSI Wrapper control register (DSI_WCR), and setting the end of refresh interrupt
flag (ERIF) flag of the DSI Wrapper status register (DSI_WSR). If the end of refresh interrupt
enable (ERIE) bit of the DSI Wrapper configuration register (DSI_WCFGR) is set, an
interrupt is generated.
The end of refresh interrupt flag (ERIF) flag of the DSI Wrapper status register (DSI_WSR)
can be reset setting the clear end of refresh interrupt flag (CERIF) bit of the DSI Wrapper
clear interrupt flag register (DSI_WCIFR).
The halting of the TFT display controller (LTDC) by the DSI Wrapper is done synchronously
on a rising edge or a falling edge of VSync according to the VSync polarity (VSPOL) bit of
the DSI Wrapper configuration register (DSI_WCFGR). It is recommended to keep the
default polarity to guarantee a correct behavior.

Support of tearing effect
The DSI specification supports tearing effect function in command mode displays. It enables
the Host processor to receive timing accurate information about where the display
peripheral is in the process of reading the content of its frame buffer.
The tearing effect can be managed through:
•

a separate pin, which is not covered in the DSI specification

•

the DSI tearing effect functionality: a set_tear_on DCS command must be issued
through the APB interface using the generic interface registers.

Tearing effect through a GPIO
When the tearing effect source (TESRC) bit of the DSI Wrapper configuration register
(DSI_WCFGR) is set, the tearing effect is signaled through a GPIO.
The polarity of the input signal can be configured by the tearing effect polarity (TEPOL) bit of
the DSI Wrapper configuration register (DSI_WCFGR).
When the programmed edge is detected, the tearing effect interrupt flag (TEIF) bit of the
DSI Wrapper interrupt and status register (DSI_WISR) is set.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

If the tearing effect interrupt enable (TEIE) bit of the DSI Wrapper interrupt enable register
(DSI_WIER) is set, an interrupt is generated.

Tearing effect through DSI link
When the TESRC bit of the DSI Wrapper configuration register (DSI_WCFGR) is reset, the
tearing effect is managed through the DSI link:
The DSI Host performs a double bus turn-around (BTA) after sending the set_tear_on
command granting the ownership of the link to the DSI display. The display holds the
ownership of the bus until the tear event occurs, which is indicated to the DSI Host by a DPHY trigger event. The DSI Host then decodes the trigger and reports the event setting the
tearing effect interrupt flag (TEIF) bit of the DSI Wrapper interrupt and status register
(DSI_WISR).
If the tearing effect interrupt enable (TEIE) bit of the DSI Wrapper interrupt enable register
(DSI_WIER) is set, an interrupt is generated.
To use this function, it is necessary to issue a set_tear_on command after the update of the
display using the WMS and WMC DCS commands. This procedure halts the DSI link until
the display is ready to receive a new frame update.
The DSI Host does not automatically generate the tearing effect request (double BTA) after
a WMS/WMC sequence for flexibility purposes, so several regions of the display can be
updated improving DSI bandwidth usage. Tearing effect request must always be triggered
by a set_tear_on command in the DSI Host implementation.
Configure the following registers to activate the tearing effect:

<!-- pagebreak -->

