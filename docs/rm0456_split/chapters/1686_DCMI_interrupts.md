1696

Digital camera interface (DCMI)

RM0456

Half resolution image extraction
This is a modification of the previous reception modes, being applicable to monochrome,
RGB or Y extraction modes.
This mode is used to only store a half resolution image. It is selected through OELS and
LSM control bits.

41.4

DCMI interrupts
Five interrupts are generated. All interrupts are maskable by software. The global interrupt
(dcmi_it) is the OR of all the individual interrupts. The table below gives the list of all
interrupts.
Table 420. DCMI interrupts

Interrupt
acronym

dcmi_it

41.5

Interrupt clear
method

Exits
Sleep
mode

Exists
Stop and
Standby
modes

Interrupt event

Event flag

Enable
control bit

End of line

LINE_RIS

LINE_IE

Set LINE_ISC

Yes

No

End of frame capture

FRAME_RIS

FRAME_IE

Set FRAME_ISC

Yes

No

Overrun of data reception

OVR_RIS

OVR_IE

Set OVR_ISC

Yes

No

Synchronization frame

VSYNC_RIS

VSYNC_IE

Set VSYNC_ISC

Yes

No

Detection of an error in the
embedded
synchronization frame
detection

ERR_RIS

ERR_IE

Set ERR_ISC

Yes

No

DCMI registers
Refer to Section 1.2 on page 127 for list of abbreviations used in register descriptions. All
DCMI registers must be accessed as 32-bit words, otherwise a bus error occurs.

41.5.1

DCMI control register (DCMI_CR)
Address offset: 0x00
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

OELS

LSM

OEBS

rw

rw

rw

rw

4

3

2

1

0

15

14

Res.

ENABL
E
rw

13
Res.

12
Res.

11

10

9

8

EDM[1:0]

FCRC[1:0]

rw

rw

rw

7

5

PCKPO
VSPOL HSPOL
L

rw

rw

Bits 31:21 Reserved, must be kept at reset value.

<!-- pagebreak -->

6

RM0456 Rev 6

rw

rw

17

16

BSM[1:0]
rw

ESS

JPEG

CROP

CM

CAPTU
RE

rw

rw

rw

rw

rw

RM0456

Digital camera interface (DCMI)

Bit 20 OELS: Odd/Even Line Select (Line Select Start)
This bit works in conjunction with the LSM field (LSM = 1).
0: Interface captures first line after the frame start, second one being dropped.
1: Interface captures second line from the frame start, first one being dropped.
Bit 19 LSM: Line Select mode
0: Interface captures all received lines.
1: Interface captures one line out of two.
Bit 18 OEBS: Odd/Even Byte Select (Byte Select Start)
This bit works in conjunction with BSM field (BSM ≠ 00).
0: Interface captures first data (byte or double byte) from the frame/line start, second one
being dropped.
1: Interface captures second data (byte or double byte) from the frame/line start, first one
being dropped.
Bits 17:16 BSM[1:0]: Byte Select mode
00: Interface captures all received data.
01: Interface captures every other byte from the received data.
10: Interface captures one byte out of four.
11: Interface captures two bytes out of four.
Note: This mode only works for EDM[1:0] = 00. For all other EDM values, this field must be
programmed to the reset value.
Bit 15 Reserved, must be kept at reset value.
Bit 14 ENABLE: DCMI enable
0: DCMI disabled
1: DCMI enabled
Note: The DCMI configuration registers must be programmed correctly before enabling this
bit.
Bits 13:12 Reserved, must be kept at reset value.
Bits 11:10 EDM[1:0]: Extended data mode
00: Interface captures 8-bit data on every pixel clock.
01: Interface captures 10-bit data on every pixel clock.
10: Interface captures 12-bit data on every pixel clock.
11: Interface captures 14-bit data on every pixel clock.
Bits 9:8 FCRC[1:0]: Frame capture rate control
These bits define the frequency of frame capture. They are meaningful only in Continuous
grab mode. They are ignored in snapshot mode.
00: All frames are captured.
01: Every alternate frame captured (50% bandwidth reduction)
10: One frame out of four captured (75% bandwidth reduction)
11: reserved
Bit 7 VSPOL: Vertical synchronization polarity
This bit indicates the level on the DCMI_VSYNC pin when the data are not valid on the
parallel interface.
0: DCMI_VSYNC active low
1: DCMI_VSYNC active high

RM0456 Rev 6

<!-- pagebreak -->

1696

Digital camera interface (DCMI)

RM0456

Bit 6 HSPOL: Horizontal synchronization polarity
This bit indicates the level on the DCMI_HSYNC pin when the data are not valid on the
parallel interface.
0: DCMI_HSYNC active low
1: DCMI_HSYNC active high
Bit 5 PCKPOL: Pixel clock polarity
This bit configures the capture edge of the pixel clock.
0: Falling edge active
1: Rising edge active
Bit 4 ESS: Embedded synchronization select
0: Hardware synchronization data capture (frame/line start/stop) is synchronized with the
DCMI_HSYNC/DCMI_VSYNC signals.
1: Embedded synchronization data capture is synchronized with synchronization codes
embedded in the data flow.
Note: Valid only for 8-bit parallel data. HSPOL/VSPOL are ignored when the ESS bit is set.
This bit is disabled in JPEG mode.
Bit 3 JPEG: JPEG format
0: Uncompressed video format
1: This bit is used for JPEG data transfers. The DCMI_HSYNC signal is used as data enable.
The crop and embedded synchronization features (ESS bit) cannot be used in this mode.
Bit 2 CROP: Crop feature
0: The full image is captured. In this case the total number of bytes in an image frame must
be a multiple of four.
1: Only the data inside the window specified by the crop register is captured. If the size of the
crop window exceeds the picture size, then only the picture size is captured.
Bit 1 CM: Capture mode
0: Continuous grab mode - The received data are transferred into the destination memory
through the DMA. The buffer location and mode (linear or circular buffer) is controlled
through the system DMA.
1: Snapshot mode (single frame) - Once activated, the interface waits for the start of frame
and then transfers a single frame through the DMA. At the end of the frame, the CAPTURE
bit is automatically reset.
Bit 0 CAPTURE: Capture enable
0: Capture disabled
1: Capture enabled
The camera interface waits for the first start of frame, then a DMA request is generated to
transfer the received data into the destination memory.
In snapshot mode, the CAPTURE bit is automatically cleared at the end of the first frame
received.
In continuous grab mode, if the software clears this bit while a capture is ongoing, the bit is
effectively cleared after the frame end.
Note: The DMA controller and all DCMI configuration registers must be programmed correctly
before enabling this bit.

<!-- pagebreak -->

