RM0456 Rev 6

RM0456

Digital camera interface (DCMI)
Table 415. Positioning of captured data bytes in 32-bit words (14-bit width)

41.3.6

Byte address

31:30

29:16

15:14

13:0

0

0

Dn+1[13:0]

0

Dn[13:0]

4

0

Dn+3[13:0]

0

Dn+2[13:0]

DCMI synchronization
The digital camera interface supports embedded or hardware (DCMI_HSYNC and
DCMI_VSYNC) synchronization. When embedded synchronization is used, it is up to the
digital camera module to make sure that the 0x00 and 0xFF values are used ONLY for
synchronization (not in data). Embedded synchronization codes are supported only for the
8-bit parallel data interface width (that is, in the DCMI_CR register, the EDM[1:0] bits must
be cleared).
For compressed data, the DCMI supports only the hardware synchronization mode. In this
case, DCMI_VSYNC is used as a start/end of the image, and DCMI_HSYNC is used as a
Data Valid signal. Figure 394 shows the corresponding timing diagram.
Figure 394. Timing diagram
Padding data
at the end of the JPEG stream

Beginning of JPEG stream

Programmable
JPEG packet size

JPEG data
End of JPEG stream
DCMI_HSYNC

DCMI_VSYNC

Packet dispatching depends on the image content.
This results in a variable blanking duration.
JPEG packet data
ai15944b

Hardware synchronization mode
In hardware synchronization mode, the two synchronization signals
(DCMI_HSYNC/DCMI_VSYNC) are used.
Depending on the camera module/mode, data may be transmitted during horizontal/vertical
synchronization periods. The DCMI_HSYNC/DCMI_VSYNC signals act like blanking
signals since all the data received during DCMI_HSYNC/DCMI_VSYNC active periods are
ignored.
In order to correctly transfer images into the DMA/RAM buffer, data transfer is synchronized
with the DCMI_VSYNC signal. When the hardware synchronization mode is selected, and

RM0456 Rev 6

<!-- pagebreak -->

1696

Digital camera interface (DCMI)

RM0456

capture is enabled (CAPTURE bit set in DCMI_CR), data transfer is synchronized with the
deactivation of the DCMI_VSYNC signal (next start of frame).
Transfer can then be continuous, with successive frames transferred by DMA to successive
buffers or the same/circular buffer. To allow the DMA management of successive frames, a
VSIF (Vertical synchronization interrupt flag) is activated at the end of each frame.

Embedded data synchronization mode
In this synchronization mode, the data flow is synchronized using 32-bit codes embedded in
the data flow. These codes use the 0x00/0xFF values that are not used in data anymore.
There are 4 types of codes, all with a 0xFF0000XY format. The embedded synchronization
codes are supported only in 8-bit parallel data width capture (in the DCMI_CR register, the
EDM[1:0] bits must be cleared). For other data widths, this mode generates unpredictable
results and must not be used.
Note:

Camera modules can have 8 such codes (in interleaved mode). For this reason, the
interleaved mode is not supported by the camera interface (otherwise, every other
half-frame would be discarded).
•

Mode 2
Four embedded codes signal the following events
–

Frame start (FS)

–

Frame end (FE)

–

Line start (LS)

–

Line end (LE)

The XY values in the 0xFF0000XY format of the four codes are programmable (see
Section 41.5.7: DCMI embedded synchronization code register (DCMI_ESCR)).
A 0xFF value programmed as a “frame end” means that all the unused codes are
interpreted as valid frame end codes.
In this mode, once the camera interface has been enabled, the frame capture starts
after the first occurrence of the frame end (FE) code followed by a frame start (FS)
code.
•

Mode 1
An alternative coding is the camera mode 1. This mode is ITU656 compatible.
The codes signal another set of events:
–

SAV (active line) - line start

–

EAV (active line) - line end

–

SAV (blanking) - end of line during interframe blanking period

–

EAV (blanking) - end of line during interframe blanking period

This mode can be supported by programming the following codes:
•

FS ≤ 0xFF

•

FE ≤ 0xFF

•

LS ≤ SAV (active)

•

LE ≤ EAV (active)

An embedded unmask code is also implemented for frame/line start and frame/line end
codes. Using it, it is possible to compare only the selected unmasked bits with the
programmed code. A bit can therefore be selected to compare in the embedded code and

<!-- pagebreak -->

