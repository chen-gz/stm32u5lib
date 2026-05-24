RM0456 Rev 6

RM0456

DSI Host (DSI)
The light yellow boxes in Figure 445 illustrate the location of some of the errors.
Figure 445. Error sources
DPI_PAYLOAD_WR_ERR

LTDC
Interface

LTDC

Packet Handler
LTDC
ctrl FIFO
Video Mode FSM
LTDC
pixel FIFO

Command Mode FSM

GEN_PAYLOAD_SEND_ERR

GEN_PAYLOAD_RECV_ERR

GEN_COMMAND_WR_ERR

GEN Comm
FIFOs
GEN
Interface

APB

GEN Pld
Send FIFOs
GEN Pld
RCV FIFOs

GEN_PAYLOAD_WR_ERR

ECC_SINGLE_ERR

Packet Analyzer
GEN Pld
Send FIFOs

GEN_PAYLOAD_RD_ERR

GEN Pld
RCV FIFOs

ECC_MULTI_ERR

VC
Router

ECC/CRC
Analysis

CRC_ERR
PKT_SIZE_ERR
EOTP_ERR

ACK_WITH_ERR
MSv35896V2

Table 447 explains the reasons that set off these interrupts and also explains how to recover
from these interrupts.
Table 447. Error causes and recovery
DSI Host interrupt
and status register

0

0

Bit

20

19

Name

Error cause

Recommended method
to handle the error

PE4

The D-PHY reports the LP1
contention error.
The D-PHY host detects the
contention while trying to drive
the line high.

Recover the D-PHY from contention.
Reset the DSI Host and transmit the
packets again. If this error is recurrent,
carefully analyze the connectivity between
the Host and the device.

PE3

D-PHY reports the LP0
contention error.
The D-PHY Host detects the
contention while trying to drive
the line low.

Recover the D-PHY from contention.
Reset the DSI Host and transmit the
packets again. If this error is recurrent,
carefully analyze the connectivity between
the Host and the device.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456
Table 447. Error causes and recovery (continued)

DSI Host interrupt
and status register

0

0

0

0

0

0

0

0

<!-- pagebreak -->

Bit

18

17

16

15

14

13

12

11

Name

Error cause

Recommended method
to handle the error

PE2

The D-PHY reports the false
control error.
The D-PHY detects an incorrect
line state sequence in lane 0
lines.

Device does not behave as expected,
communication with the device is not
properly established. This is an
unrecoverable error.
Reset the DSI Host and the D-PHY. If this
error is recurrent, analyze the behavior of
the device.

PE1

The D-PHY reports the LPDT
error.
The D-PHY detects that the
LDPT did not match a multiple
of 8 bits.

The data reception is not reliable. The DPHY recovers but the received data from
the device might not be reliable.
It is recommended to reset the DSI Host
and repeat the RX transmission.

PE0

The D-PHY reports the escape
entry error.
The D-PHY does not recognize
the received escape entry code.

The D-PHY Host does not recognize the
escape entry code. The transmission is
ignored. The D-PHY Host recovers but the
system must repeat the RX reception.

AE15

This error is directly retrieved
from acknowledge with error
packet.
The device detected a protocol
violation in the reception.

Refer to the display documentation. When
this error is active, the device must have
another read-back command that reports
additional information about this error.
Read the additional information and take
appropriate actions.

AE14

The acknowledge with error
packet contains this error.
The device chooses to use this
bit for error report.

Refer to the device documentation
regarding possible reasons for this error
and take appropriate actions.

The acknowledge with error
packet contains this error.
The device reports that the
transmission length does not
match the packet length.

Possible reason for this is multiple errors
present in the packet header (more than 2),
so the error detection fails and the device
does not discard the packet. In this case,
the packet header is corrupt and can cause
decoding mismatches.
Transmit the packets again. If this error is
recurrent, carefully analyze the connectivity
between the Host and the device.

AE12

The acknowledge with error
packet contains this error.
The device does not recognize
the VC ID in at least one of the
received packets.

Possible reason for this is multiple errors
present in the packet header (more than 2),
so the error detection fails and the device
does not discard the packet. In this case,
the packet header is corrupt and can cause
decoding mismatches.
Transmit the packets again. If this error is
recurrent, carefully analyze the connectivity
between the Host and the device.

AE11

The acknowledge with error
packet contains this error.
The device does not recognize
the data type of at least one of
the received packets.

Check the device capabilities. It is possible
that there are some packets not supported
by the device.
Repeat the transmission.

AE13

RM0456 Rev 6

RM0456

DSI Host (DSI)
Table 447. Error causes and recovery (continued)

DSI Host interrupt
and status register

0

0

0

0

0

Bit

10

9

8

7

6

Name

Error cause

Recommended method
to handle the error

The acknowledge with error
packet contains this error.
The device detects the CRC
errors in at least one of the
received packets.

Some of the long packets, transmitted after
the last acknowledge request, might
contain the CRC errors in the payload.
If the payload content is critical, transmit
the packets again.
If this error is recurrent, carefully analyze
the connectivity between the Host and the
device.

AE9

The acknowledge with error
packet contains this error.
The device detects multi-bit
ECC errors in at least one of
the received packets.

The device does not interpret the packets
transmitted after the last acknowledge
request.
If the packets are critical, transmit the
packets again.
If this error is recurrent, carefully analyze
the connectivity between the Host and the
device.

AE8

The acknowledge with error
packet contains this error.
The device detects and corrects
the 1 bit ECC error in at least
one of the received packets.

No action is required.
The device acknowledges the packet.
If this error is recurrent, analyze the signal
integrity or the noise conditions of the link.

The acknowledge with error
packet contains this error.
The device detects the line
Contention through LP0/LP1
detection.

This error might corrupt the low-power data
reception and transmission.
Ignore the packets and transmit them
again. The device recovers automatically.
If this error is recurrent, check the device
capabilities and the connectivity between
the Host and device.
Refer to.section 7.2.1 of the DSI
Specification 1.1.

The acknowledge with error
packet contains this error.
The device detects the false
control error.

The device detects one of the following:
– The LP-10 (LP request) is not followed
by the remainder of a valid escape or
turnaround sequence.
– The LP-01 (HS request) is not followed
by a bridge state (LP-00).
The D-PHY communications are corrupted.
This error is unrecoverable.
Reset the DSI Host and the D-PHY.
Refer to the section 7.1.6 of the DSI
Specification 1.1.

AE10

AE7

AE6

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456
Table 447. Error causes and recovery (continued)

DSI Host interrupt
and status register

0

0

0

0

0

Bit

5

4

3

2

1

Error cause

Recommended method
to handle the error

AE5

The acknowledge with error
packet contains this error.
The display timeout counters
for a HS reception and LP
transmission expire.

It is possible that the Host and device
timeout counters are not correctly
configured. The device HS_TX timeout
must be shorter than the Host HS_RX
timeout. Host LP_RX timeout must be
longer than the device LP_TX timeout.
Check and confirm that the Host
configuration is consistent with the device
specifications. This error is automatically
recovered, although there is no guarantee
that all the packets in the transmission or
reception are complete. For additional
information about this error, see section
7.2.2 of the DSI Specification 1.1.

AE4

The acknowledge with error
packet contains this error.
The device reports that the
LPDT is not aligned in an 8-bit
boundary

There is no guarantee that the device
properly receives the packets.
Transmit the packets again. For additional
information about this error, see section
7.1.5 of the DSI Specification.

AE3

The acknowledge with error
packet contains this error.
The device does not recognize
the escape mode entry
command.

The device does not recognize the escape
mode entry code.
Check the device capability. For additional
information about this error, see section
7.1.4 of the DSI Specification.
Repeat the transmission to the device.

AE2

The acknowledge with error
packet contains this error.
The device detects the HS
transmission did not end in an
8-bit boundary when the EoT
sequence is detected.

There is no guarantee that the device
properly received the packets. Retransmission must be performed. Transmit
the packets again. For additional
information about this error, see section
7.1.3 of the DSI Specification 1.1.

AE1

The device discards the incoming
The acknowledge with error
transmission. Re-transmission must be
packet contains this error.
performed by the Host. For additional
The device detects that the SoT
information about this error, see section
leader sequence is corrupted.
7.1.2 of the DSI Specification 1.1.
The device is tolerant to single bit and
some multi-bit errors in the SoT sequence
but the packet correctness is
compromised. If the packet content was
important, transmit the packets again. For
additional information about this error, see
section 7.1.1 of the DSI Specification 1.1.
The LTDC frequency is too slow compared
to the DSI bandwidth. Increase the LTDC
frequency or decrease the DSI bandwidth.

Name

0

0

AE0

The acknowledge with error
packet contains this error.
The device reports that the SoT
sequence is received with
errors but synchronization can
still be achieved.

1

19

PBUE

An underflow occurs in the
LTDC payload buffer.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)
Table 447. Error causes and recovery (continued)

DSI Host interrupt
and status register

1

1

1

1

1

1

1

Bit

12

11

10

9

8

7

6

Name

Error cause

Recommended method
to handle the error

An overflow occurs in the
generic read FIFO.

The read FIFO size is not correctly
dimensioned for the maximum read-back
packet size.
Configure the device to return the read
data with a suitable size for the Host
dimensioned FIFO. Data stored in the FIFO
is corrupted.
Reset the DSI Host and repeat the read
procedure.

An underflow occurs in the
generic read FIFO.

System does not wait for the read
procedure to end and starts retrieving the
data from the FIFO. The read data is
requested before it is fully received. Data is
corrupted.
Reset the DSI Host and repeat the read
procedure. Check that the read procedure
is completed before reading the data
through the APB interface.

An underflow occurs in the
generic write payload FIFO.

The system writes the packet header
before the respective packet payload is
completely loaded into the payload FIFO.
This error is unrecoverable, the transmitted
packet is corrupted.
Reset the DSI Host and repeat the write
procedure.

An overflow occurs in the
generic write payload FIFO.

The payload FIFO size is not correctly
dimensioned to store the total payload of a
long packet. Data stored in the FIFO is
corrupted.
Reset the DSI Host and repeat the write
procedure.

An overflow occurs in the
generic command FIFO.

The command FIFO size is not correctly
dimensioned to store the total headers of a
burst of packets. Data stored in the FIFO is
corrupted.
Reset the DSI Host and repeat the write
procedure.

LPWRE

An overflow occurs in the DPI
pixel payload FIFO.

The controller FIFO dimensions are not
correctly set up for the operating resolution.
Check the video mode configuration
registers. They must be consistent with the
LTDC video resolution. The pixel data
sequence is corrupted.
Reset the DSI Host and re-initiate the
Video transmission.

EOTPE

This error is not critical for the data integrity
Host receives a transmission
of the received packets.
that does not end with an end of
Check if the device supports the
transmission packet.
transmission of EoTp packets.

GPRXE

GPRDE

GPTXE

GPWRE

GCWRE

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456
Table 447. Error causes and recovery (continued)

DSI Host interrupt
and status register

1

Bit

5

Name

Error cause

Recommended method
to handle the error

PSE

Host receives a transmission
that does not end in the
expected by boundaries.

The integrity of the received data cannot be
guaranteed.
Reset the DSI Host and repeat the read
procedure.

Host reports that a received
long packet has a CRC error in
its payload.

The received payload data is corrupted.
Reset the DSI Host and repeat the read
procedure. If this error is recurrent, check
the DSI connectivity link for the noise
levels.

1

4

CRCE

1

3

Host reports that a received
ECCME packet contains multiple ECC
errors.

The received packet is corrupted. The DSI
Host ignores all the following packets. The
DSI Host must repeat the read procedure.

2

Host reports that a received
ECCSE packet contains a single bit
error.

This error is not critical because the DSI
Host can correct the error and properly
decode the packet.
If this error is recurrent, check the DSI
connectivity link for signal integrity and
noise levels.

1

Once the configured timeout counter ends,
the DSI Host automatically resets the
controller side and recovers to normal
Host reports that the configured operation. Packet transmissions happening
TOLPRX timeout counter for the lowduring this event are lost.
power reception has expired.
If this error is recurrent, check the timer
configuration for any issue. This timer must
be greater than the maximum low-power
transmission generated by the device.

1

0

Once the configured timeout counter ends,
the DSI Host automatically resets the
controller side and recovers to normal
Host reports that the configured
operation. Packet transmissions happening
timeout counter for the highTOHSTX
during this event are lost.
speed transmission has
If this error is recurrent, check the timer
expired.
configuration for any issue. This timer must
be greater than the maximum high-speed
transmission bursts generated by the Host.

DSI Wrapper

10

PLLUF

1

1

<!-- pagebreak -->

The PLL of the D-PHY has
unlocked.

RM0456 Rev 6

This error can be critical.
The graphical subsystem must be
reconfigured and restarted.

RM0456

44.14

DSI Host (DSI)

Programing procedure
To operate the DSI Host the user must be familiar with the MIPI® DSI specification. Every
software programmable register is accessible through the APB interface.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

44.14.1

RM0456

Programing procedure overview
The procedure for video mode or adapted command mode must respect the following order:
1.

2.

Configure the LTDC (refer to LTDC section)
a)

Configure RCC for LTDC

b)

Configure the LTDC PLL, turn it ON, and wait for its lock

c)

Program the panel timings

d)

Enable the relevant layers

Configure the RCC for DSI (refer to RCC section)
a)

Configure PLL3, turn it on, and wait for its lock

b)

Keep the lane byte clock on PLL3 up and running

c)

Enable the clock for DSI

3.

Optionally, configure the GPIO (if tearing effect requires GPIO usage for example or
reset for display)

4.

Optionally, reset display (when the display needs to be reset in LP-00)

5.

Optionally, validate the ISR

6.

Turn on the DSI bias (DSI_BCGGR.PWRUP = 1)

7.

Configure the DSI PLL, turn it ON, and wait for its lock as described in Section 44.12.4

8.

Enable DSI Host (DSI_CR.EN = 1)

9.

Configure clock division factors (DSI_CCR.TXECKDIV ≠ 0x0) to start tx_clk_esc clock
(this clock is required to configure the D-PHY)

10. Configure the D-PHY parameters in the DSI Host, the DSI D-PHY, and in the DSI
Wrapper to define D-PHY configuration and timing as described in Section 44.14.2
11. Enable the D-PHY setting the DEN bit of the DSI_PCTLR
12. Configure the band control (BC fields) and the slew rate (SRC fields) for the clock lane
and the data lanes in DSI_DPCBCR, DSI_DPDLxBCD, DSI_DPCSRCR, and
DSI_DPDLxSRCR
13. Enable the D-PHY clock lane setting the CKEN bit of the DSI_PCTLR
14. Switch the lane byte clock from PLL3 to D-PHY PLL in the RCC
15. Wait for the D-PHY to be in STOP state checking the bits:
a)

the clock lane stop state (PSSC) in the DSI_PSR

b)

data lanes stop state (PSS1/PSS0) in the DSI_PSR

16. Disable the DSI Host (DSI_CR.EN = 0)
17. Turn on clock lane in high speed as no automatic control (DPCC)
18. Configure the DSI Host timings as detailed in Section 44.14.3
19. Configure the DSI Host flow control and DBI interface as detailed in Section 44.14.4
20. Configure the DSI Host LTDC interface as detailed in Section 44.14.5
21. Configure the DSI Host for video mode as detailed in Section 44.14.6 or adapted
command mode as detailed in Section 44.14.7
22. Optionally, reset display (when the display needs to be reset in LP-11)
23. Start the DSI (DSI_CR.EN = 1 and DSI_WCR.DSIEN = 1)
24. Optionally, send DCS commands through the APB generic interface to configure the
display
25. Enable the LTDC in the LTDC

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)
26. Start the LTDC flow through the DSI Wrapper (DSI_WCR.LTDCEN = 1)

44.14.2

a)

In video mode, the data streaming starts as soon as the LTDC is enabled

a)

In adapted command mode, the frame buffer update is launched as soon as the
DSI_WCR.LTDCEN bit is set

Configuring the D-PHY parameters
The D-PHY requires a specific configuration prior starting any communications. The
configuration parameters are stored either in the DSI Host or the DSI Wrapper.

Configuring the D-PHY parameters
The D-PHY must be configured to adjust:
•

the slew rate for the clock lane through register DSI_DPCSRC

•

the slew rate for the data lanes through DSI_DPDL0SRC and DSIDPDL1SRC

•

the frequency band for the clock lane through DSI_DPCLBCR

•

the frequency band for the data lanes through DSI_DPDL0BCR and DSI_DPBL1BCR

Configuring the D-PHY parameters in the DSI Host
The DSI Host stores the configuration of D-PHY timing parameters and number of lanes.
The following fields must be configured prior to any startup:
•

44.14.3

Number of data lanes in the DSI_PCONFR register

•

Automatic clock lane control (ACR) in the DSI_CLCR register

•

Clock control (DPCC) in the DSI_CLCR register

•

Time for LP/HS and HS/LP transitions for both clock lane and data lanes in
DSI_CLTCR and DSI_DLTCR registers

•

Stop wait time in the DSI_PCONFR register

Configuring the DSI Host timing
All the protocol timing must be configured in the DSI Host.

Clock divider configuration
Two clocks are generated internally, namely the timeout clock and the TX escape clock.
The timeout clock is used as the timing unit in the configuration of HS to LP and LP to HS
transition error. Its division factor is configured by the timeout clock division (TOCKDIV) field
of the DSI Host clock control register (DSI_CCR).
The TX escape clock is used in low-power transmission. Its division factor is configured by
the TX escape clock division (TXECKDIV) field of the DSI Host clock control register
(DSI_CCR) relatively to the lanebyteclock. Its typical value must be around 20 MHz.

Timeout configuration
The timings for timeout management as described in Section 44.8 are configured in the DSI
Host timeout counter configuration registers (DSI_TCCR0 to DSI_TCCR5).

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

44.14.4

RM0456

Configuring flow control and DBI interface
The flow control is configured thanks to the DSI Host protocol configuration register
(DSI_PCR). The configuration parameters are the following
•

CRC reception enable (CRCRXE bit)

•

ECC reception enable (ECCRXE bit)

•

BTA enable (BTAE bit)

•

EoTp reception enable (ETRXE bit)

•

EoTp transmission enable (ETTXE bit)

Their values depend upon the protocol to be used for the communication with the DSI
display.
The virtual channel ID used for the generic DBI interface must be configured by the virtual
channel ID (VCID) field of the DSI Host generic VCID register (DSI_GVCIDR).
All the DCS command, depending on their type, can be transmitted or received either in
high-speed or low-power. For each of them, a dedicated configuration bit must be
programmed in the DSI Host command mode configuration register (DSI_CMCR).
Acknowledge request for packet or tearing effect event must also be configured in the DSI
Host command mode configuration register (DSI_CMCR).

44.14.5

Configuring the DSI Host LTDC interface
As the DSI Host is interface to the system through the LTDC for video mode or adapted
command mode, the DSI Wrapper performs a low level interfacing in between.
The parameter programmed into the DSI Wrapper must be aligned with the parameters
programmed into the LTDC and the DSI Host.
The following fields must be configured:

<!-- pagebreak -->

•

Virtual channel ID in the virtual channel ID (VCID) field of the DSI Host LTDC VCID
register (DSI_LVCIDR).

•

Color coding (COLC) field of the DSI Host LTDC color coding register (DSI_LCOLCR)
and the color multiplexing (COLMUX) in the DSI Wrapper configuration register
(DSI_WCFGR).

•

If loose packets are used for 18-bit mode, the loosely packet enable (LPE) bit of the
DSI Host LTDC color coding register (DSI_LCOLCR) must be set.

RM0456 Rev 6

RM0456

44.14.6

DSI Host (DSI)

Configuring the video mode
The video mode configuration defines the behavior of the controller in low-power for
command transmission, the type of video transmission (burst or non-burst mode) and the
panel horizontal and vertical timing:
•

•

Select the video transmission mode to define how the processor requires the video line
to be transported through the DSI link.
–

Configure the low-power transitions in the DSI_VMCR to define the video periods,
which are permitted to go to low-power if there is time available to do so.

–

Configure if the controller must request the peripheral acknowledge message at
the end of frames (DSI_VMCR.FBTAAE).

–

Configure if commands are to be transmitted in low-power (DSI_VMCR.LPE).

Select the video mode type
–

Burst mode:
Configure the video mode type (DSI_VMCR.VMT) with value 2'b1x.
Configure the video packet size (DSI_VPCR.VPSIZE) with the size of the active
line period, measured in pixels.
The registers DSI_VCCR and DSI_VNPCR are ignored by the DSI Host.

–

Non-burst mode:
Configure the video mode type (DSI_VMCR.VMT) with 2'b00 to enable the
transmission of sync pulses or with 2'b01 to enable the transmission of sync
events.
Configure the video packet size (DSI_VPCR.VPSIZE) with the number of pixels to
be transmitted in a single packet. Selecting this value depends on the available
memory of the attached peripheral, if the data is first stored, or on the memory you
want to select for the FIFO in DSI Host.
Configure the number of chunks (DSI_VCCR.NUMC) with the number of packets
to be transmitted per video line. The value of VPSIZE * NUMC is the number of
pixels per line of video, except if NUMC is 0, which disables the multi-packets. If
you set it to 1, there is still only one packet per line, but it can be part of a chunk,
followed by a null packet.
Configure the null packet size (DSI_VNPCR.NPSIZE) with the size of null packets
to be inserted as part of the chunks. Setting it to 0 disables null packets.

•

Define the video horizontal timing configuration as follows:
–

Configure the horizontal line time (DSI_VLCR.HLINE) with the time taken by an
LTDC video line measured in cycles of lane byte clock (for a clock lane at 500 MHz
the lane byte clock period is 8 ns). When the periods of LTDC clock and lane byte
clock are not multiples, the value to program the DSI_VLCR.HLINE needs to be
rounded. A timing mismatch is introduced between the lines due to the rounding of
configuration values. If the DSI Host is configured not to go to low-power, this
timing divergence accumulates on every line, introducing a significant amount of
mismatch towards the end of the frame. The reason for this is that the DSI Host
cannot resynchronize on every new line because it transmits the blanking packets
when the horizontal sync event occurs on the LTDC interface. However, the
accumulated mismatch must become extinct on the last line of a frame, where,
according to the DSI specification, the link must always return to low-power
regaining synchronization, when a new frame starts on a vertical sync event. If the
accumulated timing mismatch is greater than the time in low-power on the last

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456
line, a malfunction occurs. This phenomenon can be avoided by configuring the
DSI Host to go to low-power once per line.

•

<!-- pagebreak -->

–

Configure the horizontal sync duration (DSI_VHSACR.HSA) with the time taken by
an LTDC horizontal sync active period measured in cycles of lane byte clock
(normally a period of 8 ns).

–

Configure the horizontal back-porch duration (DSI_VHBPCR.HBP) with the time
taken by the LTDC horizontal back-porch period measured in cycles of lane byte
clock (normally a period of 8 ns). Special attention must be given to the calculation
of this parameter.

Define the vertical line configuration:
–

Configure the vertical sync duration (DSI_VVSACR.VSA) with the number of lines
existing in the LTDC vertical sync active period.

–

Configure the vertical back-porch duration (DSI_VVBPCR.VBP) with the number
of lines existing in the LTDC vertical back-porch period.

–

Configure the vertical front-porch duration (DSI_VVFPCR.VFP) with the number of
lines existing in the LTDC vertical front-porch period.

–

Configure the vertical active duration (DSI_VVACR.VA) with the number of lines
existing in the LTDC vertical active period.

RM0456 Rev 6

RM0456

DSI Host (DSI)
Figure 446 illustrates the steps for configuring the DPI packet transmission.
Figure 446. Video packet transmission configuration flow diagram
Global configuration
Configure the DPI I/F

NO

YES

Burst Mode

Determine the
DSI link to pixel ratio

Configure
video_packet_size

Enable
multiple packets
YES

If the DSI link to
pixel ratio is >1
NO

Determine number of pixel per packet
Calculate the number of chunks
Determine the chunk overhead
(needs to be  12 or = 6)

Determine number
of pixel per packet

If the DSI chunk
overhead is  12

Enable
null packets
NO
YES

Calculate:
Hline_time - Hsa_time - Hbp_time

Null packet size

Configure:
VSA lines- VBP lines - Vact lines - VFP lines
MSv35876V1

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Example of video configuration
The following is an example of video packet transmission configuration:
Video resolution:
•

PCLK period = 50 ns

•

HSA = 8 PCLK

•

HBP = 8 PCLK

•

HACT = 480 PCLK

•

HFP = 24 PCLK

•

VSA = 2 lines

•

VBP = 2 lines

•

VACT = 640 lines

•

VFP = 4 lines

Configuration steps:
•

Video transmission mode configuration:
a)

Configure the low-power transitions:
DSI_VMCR[13:8] = 6'b111111, to enable LP in all video period.

b)
•

DSI_VMCR.FBTAAE = 1, for the DSI Host to request an acknowledge response
message from the peripheral at the end of each frame.

To use the burst mode, follow these steps:
DSI_VMCR.VMT = 2'b1x
DSI_VPCR.VPSIZE = 480

•

•

<!-- pagebreak -->

Horizontal timing configuration:
–

DSI_VLCR.HLINE =
(HSA + HBP + HACT + HFP) * (PCLK period / Clk lane byte period) =
(8 + 8 + 480 + 24) * (50 / 8) = 3250

–

DSI_VHSACR.HSA = HSA * (PCLK period/Clk lane byte period) =
8 * (50 / 8) = 50

–

DSI_VHBPCR.HBP = HBP * (PCLK period / Clk lane byte period) =
8 * (50 / 8) = 50

Vertical line configuration:
–

DSI_VVSACR.VSA = 2

–

DSI_VVBPCR.VBP = 2

–

DSI_VVFPCR.VFP = 4

–

DSI_VVACR.VA = 640

RM0456 Rev 6

RM0456

44.14.7

DSI Host (DSI)

Configuring the adapted command mode
The adapted command mode requires the following parameters to be configured:

44.14.8

•

Command size (CMDSIZE) field of the DSI Host LTDC command configuration register
(DSI_LCCR) to define the maximum allowed size for a write memory command.

•

The tearing effect source (TESRC) and optionally tearing effect polarity (TEPOL) bits of
the DSI Wrapper configuration register (DSI_WCFGR).

•

The automatic refresh (AR) bit of the DSI Wrapper configuration register
(DSI_WCFGR) if the display needs to be updated automatically each time a tearing
effect event is received.

Configuring the video mode pattern generator
DSI Host can transmit a color bar pattern without horizontal/vertical color bar and D-PHY
BER testing pattern without any kind of stimuli.
Figure 447 shows the programming sequence to send a test pattern:
1.

Configure the DSI_MCR register to enable video mode. Configure the video mode type
using DSI_VMCR.VMT.

2.

Configure the DSI_LCOLCR register.

3.

Configure the frame using registers shown in Figure 448 (where the gray area
indicates the transferred pixels).

4.

Configure the pattern generation mode (DSI_VMCR.PGM) and the pattern orientation
(DSI_VMCR.PGO), and enable them (DSI_VMCR.PGE).
Figure 447. Programming sequence to send a test pattern

Video Mode
selection

Color coding
configuration
Video frame
configuration
Video pattern generator
configuration

MSv35875V1

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456
Figure 448. Frame configuration registers
DSI_VVSACR.VSA (Line)

DSI_VVACR.VA (Line)

DSI_VVBPCR.VBP (Line)

DSI_VHSACR.HSA
(lanebyteclk)

DSI_VHBPCR.HBP
(lanebyteclk)

DSI_VPCR.VPSIZE (Pixel) * DSI_VCCR.NUMC

HFP
(lanebyteclk)

DSI_VVFPCR.VFP (Line)

DSI_VLCR.HLINE (lanebyteclk)
MSv35877V1

Note:

The number of pixels of payload is restricted to a multiple of a value provided in Table 438.

44.14.9

Managing ULPM
There are two ways to configure the software to enter and exit the ULPM:
•

enter and exit the ULPM with the D-PHY PLL running (a faster process)

•

enter and exit the ULPM with the D-PHY PLL turned off (a more efficient process in
terms of power consumption).

Clock management for ULPM sequence
The ULPM management state machine is working on the lanebyteclock provided by the
D-PHY.
Because the D-PHY is providing the lanebyteclock only when the clock lane is not in ULPM
state, it is mandatory to switch the lanebyteclock source of the DSI Host before starting the
ULPM mode entry sequence.
The lanebyteclock source is controlled by the RCC. It can be

<!-- pagebreak -->

•

the lanebyteclock provided by the D-PHY (for all modes except ULPM)

•

a clock generated by the system PLL (for ULPM)

RM0456 Rev 6

RM0456

DSI Host (DSI)

Process flow to enter the ULPM
Implement the process described in detail in the following procedure to enter the ULPM on
both clock lane and data lanes:
1.

Verify the initial status of the DSI Host:
–

DSI_PCTLR[2:1] = 2’h3

–

DSI_WRPCR.PLLEN = 1’h1 and DSI_WRPCR.REGEN = 1’h1

–

DSI_PUCR[3:0] = 4'h0

–

DSI_PTTCR[3:0] = 4'h0

–

Verify that all active lanes are in Stop state and the D-PHY PLL is locked:
One-lane configuration: DSI_PSR[6:4] = 3'h3 and DSI_PSR[1] = 1’h0 and
DSI_WISR.PLLS = 1’h1
Two-lanes configuration: DSI_PSR[8:4] = 5'h1B and DSI_PSR[1] = 1’h0 and
DSI_WISR.PLLS = 1’h1

2.

Switch the lanebyteclock source in the RCC from D-PHY to system PLL

3.

Set DSI_PUCR[3:0] = 4'h5 to enter ULPM in the data and the clock lanes.

4.

Wait until the D-PHY active lanes enter into ULPM:
–

One-lane configuration: DSI_PSR[6:1] = 6'h00

–

Two-lanes configuration: DSI_PSR[8:1] = 8'h00

The DSI Host is now in ULPM.
5.

Turn off the D-PHY PLL by setting DSI_WRPCR.PLLEN = 1'b0

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Process flow to exit the ULPM
Implement the process flow described in the following procedure to exit the ULPM on both
clock lane and data lanes:
1.

Verify that all active lanes are in ULPM:
–

One-lane configuration: DSI_PSR[6:1] = 6'h00

–

Two-lanes configuration: DSI_PSR[8:1] = 8'h00

2.

Turn on the D-PHY PLL by setting DSI_WRPCR.PLLEN = 1'b1.

3.

Wait until D-PHY PLL locked
–

DSI_WISR.PLLS = 1'b1

4.

Without de-asserting the ULPM request bits, assert the exit ULPM bits by setting
DSI_PUCR[3:0] = 4'hF.

5.

Wait until all active lanes exit ULPM:
–

One-lane configuration:
DSI_PSR[5] = 1'b1
DSI_PSR[3] = 1'b1

–

Two-lanes configuration:
DSI_PSR[8] = 1'b1
DSI_PSR[5] = 1'b1
DSI_PSR[3] = 1'b1

6.

Wait for 1 ms.

7.

De-assert the ULPM requests and the ULPM exit bits by setting DSI_PUCR [3:0] =
4'h0.

8.

Switch the lanbyteclock source in the RCC from system PLL to D-PHY

9.

The DSI Host is now in Stop state and the D-PHY PLL is locked:
–

One-lane configuration:
DSI_PSR[6:4] = 3'h3
DSI_PSR[1] = 1'h0
DSI_WRPCR.PLLEN = 1'b1

–

Two-lanes configuration:
DSI_PSR[8:4] = 5'h1B
DSI_PSR[1] = 1'h0
DSI_WRPCR.PLLEN = 1'b1

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

44.15

DSI Host registers

44.15.1

DSI Host version register (DSI_VR)
Address offset: 0x0000
Reset value: 0x3134 312A

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

17

16

VERSION[31:16]
r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

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

5

4

3

2

1

0

r

r

r

r

r

r

r

r

r

r

r

r

r

r

VERSION[15:0]
r

r

Bits 31:0 VERSION[31:0]: Version of the DSI Host
This read-only register contains the version of the DSI Host

44.15.2

DSI Host control register (DSI_CR)
Address offset: 0x0004
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

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

Res.

Res.

Res.

Res.

EN
rw

Bits 31:1 Reserved, must be kept at reset value.
Bit 0 EN: Enable
This bit configures the DSI Host in either power-up mode or to reset.
0: DSI Host disabled (under reset)
1: DSI Host enabled

44.15.3

DSI Host clock control register (DSI_CCR)
Address offset: 0x0008
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

rw

rw

rw

TOCKDIV[7:0]
rw

rw

rw

rw

rw

TXECKDIV[7:0]
rw

rw

rw

rw

RM0456 Rev 6

rw

rw

rw

rw

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:8 TOCKDIV[7:0]: Timeout clock division
This field indicates the division factor for the timeout clock used as the timing unit in the
configuration of HS to LP and LP to HS transition error.
Bits 7:0 TXECKDIV[7:0]: TX escape clock division
This field indicates the division factor for the TX escape clock source (lanebyteclk). The
values 0 and 1 stop the TX_ESC clock generation.

44.15.4

DSI Host LTDC VCID register (DSI_LVCIDR)
Address offset: 0x000C
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

17

16

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

Res.

Res.

Res.

Res.

Res.

1

0

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

5

4

3

2

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

Res.

Res.

Res.

VCID[1:0]
rw

rw

Bits 31:2 Reserved, must be kept at reset value.
Bits 1:0 VCID[1:0]: Virtual channel ID
These bits configure the virtual channel ID for the LTDC interface traffic.

44.15.5

DSI Host LTDC color coding register (DSI_LCOLCR)
Address offset: 0x0010
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

17

16

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

Res.

Res.

Res.

Res.

Res.

3

2

1

0

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

5

4

Res.

Res.

Res.

Res.

Res.

Res.

Res.

LPE

Res.

Res.

Res.

Res.

rw

Bits 31:9 Reserved, must be kept at reset value.
Bit 8 LPE: Loosely packet enable
This bit enables the loosely packed variant to 18-bit configuration
0: Loosely packet variant disabled
1: Loosely packet variant enabled
Bits 7:4 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

COLC[3:0]
rw

rw

rw

rw

RM0456

DSI Host (DSI)

Bits 3:0 COLC[3:0]: Color coding
This field configures the DPI color coding.
0000: 16-bit configuration 1
0001: 16-bit configuration 2
0010: 16-bit configuration 3
0011: 18-bit configuration 1
0100: 18-bit configuration 2
0101: 24-bit
Others: Reserved

44.15.6

DSI Host LTDC polarity configuration register (DSI_LPCR)
Address offset: 0x0014
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

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

Res.

Res.

HSP

VSP

DEP

rw

rw

rw

Bits 31:3 Reserved, must be kept at reset value.
Bit 2 HSP: HSYNC polarity
This bit configures the polarity of HSYNC pin. It is recommanded to keep the default polarity
configuration to guarantee a correct behavior.
0: HSYNC pin active high (default)
1: HSYNC pin active low
Bit 1 VSP: VSYNC polarity
This bit configures the polarity of VSYNC pin. It is recommanded to keep the default polarity
configuration to guarantee a correct behavior.
0: VSYNC pin active high (default)
1: VSYNC pin active low
Bit 0 DEP: Data enable polarity
This bit configures the polarity of data enable pin. It is recommanded to keep the default
polarity configuration to guarantee a correct behavior.
0: Data enable pin active high (default)
1: Data enable pin active low

44.15.7

DSI Host low-power mode configuration register (DSI_LPMCR)
Address offset: 0x0018
Reset value: 0x0000 0000

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

31

30

29

28

27

26

25

24

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

23

22

21

20

rw

rw

rw

rw

7

6

5

4

19

18

17

16

rw

rw

rw

rw

3

2

1

0

rw

rw

rw

LPSIZE[7:0]

VLPSIZE[7:0]
rw

rw

rw

rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:16 LPSIZE[7:0]: Largest packet size
This field is used for the transmission of commands in low-power mode. It defines the size, in
bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions.
Bits 15:8 Reserved, must be kept at reset value.
Bits 7:0 VLPSIZE[7:0]: VACT largest packet size
This field is used for the transmission of commands in low-power mode. It defines the size, in
bytes, of the largest packet that can fit in a line during VACT regions.

44.15.8

DSI Host protocol configuration register (DSI_PCR)
Address offset: 0x002C
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

17

16

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

Res.

Res.

Res.

Res.

Res.

5

4

3

2

1

0

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

Res.

Res.

Res.

Res.

Res.

ETTXLPE CRCRXE ECCRXE
rw

rw

Bits 31:6 Reserved, must be kept at reset value.
Bit 5 ETTXLPE: EoTp transmission in low-power enable
This bit enables the EoTP transmission in low-power.
0: EoTp transmission in low-power is disabled.
1: EoTp transmission in low-power is enabled.
Bit 4 CRCRXE: CRC reception enable
This bit enables the CRC reception and error reporting.
0: CRC reception is disabled.
1: CRC reception is enabled.
Bit 3 ECCRXE: ECC reception enable
This bit enables the ECC reception, error correction and reporting.
0: ECC reception is disabled.
1: ECC reception is enabled.
Bit 2 BTAE: Bus-turn-around enable
This bit enables the bus-turn-around (BTA) request.
0: Bus-turn-around request is disabled.
1: Bus-turn-around request is enabled.

<!-- pagebreak -->

RM0456 Rev 6

rw

BTAE ETRXE ETTXE
rw

rw

rw

RM0456

DSI Host (DSI)

Bit 1 ETRXE: EoTp reception enable
This bit enables the EoTp reception.
0: EoTp reception is disabled.
1: EoTp reception is enabled.
Bit 0 ETTXE: EoTp transmission enable
This bit enables the EoTP transmission.
0: EoTp transmission is disabled.
1: EoTp transmission is enabled.

44.15.9

DSI Host generic VCID register (DSI_GVCIDR)
Address offset: 0x0030
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

17

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

Res.

Res.

Res.

VCIDTX[1:0]

16

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

5

4

3

2

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

Res.

Res.

Res.

rw

rw

1

0

VCIDRX[1:0]
rw

rw

Bits 31:18 Reserved, must be kept at reset value.
Bits 17:16 VCIDTX[1:0]: Virtual channel ID for transmission
This field indicates the generic interface virtual channel identification where the generic
packet is automatically generated and transmitted.
Bits 15:2 Reserved, must be kept at reset value.
Bits 1:0 VCIDRX[1:0]: Virtual channel ID for reception
This field indicates the generic interface read-back virtual channel identification.

44.15.10 DSI Host mode configuration register (DSI_MCR)
Address offset: 0x0034
Reset value: 0x0000 0001
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

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

Res.

Res.

Res.

Res.

CMDM
rw

Bits 31:1 Reserved, must be kept at reset value.
Bit 0 CMDM: Command mode
This bit configures the DSI Host in either video or command mode.
0: DSI Host is configured in video mode.
1: DSI Host is configured in command mode.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

44.15.11 DSI Host video mode configuration register (DSI_VMCR)
Address offset: 0x0038
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

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PGO

Res.

Res.

Res.

PGM

Res.

Res.

Res.

PGE

15

14

13

12

11

10

9

7

6

5

4

3

2

1

Res.

Res.

Res.

Res.

Res.

Res.

rw
8

LPCE FBTAAE LPHFPE LPHBPE LPVAE LPVFPE LPVBPE LPVSAE
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

0

VMT[1:0]
rw

rw

Bits 31:25 Reserved, must be kept at reset value.
Bit 24 PGO: Pattern generator orientation
This bit configures the color bar orientation.
0: Vertical color bars.
1: Horizontal color bars.
Bits 23:21 Reserved, must be kept at reset value.
Bit 20 PGM: Pattern generator mode
This bit configures the pattern generator mode.
0: Color bars (horizontal or vertical).
1: BER pattern (vertical only).
Bits 19:17 Reserved, must be kept at reset value.
Bit 16 PGE: Pattern generator enable
This bit enables the video mode pattern generator.
0: Pattern generator is disabled.
1: Pattern generator is enabled.
Bit 15 LPCE: Low-power command enable
This bit enables the command transmission only in low-power mode.
0: Command transmission in low-power mode is disabled.
1: Command transmission in low-power mode is enabled.
Bit 14 FBTAAE: Frame bus-turn-around acknowledge enable
This bit enables the request for an acknowledge response at the end of a frame.
0: Acknowledge response at the end of a frame is disabled.
1: Acknowledge response at the end of a frame is enabled.
Bit 13 LPHFPE: Low-power horizontal front-porch enable
This bit enables the return to low-power inside the horizontal front-porch (HFP) period when
timing allows.
0: Return to low-power inside the HFP period is disabled.
1: Return to low-power inside the HFP period is enabled.
Bit 12 LPHBPE: Low-power horizontal back-porch enable
This bit enables the return to low-power inside the horizontal back-porch (HBP) period when
timing allows.
0: Return to low-power inside the HBP period is disabled.
1: Return to low-power inside the HBP period is enabled.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

Bit 11 LPVAE: Low-power vertical active enable
This bit enables to return to low-power inside the vertical active (VACT) period when timing
allows.
0: Return to low-power inside the VACT is disabled.
1: Return to low-power inside the VACT is enabled.
Bit 10 LPVFPE: Low-power vertical front-porch enable
This bit enables to return to low-power inside the vertical front-porch (VFP) period when
timing allows.
0: Return to low-power inside the VFP is disabled.
1: Return to low-power inside the VFP is enabled.
Bit 9 LPVBPE: Low-power vertical back-porch enable
This bit enables to return to low-power inside the vertical back-porch (VBP) period when
timing allows.
0: Return to low-power inside the VBP is disabled.
1: Return to low-power inside the VBP is enabled.
Bit 8 LPVSAE: Low-power vertical sync active enable
This bit enables to return to low-power inside the vertical sync time (VSA) period when timing
allows.
0: Return to low-power inside the VSA is disabled.
1: Return to low-power inside the VSA is enabled
Bits 7:2 Reserved, must be kept at reset value.
Bits 1:0 VMT[1:0]: Video mode type
This field configures the video mode transmission type :
00: Non-burst with sync pulses.
01: Non-burst with sync events.
1x: Burst mode

44.15.12 DSI Host video packet configuration register (DSI_VPCR)
Address offset: 0x003C
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

rw

rw

rw

rw

rw

rw

VPSIZE[13:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:14 Reserved, must be kept at reset value.
Bits 13:0 VPSIZE[13:0]: Video packet size
This field configures the number of pixels in a single video packet.
For 18-bit not loosely packed data types, this number must be a multiple of 4.
For YCbCr data types, it must be a multiple of 2 as described in the DSI specification.

44.15.13 DSI Host video chunks configuration register (DSI_VCCR)
Address offset: 0x0040

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

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

17

16

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

Res.

Res.

Res.

Res.

Res.

12

11

10

9

8

7

6

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

rw

15

14

13

Res.

Res.

Res.

NUMC[12:0]
rw

rw

rw

rw

rw

rw

rw

Bits 31:13 Reserved, must be kept at reset value.
Bits 12:0 NUMC[12:0]: Number of chunks
This register configures the number of chunks to be transmitted during a line period (a chunk
consists of a video packet and a null packet).
If set to 0 or 1, the video line is transmitted in a single packet.
If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise,
multiple chunks are used to transmit each video line.

44.15.14 DSI Host video null packet configuration register (DSI_VNPCR)
Address offset: 0x0044
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

17

16

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

Res.

Res.

Res.

Res.

Res.

12

11

10

9

8

7

6

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

rw

15

14

13

Res.

Res.

Res.

NPSIZE[12:0]
rw

rw

rw

rw

rw

rw

rw

Bits 31:13 Reserved, must be kept at reset value.
Bits 12:0 NPSIZE[12:0]: Null packet size
This field configures the number of bytes inside a null packet.
Setting to 0 disables the null packets.

44.15.15 DSI Host video HSA configuration register (DSI_VHSACR)
Address offset: 0x0048
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

17

16

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

Res.

Res.

Res.

Res.

Res.

11

10

9

8

7

6

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

15

14

13

12

Res.

Res.

Res.

Res.

HSA[11:0]
rw

rw

rw

rw

rw

Bits 31:12 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

rw

rw

RM0456

DSI Host (DSI)

Bits 11:0 HSA[11:0]: Horizontal synchronism active duration
This fields configures the horizontal synchronism active period in lane byte clock cycles.

44.15.16 DSI Host video HBP configuration register (DSI_VHBPCR)
Address offset: 0x004C
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

Res.

Res.
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

HBP[11:0]
rw

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 HBP[11:0]: Horizontal back-porch duration
This fields configures the horizontal back-porch period in lane byte clock cycles.

44.15.17 DSI Host video line configuration register (DSI_VLCR)
Address offset: 0x0050
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

rw

rw

Res.

HLINE[14:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:15 Reserved, must be kept at reset value.
Bits 14:0 HLINE[14:0]: Horizontal line duration
This fields configures the total of the horizontal line period (HSA+HBP+HACT+HFP) counted
in lane byte clock cycles.

44.15.18 DSI Host video VSA configuration register (DSI_VVSACR)
Address offset: 0x0054
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

17

16

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

Res.

Res.

Res.

Res.

Res.

9

8

7

6

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

15

14

13

12

11

10

Res.

Res.

Res.

Res.

Res.

Res.

VSA[9:0]
rw

rw

rw

RM0456 Rev 6

rw

rw

rw

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Bits 31:10 Reserved, must be kept at reset value.
Bits 9:0 VSA[9:0]: Vertical synchronism active duration
This fields configures the vertical synchronism active period measured in number of
horizontal lines.

44.15.19 DSI Host video VBP configuration register (DSI_VVBPCR)
Address offset: 0x0058
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

17

16

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

Res.

Res.

Res.

Res.

Res.

9

8

7

6

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

15

14

13

12

11

10

Res.

Res.

Res.

Res.

Res.

Res.

VBP[9:0]
rw

rw

rw

rw

rw

rw

Bits 31:10 Reserved, must be kept at reset value.
Bits 9:0 VBP[9:0]: Vertical back-porch duration
This fields configures the vertical back-porch period measured in number of horizontal lines.

44.15.20 DSI Host video VFP configuration register (DSI_VVFPCR)
Address offset: 0x005C
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.
rw

rw

rw

rw

rw

rw

rw

rw

rw

VFP[9:0]
rw

Bits 31:10 Reserved, must be kept at reset value.
Bits 9:0 VFP[9:0]: Vertical front-porch duration
This fields configures the vertical front-porch period measured in number of horizontal lines.

44.15.21 DSI Host video VA configuration register (DSI_VVACR)
Address offset: 0x0060
Reset value: 0x0000 0000

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

rw

rw

rw

rw

rw

rw

VA[13:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:14 Reserved, must be kept at reset value.
Bits 13:0 VA[13:0]: Vertical active duration
This fields configures the vertical active period measured in number of horizontal lines.

44.15.22 DSI Host LTDC command configuration register (DSI_LCCR)
Address offset: 0x0064
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

rw

rw

CMDSIZE[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 CMDSIZE[15:0]: Command size
This field configures the maximum allowed size for an LTDC write memory command,
measured in pixels. Automatic partitioning of data obtained from LTDC is permanently
enabled.

44.15.23 DSI Host command mode configuration register (DSI_CMCR)
Address offset: 0x0068
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

MRDPS

Res.

Res.

Res.

Res.

rw
15
Res.

19

18

17

16

DLWTX DSR0TX DSW1TX DSW0TX
rw

rw

rw

rw

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

GLWTX

GSR
2TX

GSR
1TX

GSR
0TX

GSW
2TX

GSW
1TX

GSW
0TX

Res.

Res.

Res.

Res.

Res.

Res.

ARE

TEARE

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:25 Reserved, must be kept at reset value.
Bit 24 MRDPS: Maximum read packet size
This bit configures the maximum read packet size command transmission type:
0: High-speed
1: Low-power

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Bits 23:20 Reserved, must be kept at reset value.
Bit 19 DLWTX: DCS long write transmission
This bit configures the DCS long write packet command transmission type:
0: High-speed
1: Low-power
Bit 18 DSR0TX: DCS short read zero parameter transmission
This bit configures the DCS short read packet with zero parameter command transmission
type:
0: High-speed
1: Low-power
Bit 17 DSW1TX: DCS short read one parameter transmission
This bit configures the DCS short read packet with one parameter command transmission
type:
0: High-speed
1: Low-power
Bit 16 DSW0TX: DCS short write zero parameter transmission
This bit configures the DCS short write packet with zero parameter command transmission
type:
0: High-speed
1: Low-power
Bit 15 Reserved, must be kept at reset value.
Bit 14 GLWTX: Generic long write transmission
This bit configures the generic long write packet command transmission type:
0: High-speed
1: Low-power
Bit 13 GSR2TX: Generic short read two parameters transmission
This bit configures the generic short read packet with two parameters command transmission
type:
0: High-speed
1: Low-power
Bit 12 GSR1TX: Generic short read one parameters transmission
This bit configures the generic short read packet with one parameters command
transmission type:
0: High-speed
1: Low-power
Bit 11 GSR0TX: Generic short read zero parameters transmission
This bit configures the generic short read packet with zero parameters command
transmission type:
0: High-speed
1: Low-power
Bit 10 GSW2TX: Generic short write two parameters transmission
This bit configures the generic short write packet with two parameters command
transmission type:
0: High-speed
1: Low-power

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

Bit 9 GSW1TX: Generic short write one parameters transmission
This bit configures the generic short write packet with one parameters command
transmission type:
0: High-speed
1: Low-power
Bit 8 GSW0TX: Generic short write zero parameters transmission
This bit configures the generic short write packet with zero parameters command
transmission type:
0: High-speed
1: Low-power
Bits 7:2 Reserved, must be kept at reset value.
Bit 1 ARE: Acknowledge request enable
This bit enables the acknowledge request after each packet transmission:
0: Acknowledge request is disabled.
1: Acknowledge request is enabled.
Bit 0 TEARE: Tearing effect acknowledge request enable
This bit enables the tearing effect acknowledge request:
0: Tearing effect acknowledge request is disabled.
1: Tearing effect acknowledge request is enabled.

44.15.24 DSI Host generic header configuration register (DSI_GHCR)
Address offset: 0x006C
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

WCLSB[7:0]
rw

rw

rw

rw

rw

23

22

21

20

rw

rw

18

17

16

rw

rw

rw

rw

rw

rw

rw

rw

7

6

5

4

3

2

1

0

rw

rw

rw

VCID[1:0]
rw

19

WCMSB[7:0]

rw

rw

DT[5:0]
rw

rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:16 WCMSB[7:0]: WordCount MSB
This field configures the most significant byte of the header packet’s word count for long
packets, or data 1 for short packets.
Bits 15:8 WCLSB[7:0]: WordCount LSB
This field configures the less significant byte of the header packet word count for long
packets, or data 0 for short packets.
Bits 7:6 VCID[1:0]: Channel
This field configures the virtual channel ID of the header packet.
Bits 5:0 DT[5:0]: Type
This field configures the packet data type of the header packet.

44.15.25 DSI Host generic payload data register (DSI_GPDR)
Address offset: 0x0070

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

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

DATA4[7:0]
rw

rw

rw

rw

rw

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

5

DATA2[7:0]
rw

rw

rw

rw

19

18

17

16

DATA3[7:0]
rw

rw

rw

rw

rw

4

3

2

1

0

rw

rw

rw

17

16

DATA1[7:0]

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:24 DATA4[7:0]: Payload byte 4
This field indicates the byte 4 of the packet payload.
Bits 23:16 DATA3[7:0]: Payload byte 3
This field indicates the byte 3 of the packet payload.
Bits 15:8 DATA2[7:0]: Payload byte 2
This field indicates the byte 2 of the packet payload.
Bits 7:0 DATA1[7:0]: Payload byte 1
This field indicates the byte 1 of the packet payload.

44.15.26 DSI Host generic packet status register (DSI_GPSR)
Address offset: 0x0074
Reset value: 0x0005 0015
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

Res.

PBF

PBE

r

r

r

r

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

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

RCB

PRDFF

r

r

PRDFE PWRFF PWRFE CMDFF CMDFE
r

r

Bits 31:20 Reserved, must be kept at reset value.
Bit 19 PBF: Payload buffer full
This bit indicates the full status of the generic payload internal buffer:
0: Payload internal buffer not full
1: Payload internal buffer full
Bit 18 PBE: Payload buffer empty
This bit indicates the empty status of the generic payload internal buffer:
0: Payload internal buffer not empty
1: Payload internal buffer empty
Bit 17 CMDBF: Command buffer full
This bit indicates the full status of the generic command internal buffer:
0: Command internal buffer not full
1: Command internal buffer full

<!-- pagebreak -->

RM0456 Rev 6

CMDBF CMDBE

r

r

r

RM0456

DSI Host (DSI)

Bit 16 CMDBE: Command buffer empty
This bit indicates the empty status of the generic payload internal buffer:
0: Payload internal buffer not full
1: Payload internal buffer full
Bits 15:7 Reserved, must be kept at reset value.
Bit 6 RCB: Read command busy
This bit is set when a read command is issued and cleared when the entire response is
stored in the FIFO:
0: No read command ongoing
1: Read command ongoing
Bit 5 PRDFF: Payload read FIFO full
This bit indicates the full status of the generic read payload FIFO:
0: Read payload FIFO not full
1: Read payload FIFO full
Bit 4 PRDFE: Payload read FIFO empty
This bit indicates the empty status of the generic read payload FIFO:
0: Read payload FIFO not empty
1: Read payload FIFO empty
Bit 3 PWRFF: Payload write FIFO full
This bit indicates the full status of the generic write payload FIFO:
0: Write payload FIFO not full
1: Write payload FIFO full
Bit 2 PWRFE: Payload write FIFO empty
This bit indicates the empty status of the generic write payload FIFO:
0: Write payload FIFO not empty
1: Write payload FIFO empty
Bit 1 CMDFF: Command FIFO full
This bit indicates the full status of the generic command FIFO:
0: Write payload FIFO not full
1: Write payload FIFO full
Bit 0 CMDFE: Command FIFO empty
This bit indicates the empty status of the generic command FIFO:
0: Write payload FIFO not empty
1: Write payload FIFO empty

44.15.27 DSI Host timeout counter configuration register 0 (DSI_TCCR0)
Address offset: 0x0078
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

17

16

HSTX_TOCNT[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

rw

rw

LPRX_TOCNT[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Bits 31:16 HSTX_TOCNT[15:0]: High-speed transmission timeout counter
This field configures the timeout counter that triggers a high-speed transmission timeout
contention detection (measured in TOCKDIV cycles).
When using the non-burst mode and there is no enough time to switch from high-speed to
low-power and back in the period from one line data finishing to the next line sync start, the
DSI link returns the low-power state once per frame, then configure TOCKDIV and
HSTX_TOCNT to be in accordance with:
HSTX_TOCNT * lanebyteclkperiod * TOCKDIV ≥ the time of one FRAME data transmission
* (1 + 10%)
In burst mode, RGB pixel packets are time-compressed, leaving more time during a scan
line. Therefore, if in burst mode and there is enough time to switch from high-speed to lowpower and back in the period from one line data finishing to the next line sync start, the DSI
link can return low-power mode and back in this time interval to save power. For this,
configure the TOCKDIV and HSTX_TOCNT to be in accordance with:
HSTX_TOCNT * lanebyteclkperiod * TOCKDIV ≥ the time of one LINE data transmission
* (1 + 10%)
Bits 15:0 LPRX_TOCNT[15:0]: Low-power reception timeout counter
This field configures the timeout counter that triggers a low-power reception timeout
contention detection (measured in TOCKDIV cycles).

44.15.28 DSI Host timeout counter configuration register 1 (DSI_TCCR1)
Address offset: 0x007C
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

rw

rw

HSRD_TOCNT[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 HSRD_TOCNT[15:0]: High-speed read timeout counter
This field sets a period for which the DSI Host keeps the link still, after sending a high-speed
read operation. This period is measured in cycles of lanebyteclk. The counting starts when
the D-PHY enters the Stop state and causes no interrupts.

44.15.29 DSI Host timeout counter configuration register 2 (DSI_TCCR2)
Address offset: 0x0080
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

rw

rw

LPRD_TOCNT[15:0]
rw

rw

<!-- pagebreak -->

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

RM0456

DSI Host (DSI)

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 LPRD_TOCNT[15:0]: Low-power read timeout counter
This field sets a period for which the DSI Host keeps the link still, after sending a low-power
read operation. This period is measured in cycles of lanebyteclk. The counting starts when
the D-PHY enters the Stop state and causes no interrupts.

44.15.30 DSI Host timeout counter configuration register 3 (DSI_TCCR3)
Address offset: 0x0084
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

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PM

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

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

HSWR_TOCNT[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:25 Reserved, must be kept at reset value.
Bit 24 PM: Presp mode
When set to 1, this bit ensures that the peripheral response timeout caused by
HSWR_TOCNT is used only once per LTDC frame in command mode, when both the
following conditions are met:
– dpivsync_edpiwms has risen and fallen.
– Packets originated from LTDC in command mode have been transmitted and its FIFO is
empty again.
In this scenario no non-LTDC command requests are sent to the D-PHY, even if there is
traffic from generic interface ready to be sent, making it return to stop state. When it does so,
PRESP_TO counter is activated and only when it finishes does the controller send any other
traffic that is ready.
Bits 23:16 Reserved, must be kept at reset value.
Bits 15:0 HSWR_TOCNT[15:0]: High-speed write timeout counter
This field sets a period for which the DSI Host keeps the link inactive after sending a highspeed write operation. This period is measured in cycles of lanebyteclk. The counting starts
when the D-PHY enters the Stop state and causes no interrupts.

44.15.31 DSI Host timeout counter configuration register 4 (DSI_TCCR4)
Address offset: 0x0088
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

rw

rw

LPWR_TOCNT[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 LPWR_TOCNT[15:0]: Low-power write timeout counter
This field sets a period for which the DSI Host keeps the link still, after sending a low-power
write operation. This period is measured in cycles of lanebyteclk. The counting starts when
the D-PHY enters the Stop state and causes no interrupts.

44.15.32 DSI Host timeout counter configuration register 5 (DSI_TCCR5)
Address offset: 0x008C
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

rw

rw

BTA_TOCNT[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 BTA_TOCNT[15:0]: Bus-turn-around timeout counter
This field sets a period for which the DSI Host keeps the link still, after completing a bus-turnaround. This period is measured in cycles of lanebyteclk. The counting starts when the
D-PHY enters the Stop state and causes no interrupts.

44.15.33 DSI Host clock lane configuration register (DSI_CLCR)
Address offset: 0x0094
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

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

Res.

Res.

Res.

ACR

DPCC

rw

rw

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 ACR: Automatic clock lane control
This bit enables the automatic mechanism to stop providing clock in the clock lane when time
allows.
0: Automatic clock lane control disabled
1: Automatic clock lane control enabled
Bit 0 DPCC: D-PHY clock control
This bit controls the D-PHY clock state:
0: Clock lane is in low-power mode.
1: Clock lane runs in high-speed mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

44.15.34 DSI Host clock lane timer configuration register (DSI_CLTCR)
Address offset: 0x0098
Reset value: 0x0000 0000
31

30

29

28

27

26

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

Res.

Res.

Res.

Res.

Res.

Res.

25

24

23

22

21

20

19

18

17

16

HS2LP_TIME[9:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

9

8

7

6

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

rw

rw

rw

LP2HS_TIME[9:0]
rw

rw

Bits 31:26 Reserved, must be kept at reset value.
Bits 25:16 HS2LP_TIME[9:0]: High-speed to low-power time
This field configures the maximum time that the D-PHY clock lane takes to go from
high-speed to low-power transmission measured in lane byte clock cycles.
Bits 15:10 Reserved, must be kept at reset value.
Bits 9:0 LP2HS_TIME[9:0]: Low-power to high-speed time
This field configures the maximum time that the D-PHY clock lane takes to go from
low-power to high-speed transmission measured in lane byte clock cycles.

44.15.35 DSI Host data lane timer configuration register (DSI_DLTCR)
Address offset: 0x009C
Reset value: 0x0000 0000
31

30

29

28

27

26

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

Res.

Res.

Res.

Res.

Res.

Res.

25

24

23

22

21

rw

rw

rw

rw

rw

9

8

7

6

5

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

4

3

2

1

0

rw

rw

rw

rw

HS2LP_TIME[9:0]

LP2HS_TIME[9:0]
rw

rw

rw

rw

rw

rw

Bits 31:26 Reserved, must be kept at reset value.
Bits 25:16 HS2LP_TIME[9:0]: High-speed to low-power time
This field configures the maximum time that the D-PHY data lanes take to go from highspeed to low-power transmission measured in lane byte clock cycles.
Bits 15:10 Reserved, must be kept at reset value.
Bits 9:0 LP2HS_TIME[9:0]: Low-power to high-speed time
This field configures the maximum time that the D-PHY data lanes take to go from low-power
to high-speed transmission measured in lane byte clock cycles.

44.15.36 DSI Host PHY control register (DSI_PCTLR)
Address offset: 0x00A0
Reset value: 0x0000 0000

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

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

Res.

Res.

CKE

DEN

Res.

rw

rw

Bits 31:3 Reserved, must be kept at reset value.
Bit 2 CKE: Clock enable
This bit enables the D-PHY clock lane module:
0: D-PHY clock lane module is disabled
1: D-PHY clock lane module is enabled.
Bit 1 DEN: Digital enable
When set to 0, this bit puts the digital section of the D-PHY in the reset state
0: The digital section of the D-PHY is in the reset state
1: The digital section of the D-PHY is enabled
Bit 0 Reserved, must be kept at reset value.

44.15.37 DSI Host PHY configuration register (DSI_PCONFR)
Address offset: 0x00A4
Reset value: 0x0000 0001
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

rw

rw

rw

rw

rw

rw

SW_TIME[7:0]
rw

rw

NL[1:0]
rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:8 SW_TIME[7:0]: Stop wait time
This field configures the minimum wait period to request a high-speed transmission after the
Stop state.
Bits 7:2 Reserved, must be kept at reset value.
Bits 1:0 NL[1:0]: Number of lanes
This field configures the number of active data lanes:
00: One data lane (lane 0)
01: Two data lanes (lanes 0 and 1) - Reset value
Others: Reserved

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

44.15.38 DSI Host PHY ULPS control register (DSI_PUCR)
Address offset: 0x00A8
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

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

Res.

UEDL

URDL

UECL

URCL

rw

rw

rw

rw

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 UEDL: ULPS exit on data lane
ULPS mode exit on all active data lanes.
0: No exit request
1: Exit ULPS mode on all active data lane URDL
Bit 2 URDL: ULPS request on data lane
ULPS mode request on all active data lanes.
0: No ULPS request
1: Request ULPS mode on all active data lane UECL
Bit 1 UECL: ULPS exit on clock lane
ULPS mode exit on clock lane.
0: No exit request
1: Exit ULPS mode on clock lane
Bit 0 URCL: ULPS request on clock lane
ULPS mode request on clock lane.
0: No ULPS request
1: Request ULPS mode on clock lane

44.15.39 DSI Host PHY TX triggers configuration register (DSI_PTTCR)
Address offset: 0x00AC
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

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

Res.

TX_TRIG[3:0]
rw

rw

rw

rw

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 TX_TRIG[3:0]: Transmission trigger
Escape mode transmit trigger 0-3.
Only one bit of TX_TRIG is asserted at any given time.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

44.15.40 DSI Host PHY status register (DSI_PSR)
Address offset: 0x00B0
Reset value: 0x0000 1528
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

UAN1

PSS1

RUE0

UAN0

PSS0

UANC

PSSC

PD

Res.

r

r

r

r

r

r

r

r

Bits 31:9 Reserved, must be kept at reset value.
Bit 8 UAN1: ULPS active not lane 1
This bit indicates the status of ulpsactivenot1lane D-PHY signal.
Bit 7 PSS1: PHY stop state lane 1
This bit indicates the status of phystopstate1lane D-PHY signal.
Bit 6 RUE0: RX ULPS escape lane 0
This bit indicates the status of rxulpsesc0lane D-PHY signal.
Bit 5 UAN0: ULPS active not lane 1
This bit indicates the status of ulpsactivenot0lane D-PHY signal.
Bit 4 PSS0: PHY stop state lane 0
This bit indicates the status of phystopstate0lane D-PHY signal.
Bit 3 UANC: ULPS active not clock lane
This bit indicates the status of ulpsactivenotclklane D-PHY signal.
Bit 2 PSSC: PHY stop state clock lane
This bit indicates the status of phystopstateclklane D-PHY signal.
Bit 1 PD: PHY direction
This bit indicates the status of phydirection D-PHY signal.
Bit 0 Reserved, must be kept at reset value.

44.15.41 DSI Host interrupt and status register 0 (DSI_ISR0)
Address offset: 0x00BC
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

17

16

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

PE4

PE3

PE2

PE1

PE0

r

r

r

r

r

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

5

4

3

2

1

0

AE15

AE14

AE13

AE12

AE11

AE10

AE9

AE8

AE7

AE6

AE5

AE4

AE3

AE2

AE1

AE0

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

Bits 31:21 Reserved, must be kept at reset value.
Bit 20 PE4: PHY error 4
This bit indicates the LP1 contention error ErrContentionLP1 from lane 0.
Bit 19 PE3: PHY error 3
This bit indicates the LP0 contention error ErrContentionLP0 from lane 0.
Bit 18 PE2: PHY error 2
This bit indicates the ErrControl error from lane 0.
Bit 17 PE1: PHY error 1
This bit indicates the ErrSyncEsc low-power transmission synchronization error from lane 0.
Bit 16 PE0: PHY error 0
This bit indicates the ErrEsc escape entry error from lane 0.
Bit 15 AE15: Acknowledge error 15
This bit retrieves the DSI protocol violation from the acknowledge error report.
Bit 14 AE14: Acknowledge error 14
This bit retrieves the reserved (specific to the device) from the acknowledge error report.
Bit 13 AE13: Acknowledge error 13
This bit retrieves the invalid transmission length from the acknowledge error report.
Bit 12 AE12: Acknowledge error 12
This bit retrieves the DSI VC ID Invalid from the acknowledge error report.
Bit 11 AE11: Acknowledge error 11
This bit retrieves the not recognized DSI data type from the acknowledge error report.
Bit 10 AE10: Acknowledge error 10
This bit retrieves the checksum error (long packet only) from the acknowledge error report.
Bit 9 AE9: Acknowledge error 9
This bit retrieves the ECC error, multi-bit (detected, not corrected) from the acknowledge
error report.
Bit 8 AE8: Acknowledge error 8
This bit retrieves the ECC error, single-bit (detected and corrected) from the acknowledge
error report.
Bit 7 AE7: Acknowledge error 7
This bit retrieves the reserved (specific to the device) from the acknowledge error report.
Bit 6 AE6: Acknowledge error 6
This bit retrieves the false control error from the acknowledge error report.
Bit 5 AE5: Acknowledge error 5
This bit retrieves the peripheral timeout error from the acknowledge error report.
Bit 4 AE4: Acknowledge error 4
This bit retrieves the LP transmit sync error from the acknowledge error report.
Bit 3 AE3: Acknowledge error 3
This bit retrieves the escape mode entry command error from the acknowledge error report.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Bit 2 AE2: Acknowledge error 2
This bit retrieves the EoT sync error from the acknowledge error report.
Bit 1 AE1: Acknowledge error 1
This bit retrieves the SoT sync error from the acknowledge error report.
Bit 0 AE0: Acknowledge error 0
This bit retrieves the SoT error from the acknowledge error report.

44.15.42 DSI Host interrupt and status register 1 (DSI_ISR1)
Address offset: 0x00C0
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

17

16

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

Res.

PBUE

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

Res.

r

GPRXE GPRDE GPTXE GPWRE GCWRE LPWRE EOTPE
r

r

r

r

r

r

r

PSE
r

CRCE ECCME ECCSE TOLPRX TOHSTX
r

r

r

r

r

Bits 31:20 Reserved, must be kept at reset value.
Bit 19 PBUE: Payload buffer underflow error
This bit indicates that underflow has occurred when reading payload to build DSI packet for
video mode.
Bits 18:13 Reserved, must be kept at reset value.
Bit 12 GPRXE: Generic payload receive error
This bit indicates that during a generic interface packet read back, the payload FIFO
becomes full and the received data is corrupted.
Bit 11 GPRDE: Generic payload read error
This bit indicates that during a DCS read data, the payload FIFO becomes empty and the
data sent to the interface is corrupted.
Bit 10 GPTXE: Generic payload transmit error
This bit indicates that during a generic interface packet build, the payload FIFO becomes
empty and corrupt data is sent.
Bit 9 GPWRE: Generic payload write error
This bit indicates that the system tried to write a payload data through the generic interface
and the FIFO is full. Therefore, the payload is not written.
Bit 8 GCWRE: Generic command write error
This bit indicates that the system tried to write a command through the generic interface and
the FIFO is full. Therefore, the command is not written.
Bit 7 LPWRE: LTDC payload write error
This bit indicates that during a DPI pixel line storage, the payload FIFO becomes full and the
data stored is corrupted.
Bit 6 EOTPE: EoTp error
This bit indicates that the EoTp packet is not received at the end of the incoming peripheral
transmission.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

Bit 5 PSE: Packet size error
This bit indicates that the packet size error is detected during the packet reception.
Bit 4 CRCE: CRC error
This bit indicates that the CRC error is detected in the received packet payload.
Bit 3 ECCME: ECC multi-bit error
This bit indicates that the ECC multiple error is detected in a received packet.
Bit 2 ECCSE: ECC single-bit error
This bit indicates that the ECC single error is detected and corrected in a received packet.
Bit 1 TOLPRX: Timeout low-power reception
This bit indicates that the low-power reception timeout counter reached the end and
contention is detected.
Bit 0 TOHSTX: Timeout high-speed transmission
This bit indicates that the high-speed transmission timeout counter reached the end and
contention is detected.

44.15.43 DSI Host interrupt enable register 0 (DSI_IER0)
Address offset: 0x00C4
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

17

16

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

PE4IE

PE3IE

PE2IE

PE1IE

PE0IE

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

5

4

3

2

1

0

AE9IE

AE8IE

AE7IE

AE6IE

AE5IE

AE4IE

AE3IE

AE2IE

AE1IE

AE0IE

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

AE15IE AE14IE AE13IE AE12IE AE11IE AE10IE
rw

rw

rw

rw

rw

rw

Bits 31:21 Reserved, must be kept at reset value.
Bit 20 PE4IE: PHY error 4 interrupt enable
This bit enables the interrupt generation on PHY error 4.
0: Interrupt on PHY error 4 disabled
1: Interrupt on PHY error 4 enabled
Bit 19 PE3IE: PHY error 3 interrupt enable
This bit enables the interrupt generation on PHY error 4.
0: Interrupt on PHY error 3 disabled
1: Interrupt on PHY error 3 enabled
Bit 18 PE2IE: PHY error 2 interrupt enable
This bit enables the interrupt generation on PHY error 2.
0: Interrupt on PHY error 2 disabled
1: Interrupt on PHY error 2 enabled
Bit 17 PE1IE: PHY error 1 interrupt enable
This bit enables the interrupt generation on PHY error 1.
0: Interrupt on PHY error 1 disabled
1: Interrupt on PHY error 1 enabled

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Bit 16 PE0IE: PHY error 0 interrupt enable
This bit enables the interrupt generation on PHY error 0.
0: Interrupt on PHY error 0 disabled
1: Interrupt on PHY error 0 enabled
Bit 15 AE15IE: Acknowledge error 15 interrupt enable
This bit enables the interrupt generation on acknowledge error 15.
0: Interrupt on acknowledge error 15 disabled
1: Interrupt on acknowledge error 15 enabled
Bit 14 AE14IE: Acknowledge error 14 interrupt enable
This bit enables the interrupt generation on acknowledge error 14.
0: Interrupt on acknowledge error 14 disabled
1: Interrupt on acknowledge error 14 enabled
Bit 13 AE13IE: Acknowledge error 13 interrupt enable
This bit enables the interrupt generation on acknowledge error 13.
0: Interrupt on acknowledge error 13 disabled
1: Interrupt on acknowledge error 13 enabled
Bit 12 AE12IE: Acknowledge error 12 interrupt enable
This bit enables the interrupt generation on acknowledge error 12.
0: Interrupt on acknowledge error 12 disabled
1: Interrupt on acknowledge error 12 enabled
Bit 11 AE11IE: Acknowledge error 11 interrupt enable
This bit enables the interrupt generation on acknowledge error 11.
0: Interrupt on acknowledge error 11 disabled
1: Interrupt on acknowledge error 11 enabled
Bit 10 AE10IE: Acknowledge error 10 interrupt enable
This bit enables the interrupt generation on acknowledge error 10.
0: Interrupt on acknowledge error 10 disabled
1: Interrupt on acknowledge error 10 enable.
Bit 9 AE9IE: Acknowledge error 9 interrupt enable
This bit enables the interrupt generation on acknowledge error 9.
0: Interrupt on acknowledge error 9 disabled
1: Interrupt on acknowledge error 9 enabled
Bit 8 AE8IE: Acknowledge error 8 interrupt enable
This bit enables the interrupt generation on acknowledge error 8.
0: Interrupt on acknowledge error 8 disabled
1: Interrupt on acknowledge error 8 enabled
Bit 7 AE7IE: Acknowledge error 7 interrupt enable
This bit enables the interrupt generation on acknowledge error 7.
0: Interrupt on acknowledge error 7 disabled
1: Interrupt on acknowledge error 7 enabled
Bit 6 AE6IE: Acknowledge error 6 interrupt enable
This bit enables the interrupt generation on acknowledge error 6.
0: Interrupt on acknowledge error 6 disabled
1: Interrupt on acknowledge error 6 enabled

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

Bit 5 AE5IE: Acknowledge error 5 interrupt enable
This bit enables the interrupt generation on acknowledge error 5.
0: Interrupt on acknowledge error 5 disabled
1: Interrupt on acknowledge error 5 enabled
Bit 4 AE4IE: Acknowledge error 4 interrupt enable
This bit enables the interrupt generation on acknowledge error 4.
0: Interrupt on acknowledge error 4 disabled
1: Interrupt on acknowledge error 4 enabled
Bit 3 AE3IE: Acknowledge error 3 interrupt enable
This bit enables the interrupt generation on acknowledge error 3.
0: Interrupt on acknowledge error 3 disabled
1: Interrupt on acknowledge error 3 enabled
Bit 2 AE2IE: Acknowledge error 2 interrupt enable
This bit enables the interrupt generation on acknowledge error 2.
0: Interrupt on acknowledge error 2 disabled
1: Interrupt on acknowledge error 2 enabled
Bit 1 AE1IE: Acknowledge error 1 interrupt enable
This bit enables the interrupt generation on acknowledge error 1.
0: Interrupt on acknowledge error 1 disabled
1: Interrupt on acknowledge error 1 enabled
Bit 0 AE0IE: Acknowledge error 0 interrupt enable
This bit enables the interrupt generation on acknowledge error 0.
0: Interrupt on acknowledge error 0 disabled
1: Interrupt on acknowledge error 0 enabled

44.15.44 DSI Host interrupt enable register 1 (DSI_IER1)
Address offset: 0x00C8
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

17

16

PBU
EIE

Res.

Res.

Res.

2

1

0

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

Res.

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

5

4

3

Res.

Res.

Res.

GPRX
EIE

GPRD
EIE

GPTX
EIE

GPWR
EIE

GCWR
EIE

LPWR
EIE

EOTP
EIE

PS
EIE

CRC
EIE

ECCM
EIE

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

ECCS TOLPRX TOHSTX
EIE
IE
IE
rw

rw

rw

Bits 31:20 Reserved, must be kept at reset value.
Bit 19 PBUEIE: Payload buffer underflow error interrupt enable
This bit enables the interrupt generation on payload buffer underflow error.
0: Interrupt on payload buffer underflow error disabled
1: Interrupt on payload buffer underflow error enabled
Bits 18:13 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Bit 12 GPRXEIE: Generic payload receive error interrupt enable
This bit enables the interrupt generation on generic payload receive error.
0: Interrupt on generic payload receive error disabled
1: Interrupt on generic payload receive error enabled
Bit 11 GPRDEIE: Generic payload read error interrupt enable
This bit enables the interrupt generation on generic payload read error.
0: Interrupt on generic payload read error disabled
1: Interrupt on generic payload read error enabled
Bit 10 GPTXEIE: Generic payload transmit error interrupt enable
This bit enables the interrupt generation on generic payload transmit error.
0: Interrupt on generic payload transmit error disabled
1: Interrupt on generic payload transmit error enabled
Bit 9 GPWREIE: Generic payload write error interrupt enable
This bit enables the interrupt generation on generic payload write error.
0: Interrupt on generic payload write error disabled
1: Interrupt on generic payload write error enabled
Bit 8 GCWREIE: Generic command write error interrupt enable
This bit enables the interrupt generation on generic command write error.
0: Interrupt on generic command write error disabled
1: Interrupt on generic command write error enabled
Bit 7 LPWREIE: LTDC payload write error interrupt enable
This bit enables the interrupt generation on LTDC payload write error.
0: Interrupt on LTDC payload write error disabled
1: Interrupt on LTDC payload write error enabled
Bit 6 EOTPEIE: EoTp error interrupt enable
This bit enables the interrupt generation on EoTp error.
0: Interrupt on EoTp error disabled
1: Interrupt on EoTp error enabled
Bit 5 PSEIE: Packet size error interrupt enable
This bit enables the interrupt generation on packet size error.
0: Interrupt on packet size error disabled
1: Interrupt on packet size error enabled
Bit 4 CRCEIE: CRC error interrupt enable
This bit enables the interrupt generation on CRC error.
0: Interrupt on CRC error disabled
1: Interrupt on CRC error enabled
Bit 3 ECCMEIE: ECC multi-bit error interrupt enable
This bit enables the interrupt generation on ECC multi-bit error.
0: Interrupt on ECC multi-bit error disabled
1: Interrupt on ECC multi-bit error enabled
Bit 2 ECCSEIE: ECC single-bit error interrupt enable
This bit enables the interrupt generation on ECC single-bit error.
0: Interrupt on ECC single-bit error disabled
1: Interrupt on ECC single-bit error enabled

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

Bit 1 TOLPRXIE: Timeout low-power reception interrupt enable
This bit enables the interrupt generation on timeout low-power reception.
0: Interrupt on timeout low-power reception disabled
1: Interrupt on timeout low-power reception enabled
Bit 0 TOHSTXIE: Timeout high-speed transmission interrupt enable
This bit enables the interrupt generation on timeout high-speed transmission .
0: Interrupt on timeout high-speed transmission disabled
1: Interrupt on timeout high-speed transmission enabled

44.15.45 DSI Host force interrupt register 0 (DSI_FIR0)
Address offset: 0x00D8
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

17

16

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

FPE4

FPE3

FPE2

FPE1

FPE0

w

w

w

w

w

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

5

4

3

2

1

0

FAE15

FAE14

FAE13

FAE12

FAE11

FAE10

FAE9

FAE8

FAE7

FAE6

FAE5

FAE4

FAE3

FAE2

FAE1

FAE0

w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

Bits 31:21 Reserved, must be kept at reset value.
Bit 20 FPE4: Force PHY error 4
Writing one to this bit forces a PHY error 4.
Bit 19 FPE3: Force PHY error 3
Writing one to this bit forces a PHY error 3.
Bit 18 FPE2: Force PHY error 2
Writing one to this bit forces a PHY error 2.
Bit 17 FPE1: Force PHY error 1
Writing one to this bit forces a PHY error 1.
Bit 16 FPE0: Force PHY error 0
Writing one to this bit forces a PHY error 0.
Bit 15 FAE15: Force acknowledge error 15
Writing one to this bit forces an acknowledge error 15.
Bit 14 FAE14: Force acknowledge error 14
Writing one to this bit forces an acknowledge error 14.
Bit 13 FAE13: Force acknowledge error 13
Writing one to this bit forces an acknowledge error 13.
Bit 12 FAE12: Force acknowledge error 12
Writing one to this bit forces an acknowledge error 12.
Bit 11 FAE11: Force acknowledge error 11
Writing one to this bit forces an acknowledge error 11.
Bit 10 FAE10: Force acknowledge error 10
Writing one to this bit forces an acknowledge error 10.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Bit 9 FAE9: Force acknowledge error 9
Writing one to this bit forces an acknowledge error 9.
Bit 8 FAE8: Force acknowledge error 8
Writing one to this bit forces an acknowledge error 8.
Bit 7 FAE7: Force acknowledge error 7
Writing one to this bit forces an acknowledge error 7.
Bit 6 FAE6: Force acknowledge error 6
Writing one to this bit forces an acknowledge error 6.
Bit 5 FAE5: Force acknowledge error 5
Writing one to this bit forces an acknowledge error 5.
Bit 4 FAE4: Force acknowledge error 4
Writing one to this bit forces an acknowledge error 4.
Bit 3 FAE3: Force acknowledge error 3
Writing one to this bit forces an acknowledge error 3.
Bit 2 FAE2: Force acknowledge error 2
Writing one to this bit forces an acknowledge error 2.
Bit 1 FAE1: Force acknowledge error 1
Writing one to this bit forces an acknowledge error 1.
Bit 0 FAE0: Force acknowledge error 0
Writing one to this bit forces an acknowledge error 0.

44.15.46 DSI Host force interrupt register 1 (DSI_FIR1)
Address offset: 0x00DC
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

17

16

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

Res.

FPBUE

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

Res.

FGP
RXE

FGP
RDE

FGP
TXE

FGP
WRE

FGC
WRE

FLP
WRE

FE
OTPE

FPSE

FCRCE

FECC
ME

FECC
SE

w

w

w

w

w

w

w

w

w

w

w

w

Bits 31:20 Reserved, must be kept at reset value.
Bit 19 FPBUE: Force payload buffer underflow error
Writing one to this bit forces a payload underflow error.
Bits 18:13 Reserved, must be kept at reset value.
Bit 12 FGPRXE: Force generic payload receive error
Writing one to this bit forces a generic payload receive error.
Bit 11 FGPRDE: Force generic payload read error
Writing one to this bit forces a generic payload read error.

<!-- pagebreak -->

RM0456 Rev 6

FTOLP FTOHS
RX
TX
w

w

RM0456

DSI Host (DSI)

Bit 10 FGPTXE: Force generic payload transmit error
Writing one to this bit forces a generic payload transmit error.
Bit 9 FGPWRE: Force generic payload write error
Writing one to this bit forces a generic payload write error.
Bit 8 FGCWRE: Force generic command write error
Writing one to this bit forces a generic command write error.
Bit 7 FLPWRE: Force LTDC payload write error
Writing one to this bit forces a LTDC payload write error.
Bit 6 FEOTPE: Force EoTp error
Writing one to this bit forces a EoTp error.
Bit 5 FPSE: Force packet size error
Writing one to this bit forces a packet size error.
Bit 4 FCRCE: Force CRC error
Writing one to this bit forces a CRC error.
Bit 3 FECCME: Force ECC multi-bit error
Writing one to this bit forces a ECC multi-bit error.
Bit 2 FECCSE: Force ECC single-bit error
Writing one to this bit forces a ECC single-bit error.
Bit 1 FTOLPRX: Force timeout low-power reception
Writing one to this bit forces a timeout low-power reception.
Bit 0 FTOHSTX: Force timeout high-speed transmission
Writing one to this bit forces a timeout high-speed transmission.

44.15.47 DSI Host data lane timer read configuration register
(DSI_DLTRCR)
Address offset: 0x00F4
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

rw

Res.

MRD_TIME[14:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:15 Reserved, must be kept at reset value.
Bits 14:0 MRD_TIME[14:0]: Maximum read time
This field configures the maximum time required to perform a read command in lane byte
clock cycles. This register can only be modified when no read command is in progress.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

44.15.48 DSI Host video shadow control register (DSI_VSCR)
Address offset: 0x0100
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

UR

Res.

Res.

Res.

Res.

Res.

Res.

Res.

EN

rw

rw

Bits 31:9 Reserved, must be kept at reset value.
Bit 8 UR: Update register
When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit
is auto cleared.
0: No update requested
1: Register update requested
Bits 7:1 Reserved, must be kept at reset value.
Bit 0 EN: Enable
When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary
registers.
When this bit is set along with the UR bit, the auxiliary registers are automatically updated.
0: Register update is disabled.
1: Register update is enabled.

44.15.49 DSI Host LTDC current VCID register (DSI_LCVCIDR)
Address offset: 0x010C
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

17

16

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

Res.

Res.

Res.

Res.

Res.

1

0

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

5

4

3

2

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

Res.

Res.

Res.

VCID[1:0]
rw

Bits 31:2 Reserved, must be kept at reset value.
Bits 1:0 VCID[1:0]: Virtual channel ID
This field returns the virtual channel ID for the LTDC interface.

44.15.50 DSI Host LTDC current color coding register (DSI_LCCCR)
Address offset: 0x0110
Reset value: 0x0000 0000

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

DSI Host (DSI)

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

17

16

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

Res.

Res.

Res.

Res.

Res.

3

2

1

0

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

5

4

Res.

Res.

Res.

Res.

Res.

Res.

Res.

LPE

Res.

Res.

Res.

Res.

r

COLC[3:0]
r

r

r

r

Bits 31:9 Reserved, must be kept at reset value.
Bit 8 LPE: Loosely packed enable
This bit returns the current state of the loosely packed variant to 18-bit configurations.
0: Loosely packed variant disabled
1: Loosely packed variant enabled
Bits 7:4 Reserved, must be kept at reset value.
Bits 3:0 COLC[3:0]: Color coding
This field returns the current LTDC interface color coding.
0000: 16-bit configuration 1
0001: 16-bit configuration 2
0010: 16-bit configuration 3
0011: 18-bit configuration 1
0100: 18-bit configuration 2
0101: 24-bit
0110 - 1111: reserved
If LTDC interface in command mode is chosen and currently works in the command mode
(CMDM=1), then 0110-1111: 24-bit

44.15.51 DSI Host low-power mode current configuration register
(DSI_LPMCCR)
Address offset: 0x0118
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

23

22

21

20

19

18

17

16

LPSIZE[7:0]
r

r

r

7

6

5

r

r

r

r

r

4

3

2

1

0

r

r

r

VLPSIZE[7:0]
r

r

r

r

r

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:16 LPSIZE[7:0]: Largest packet size
This field is returns the current size, in bytes, of the largest packet that can fit in a line during
VSA, VBP and VFP regions, for the transmission of commands in low-power mode.
Bits 15:8 Reserved, must be kept at reset value.
Bits 7:0 VLPSIZE[7:0]: VACT largest packet size
This field returns the current size, in bytes, of the largest packet that can fit in a line during
VACT regions, for the transmission of commands in low-power mode.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

44.15.52 DSI Host video mode current configuration register
(DSI_VMCCR)
Address offset: 0x0138
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

17

16

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

Res.

Res.

Res.

Res.

Res.

4

3

2

1

0

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

5

Res.

Res.

Res.

Res.

Res.

Res.

LPCE

FBTAAE

LPHFE

LPHBPE

LPVAE

r

r

r

r

r

LPVFPE LPVBPE LPVSAE
r

r

r

VMT[1:0]
r

r

Bits 31:10 Reserved, must be kept at reset value.
Bit 9 LPCE: Low-power command enable
This bit returns the current command transmission state in low-power mode.
0: Command transmission in low-power mode is disabled.
1: Command transmission in low-power mode is enabled.
Bit 8 FBTAAE: Frame BTA acknowledge enable
This bit returns the current state of request for an acknowledge response at the end of a
frame.
0: Acknowledge response at the end of a frame is disabled.
1: Acknowledge response at the end of a frame is enabled.
Bit 7 LPHFE: Low-power horizontal front-porch enable
This bit returns the current state of return to low-power inside the horizontal front-porch
(HFP) period when timing allows.
0: Return to low-power inside the HFP period is disabled.
1: Return to low-power inside the HFP period is enabled.
Bit 6 LPHBPE: Low-power horizontal back-porch enable
This bit returns the current state of return to low-power inside the horizontal back-porch
(HBP) period when timing allows.
0: Return to low-power inside the HBP period is disabled.
1: Return to low-power inside the HBP period is enabled.
Bit 5 LPVAE: Low-power vertical active enable
This bit returns the current state of return to low-power inside the vertical active (VACT)
period when timing allows.
0: Return to low-power inside the VACT is disabled.
1: Return to low-power inside the VACT is enabled.
Bit 4 LPVFPE: Low-power vertical front-porch enable
This bit returns the current state of return to low-power inside the vertical front-porch (VFP)
period when timing allows.
0: Return to low-power inside the VFP is disabled.
1: Return to low-power inside the VFP is enabled.
Bit 3 LPVBPE: Low-power vertical back-porch enable
This bit returns the current state of return to low-power inside the vertical back-porch (VBP)
period when timing allows.
0: Return to low-power inside the VBP is disabled.
1: Return to low-power inside the VBP is enabled.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

Bit 2 LPVSAE: Low-power vertical sync time enable
This bit returns the current state of return to low-power inside the vertical sync time (VSA)
period when timing allows.
0: Return to low-power inside the VSA is disabled.
1: Return to low-power inside the VSA is enabled
Bits 1:0 VMT[1:0]: Video mode type
This field returns the current video mode transmission type:
00: Non-burst with sync pulses
01: Non-burst with sync events
1x: Burst mode

44.15.53 DSI Host video packet current configuration register
(DSI_VPCCR)
Address offset: 0x013C
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

r

r

r

r

r

r

VPSIZE[13:0]
r

r

r

r

r

r

r

r

Bits 31:14 Reserved, must be kept at reset value.
Bits 13:0 VPSIZE[13:0]: Video packet size
This field returns the number of pixels in a single video packet.

44.15.54 DSI Host video chunks current configuration register
(DSI_VCCCR)
Address offset: 0x0140
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

17

16

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

Res.

Res.

Res.

Res.

Res.

12

11

10

9

8

7

6

5

4

3

2

1

0

r

r

r

r

r

r

15

14

13

Res.

Res.

Res.

NUMC[12:0]
r

r

r

r

r

r

r

Bits 31:13 Reserved, must be kept at reset value.
Bits 12:0 NUMC[12:0]: Number of chunks
This field returns the number of chunks transmitted during a line period.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

44.15.55 DSI Host video null packet current configuration register
(DSI_VNPCCR)
Address offset: 0x0144
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

17

16

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

Res.

Res.

Res.

Res.

Res.

12

11

10

9

8

7

6

5

4

3

2

1

0

r

r

r

r

r

r

15

14

13

Res.

Res.

Res.

NPSIZE[12:0]
r

r

r

r

r

r

r

Bits 31:13 Reserved, must be kept at reset value.
Bits 12:0 NPSIZE[12:0]: Null packet size
This field returns the number of bytes inside a null packet.

44.15.56 DSI Host video HSA current configuration register
(DSI_VHSACCR)
Address offset: 0x0148
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

17

16

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

Res.

Res.

Res.

Res.

Res.

11

10

9

8

7

6

5

4

3

2

1

0

r

r

r

r

r

15

14

13

12

Res.

Res.

Res.

Res.

HSA[11:0]
r

r

r

r

r

r

r

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 HSA[11:0]: Horizontal synchronism active duration
This fields returns the horizontal synchronism active period in lane byte clock cycles.

44.15.57 DSI Host video HBP current configuration register
(DSI_VHBPCCR)
Address offset: 0x014C
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

17

16

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

Res.

Res.

Res.

Res.

Res.

11

10

9

8

7

6

5

4

3

2

1

0

r

r

r

r

r

15

14

13

12

Res.

Res.

Res.

Res.

HBP[11:0]
r

<!-- pagebreak -->

r

r

r

r

RM0456 Rev 6

r

r

RM0456

DSI Host (DSI)

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 HBP[11:0]: Horizontal back-porch duration
This field returns the horizontal back-porch period in lane byte clock cycles.

44.15.58 DSI Host video line current configuration register (DSI_VLCCR)
Address offset: 0x0150
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

r

r

r

r

r

r

r

Res.

HLINE[14:0]
r

r

r

r

r

r

r

r

Bits 31:15 Reserved, must be kept at reset value.
Bits 14:0 HLINE[14:0]: Horizontal line duration
This field returns the current total of the horizontal line period (HSA + HBP + HACT + HFP)
counted in lane byte clock cycles.

44.15.59 DSI Host video VSA current configuration register
(DSI_VVSACCR)
Address offset: 0x0154
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

17

16

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

Res.

Res.

Res.

Res.

Res.

9

8

7

6

5

4

3

2

1

0

r

r

r

r

15

14

13

12

11

10

Res.

Res.

Res.

Res.

Res.

Res.

VSA[9:0]
r

r

r

r

r

r

Bits 31:10 Reserved, must be kept at reset value.
Bits 9:0 VSA[9:0]: Vertical synchronism active duration
This field returns the current vertical synchronism active period measured in number of
horizontal lines.

44.15.60 DSI Host video VBP current configuration register
(DSI_VVBPCCR)
Address offset: 0x0158
Reset value: 0x0000 0000

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

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

17

16

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

Res.

Res.

Res.

Res.

Res.

9

8

7

6

5

4

3

2

1

0

r

r

r

r

15

14

13

12

11

10

Res.

Res.

Res.

Res.

Res.

Res.

VBP[9:0]
r

r

r

r

r

r

Bits 31:10 Reserved, must be kept at reset value.
Bits 9:0 VBP[9:0]: Vertical back-porch duration
This field returns the current vertical back-porch period measured in number of horizontal
lines.

44.15.61 DSI Host video VFP current configuration register
(DSI_VVFPCCR)
Address offset: 0x015C
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

17

16

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

Res.

Res.

Res.

Res.

Res.

9

8

7

6

5

4

3

2

1

0

r

r

r

r

15

14

13

12

11

10

Res.

Res.

Res.

Res.

Res.

Res.

VFP[9:0]
r

r

r

r

r

r

Bits 31:10 Reserved, must be kept at reset value.
Bits 9:0 VFP[9:0]: Vertical front-porch duration
This field returns the current vertical front-porch period measured in number of horizontal
lines.

44.15.62 DSI Host video VA current configuration register
(DSI_VVACCR)
Address offset: 0x0160
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

r

r

r

r

r

r

VA[13:0]
r

r

r

r

r

r

r

r

Bits 31:14 Reserved, must be kept at reset value.
Bits 13:0 VA[13:0]: Vertical active duration
This field returns the current vertical active period measured in number of horizontal lines.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

44.15.63 DSI Host FIFO and buffer status register (DSI_FBSR)
Address offset: 0x0168
Reset value: 0x0005 0015
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

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

APBF

APBE

ACBF

ACBE

Res.

Res.

VPBF

VPBE

r

r

r

r

r

r

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

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

APWFF APWFE ACWFF ACWFE VPWFF VPWFE VCWFF VCWFE
r

r

r

r

r

r

r

r

Bits 31:24 Reserved, must be kept at reset value.
Bit 23 APBF: Adapted command mode payload buffer full
This bit indicates the full status of the adapted command mode payload internal buffer:
0: Payload internal buffer not full
1: Payload internal buffer full
Bit 22 APBE: Adapted command mode payload buffer empty
This bit indicates the empty status of the adapted command mode payload internal
buffer:
0: Payload internal buffer not empty
1: Payload internal buffer empty
Bit 21 ACBF: Adapted command mode command buffer full
This bit indicates the full status of the adapted command mode command internal
buffer:
0: Command internal buffer not full
1: Command internal buffer full
Bit 20 ACBE: Adapted command mode command buffer empty
This bit indicates the empty status of the adapted command mode command internal
buffer:
0: Command internal buffer not empty
1: Command internal buffer empty
Bits 19:18 Reserved, must be kept at reset value.
Bit 17 VPBF: Video mode payload buffer full
This bit indicates the full status of the video mode payload internal buffer:
0: Payload internal buffer not full
1: Payload internal buffer full
Bit 16 VPBE: Video mode payload buffer empty
This bit indicates the empty status of the video mode payload internal buffer:
0: Payload internal buffer not empty
1: Payload internal buffer empty
Bits 15:8 Reserved, must be kept at reset value.
Bit 7 APWFF: Adapted command mode payload write FIFO full
This bit indicates the full status of the adapted command mode write payload FIFO:
0: Write payload FIFO not full
1: Write payload FIFO full

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Bit 6 APWFE: Adapted command mode payload write FIFO empty
This bit indicates the empty status of the adapted command mode write payload FIFO:
0: Write payload FIFO not empty
1: Write payload FIFO empty
Bit 5 ACWFF: Adapted command mode command write FIFO full
This bit indicates the full status of the adapted command mode write command FIFO:
0: Write command FIFO not full
1: Write command FIFO full
Bit 4 ACWFE: Adapted command mode command write FIFO empty
This bit indicates the empty status of the adapted command mode write command FIFO:
0: Write command FIFO not empty
1: Write command FIFO empty
Bit 3 VPWFF: Video mode payload write FIFO full
This bit indicates the full status of the video mode write payload FIFO:
0: Write payload FIFO not full
1: Write payload FIFO full
Bit 2 VPWFE: Video mode payload write FIFO empty
This bit indicates the empty status of the video mode write payload FIFO:
0: Write payload FIFO not empty
1: Write payload FIFO empty
Bit 1 VCWFF: Video mode command write FIFO full
This bit indicates the full status of the video mode write command FIFO:
0: Write command FIFO not full
1: Write command FIFO full
Bit 0 VCWFE: Video mode command write FIFO empty
This bit indicates the empty status of the video mode write command FIFO:
0: Write command FIFO not empty
1: Write command FIFO empty

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

44.16

DSI Wrapper registers

44.16.1

DSI Wrapper configuration register (DSI_WCFGR)
Address offset: 0x0400
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

17

16

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

Res.

Res.

Res.

Res.

Res.

5

4

3

2

1

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

Res.

Res.

Res.

VSPOL

AR

rw

rw

TEPOL TESRC
rw

rw

COLMUX[2:0]
rw

rw

0
DSIM

rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bit 7 VSPOL: VSync polarity
This bit selects the VSync edge on which the LTDC is halted. It can be changed only when
DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.EN = 0). It is recommanded to keep the
default polarity configuration to guarantee a correct behavior.
0: LTDC halted on a falling edge
1: LTDC halted on a rising edge
Bit 6 AR: Automatic refresh
This bit selects the refresh mode in DBI mode. It can be changed only when DSI Host is
stopped (DSI_CR.EN = 0).
0: automatic refresh mode disabled
1: automatic refresh mode enabled
Bit 5 TEPOL: TE polarity
This bit selects the polarity of the external pin tearing effect (TE) source. It can be changed
only when DSI Host is stopped (DSI_CR.EN = 0).
0: rising edge
1: falling edge
Bit 4 TESRC: TE source
This bit selects the tearing effect (TE) source. It can be changed when DSI Host is stopped
(DSI_CR.EN = 0).
0: DSI link
1: External pin
Bits 3:1 COLMUX[2:0]: Color multiplexing
This bitfield selects the color multiplexing used by DSI Host. It can be changed only when
DSI Host is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.EN = 0).
000: 16-bit configuration 1
001: 16-bit configuration 2
010: 16-bit configuration 3
011: 18-bit configuration 1
100: 18-bit configuration 2
101: 24-bit

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Bit 0 DSIM: DSI mode
This bit selects the mode for the video transmission. It can be changed when DSI Host is
stopped (DSI_CR.EN = 0).
0: Video mode
1: Adapted command mode

44.16.2

DSI Wrapper control register (DSI_WCR)
Address offset: 0x0404
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

17

16

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

Res.

Res.

Res.

Res.

Res.

3

2

1

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

5

4

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

Res.

DSIEN LTDCEN SHTDN
rw

rs

0
COLM

rw

rw

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 DSIEN: DSI enable
This bit enables the DSI Wrapper.
0: DSI disabled
1: DSI enabled
Bit 2 LTDCEN: LTDC enable
This bit enables the LTDC for a frame transfer in adapted command mode.
0: LTDC disabled
1: LTDC enabled
Bit 1 SHTDN: Shutdown
This bit controls the display shutdown in video mode.
0: Display ON
1: Display OFF
Bit 0 COLM: Color mode
This bit controls the display color mode in video mode.
0: Full color mode
1: Eight color mode

44.16.3

DSI Wrapper interrupt enable register (DSI_WIER)
Address offset: 0x0408
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

PLLUIE

PLLLIE

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ERIE

TEIE

rw

rw

rw

rw

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

Bits 31:11 Reserved, must be kept at reset value.
Bit 10 PLLUIE: PLL unlock interrupt enable
This bit enables the PLL unlock interrupt.
0: PLL unlock interrupt disabled
1: PLL unlock interrupt enabled
Bit 9 PLLLIE: PLL lock interrupt enable
This bit enables the PLL lock interrupt.
0: PLL lock interrupt disabled
1: PLL lock interrupt enabled
Bits 8:2 Reserved, must be kept at reset value.
Bit 1 ERIE: End of refresh interrupt enable
This bit enables the end of refresh interrupt.
0: End of refresh interrupt disabled
1: End of refresh interrupt enabled
Bit 0 TEIE: Tearing effect interrupt enable
This bit enables the tearing effect interrupt.
0: Tearing effect interrupt disabled
1: Tearing effect interrupt enabled

44.16.4

DSI Wrapper interrupt and status register (DSI_WISR)
Address offset: 0x040C
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

17

16

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

Res.

Res.

Res.

Res.

Res.

10

9

15

14

13

12

11

Res.

Res.

Res.

Res.

Res.

PLLUIF PLLLIF
r

r

8

7

6

5

4

3

2

1

0

PLLLS

Res.

Res.

Res.

Res.

Res.

BUSY

ERIF

TEIF

r

r

r

r

Bits 31:11 Reserved, must be kept at reset value.
Bit 10 PLLUIF: PLL unlock interrupt flag
This bit is set when the PLL becomes unlocked.
0: No PLL unlock event occurred
1: PLL unlock event occurred
Bit 9 PLLLIF: PLL lock interrupt flag
This bit is set when the PLL becomes locked.
0: No PLL lock event occurred
1: PLL lock event occurred
Bit 8 PLLLS: PLL lock status
This bit is set when the PLL is locked and cleared when it is unlocked.
0: PLL is unlocked.
1: PLL is locked.
Bits 7:3 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Bit 2 BUSY: Busy flag
This bit is set when the transfer of a frame in adapted command mode is ongoing.
0: No transfer ongoing
1: Transfer ongoing
Bit 1 ERIF: End of refresh interrupt flag
This bit is set when the transfer of a frame in adapted command mode is finished.
0: No end of refresh event occurred
1: End of refresh event occurred
Bit 0 TEIF: Tearing effect interrupt flag
This bit is set when a tearing effect event occurs.
0: No tearing effect event occurred
1: Tearing effect event occurred

44.16.5

DSI Wrapper interrupt flag clear register (DSI_WIFCR)
Address offset: 0x0410
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

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

Res.

CER
IF

CTE
IF

w

w

CPLLU CPLLL
IF
IF
w

w

Bits 31:11 Reserved, must be kept at reset value.
Bit 10 CPLLUIF: Clear PLL unlock interrupt flag
Writing 1 clears the PLLUIF flag in the DSI_WSR register.
0: No effect
1: Clears the PLLUIF flag
Bit 9 CPLLLIF: Clear PLL lock interrupt flag
Writing 1 clears the PLLLIF flag in the DSI_WSR register.
0: No effect
1: Clears the PLLUIF flag
Bits 8:2 Reserved, must be kept at reset value.
Bit 1 CERIF: Clear end of refresh interrupt flag
Writing 1 clears the ERIF flag in the DSI_WSR register.
0: No effect
1: Clears the PLLUIF flag
Bit 0 CTEIF: Clear tearing effect interrupt flag
Writing 1 clears the TEIF flag in the DSI_WSR register.
0: No effect
1: Clears the PLLUIF flag

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)

44.16.6

DSI Wrapper PHY configuration register 0 (DSI_WPCR0)
Address offset: 0x0418
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

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

FTXSM FTXSM
DL
CL
rw

rw

SWDL1 SWDL0 SWCL
rw

rw

rw

Bits 31:14 Reserved, must be kept at reset value.
Bit 13 FTXSMDL: Force in TX Stop mode the data lanes
This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit
mode. It causes the lane module to immediately jump to transmit control mode and to begin
transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA
sequence.
0: No effect
1: Force the data lanes in TX Stop mode
Bit 12 FTXSMCL: Force in TX Stop mode the clock lane
This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit
mode. It causes the lane module to immediately jump to transmit control mode and to begin
transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA
sequence.
0: No effect
1: Force the clock lane in TX Stop mode
Bits 11:9 Reserved, must be kept at reset value.
Bit 8 SWDL1: Swap data lane 1 pins
This bit swaps the pins on clock lane.
0: Regular clock lane pin configuration
1: Swapped clock lane pin
Bit 7 SWDL0: Swap data lane 0 pins
This bit swaps the pins on data lane 0.
0: Regular clock lane pin configuration
1: Swapped clock lane pin
Bit 6 SWCL: Swap clock lane pins
This bit swaps the pins on clock lane.
0: Regular clock lane pin configuration
1: Swapped clock lane pin
Bits 5:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

44.16.7

RM0456

DSI Wrapper regulator and PLL control register (DSI_WRPCR)
Address offset: 0x0430
Reset value: 0x0000 0000

31

30

Res.

29

28

27

26

25

BC[1:0]

24

23

22

21

20

19

18

ODF[8:0]

17

16

IDF[8:5]

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

5

4

3

2

1

0

Res.

PLLEN

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

IDF[4:0]
rw

NDIV[8:0]
rw

Bit 31 Reserved, must be kept at reset value.
Bits 30:29 BC[1:0]: Band control
This field selects the VCO frequency band.
00: 500 to 800 MHz
01: 800 to 1000 MHz
Others: Reserved
Bits 28:20 ODF[8:0]: PLL output division factor
This field configures the PLL output division factor.
0: PLL output divided by 1
1: PLL output divided by 1
2: PLL output divided by 2
...
511: PLL output divided by 511
Bits 19:11 IDF[8:0]: PLL input division factor
This field configures the PLL input division factor.
0: PLL input divided by 1
1: PLL input divided by 1
2: PLL input divided by 2
...
511: PLL input divided by 511
Bits 10:2 NDIV[8:0]: PLL loop division factor
This field configures the PLL loop division factor.
0: PLL loop divided by 1x2
1: PLL loop divided by 1x2
2: PLL loop divided by 2x2
...
511: PLL loop divided by 511x2
Bit 1 Reserved, must be kept at reset value.
Bit 0 PLLEN: PLL enable
This bit enables the D-PHY PLL.
0: PLL disabled
1: PLL enabled

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

DSI Host (DSI)

44.16.8

DSI Wrapper PLL tuning register (DSI_WPTR)
Address offset: 0x0434
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

rw

rw

rw

rw

rw

LPF[3:0]
rw

CP[3:0]
rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:12 LPF[3:0]: Loop filter
This field controls the PLL loop filter, it must be configured according to the PFD frequency
range:
0000: 2.0 to 4.4 MHz
0001: 4.4 to 30.9 MHz
0010: 30.9 to 50 MHz
Others: Reserved
Bits 11:8 CP[3:0]: Charge pump
This field controls the PLL charge pump, it must be configured according to the PFD
frequency range and the LPF value:
0000: 2.0 to 4.4 MHz and 14.1 to 30.9 MHz
0001: 4.4 to 14.1 MHz
0010: 45.7 to 50 MHz
0011: 30.9 to 45.7 MHz
Others: Reserved
Bits 7:0 Reserved, must be kept at reset value.

44.17

DSI bias registers

44.17.1

DSI bias configuration register (DSI_BCFGR)
Address offset: 0x0808
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PWRUP

Res.

Res.

Res.

Res.

Res.

Res.

rw

Bits 31:7 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Bit 6 PWRUP: Power-up
This bit powers-up the reference bias for the MIPI D-PHY
0: Reference bias is powered down.
1: Reference bias is powered up.
Bits 5:0 Reserved, must be kept at reset value.

44.18

D-PHY registers

44.18.1

DSI D-PHY clock band control register (DSI_DPCBCR)
Address offset: 0x0C04
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

17

16

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

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

BC[4:0]
rw

rw

rw

rw

2

1

0

Res.

Res.

Res.

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:3 BC[4:0]: Band control
This field selects the frequency band used by the D-PHY.
00000: 80 to 100 MHz
00001: 100 to 120 MHz
00010: 120 to 160 MHz
00011: 160 to 200 MHz
00100: 200 to 240 MHz
00101: 240 to 320 MHz
00110: 320 to 390 MHz
00111: 390 to 450 MHz
01000: 450 to 510 MHz
Others: Reserved
Bits 2:0 Reserved, must be kept at reset value.

44.18.2

DSI D-PHY clock skew rate control register (DSI_DPCSRCR)
Address offset: 0x0C34
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

17

16

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

Res.

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
rw

rw

rw

rw

rw

rw

rw

<!-- pagebreak -->

SRC[7:0]

RM0456 Rev 6

rw

RM0456

DSI Host (DSI)

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 SRC[7:0]: Slew rate control
This field selects the slew rate for HS-TX speed.
0x0E: 80 to 750 Mbit/s
Others: Reserved

44.18.3

DSI D-PHY data lane 0 HS offset control register
(DSI_DPDL0HSOCR)
Address offset: 0x0C5C
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

17

16

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

Res.

Res.

Res.

Res.

Res.

7

6

5

4

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

HSPRPO[3:0]
rw

rw

rw

3

2

1

0

Res.

Res.

Res.

Res.

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 HSPRPO[3:0]: HS prepare offset
This field selects the offset in lane byte clock to be added to the HS prepare timing. The
offset is dependent on the frequency band selected for the D-PHY
0000: 100 to 120 MHz - 120 to 160 MHz - 240 to 320 MHz
0001: 80 to 100 MHz - 160 to 200 MHz - 200 to 240 MHz - 320 to 390 MHz
0010: 390 to 450 MHz - 450 to 510 MHz
Others: Reserved
Bits 3:0 Reserved, must be kept at reset value.

44.18.4

DSI D-PHY data lane 0 HS LPX offset control register
(DSI_DPDL0LPXOCR)
Address offset: 0x0C60
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

17

16

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

Res.

Res.

Res.

Res.

Res.

3

2

1

0

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

5

4

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

Res.

LPXO[3:0]
rw

rw

rw

rw

Bits 31:4 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Bits 3:0 LPXO[3:0]: LPX offset
This field selects the offset added to fine tune the delay associated to the following states:
INIT_STATE, STOP_STATE, LP01_STATE and LP11_STATE.
This field is a 4-bit signed value in complement 2 format (-8 to +7 range).
The LPX timing is composed of a unsigned fixed 7-bit value dependent of the frequency band
selected for the D-PHY and the 4-bit signed value of this field.
The LPX timing is expressed in lane byte clock period.
The LPX fixed value is:
80 to 120 MHz: 7’h01
120 to 160 MHz: 7’h02
160 to 320 MHz: 7’h03
320 to 450 MHz: 7’h04
450 to 510 MHz: 7’h05
As the resulting LPX timing is an unsigned 7-bit value, the user must take care of underflow
when the value is negative (complement 2 format).

44.18.5

DSI D-PHY data lane 0 band control register
(DSI_DPDL0BCR)
Address offset: 0x0C70
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

17

16

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

Res.

Res.

Res.

Res.

Res.

4

3

2

1

0

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

5

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

BC[4:0]
rw

Bits 31:5 Reserved, must be kept at reset value.
Bits 4:0 BC[4:0]: Band control
This field selects the frequency band used by the D-PHY.
00000: 80 to 100 MHz
00001: 100 to 120 MHz
00010: 120 to 160 MHz
00011: 160 to 200 MHz
00100: 200 to 240 MHz
00101: 240 to 320 MHz
00110: 320 to 390 MHz
00111: 390 to 450 MHz
01000: 450 to 510 MHz
Others: Reserved

<!-- pagebreak -->

RM0456 Rev 6

rw

rw

RM0456

DSI Host (DSI)

44.18.6

DSI D-PHY data lane 0 skew rate control register
(DSI_DPDL0SRCR)
Address offset: 0x0CA0
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

17

16

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

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SRC[7:0]
rw

rw

rw

rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 SRC[7:0]: Slew rate control
This field selects the slew rate for HS-TX speed.
0x0E: 80 to 750 Mbit/s
Others: Reserved

44.18.7

DSI D-PHY data lane 1 HS offset control register
(DSI_DPDL1HSOCR)
Address offset: 0x0CF4
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

17

16

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

Res.

Res.

Res.

Res.

Res.

7

6

5

4

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

HSPRPO[3:0]
rw

rw

rw

3

2

1

0

Res.

Res.

Res.

Res.

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 HSPRPO[3:0]: HS prepare offset
This field selects the offset in lane byte clock to be added to the HS prepare timing. The
offset is dependent on the frequency band selected for the D-PHY
0000: 100 to 120 MHz - 120 to 160 MHz - 240 to 320 MHz
0001: 80 to 100 MHz - 160 to 200 MHz - 200 to 240 MHz - 320 to 390 MHz
0010: 390 to 450 MHz - 450 to 510 MHz
Others: Reserved
Bits 3:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

44.18.8

RM0456

DSI D-PHY data lane 1 HS LPX offset control register
(DSI_DPDL1LPXOCR)
Address offset: 0x0CF8
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

17

16

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

Res.

Res.

Res.

Res.

Res.

3

2

1

0

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

5

4

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

Res.

LPXO[3:0]
rw

rw

rw

rw

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 LPXO[3:0]: LPX offset
This field selects the offset added to fine tune the delay associated to the following states:
INIT_STATE, STOP_STATE, LP01_STATE and LP11_STATE.
This field is a 4-bit signed value in complement 2 format.
The LPX timing is composed of a unsigned fixed 7-bit value dependent of the frequency band
selected for the D-PHY and the 4-bit signed value of this field.
The offset is expressed in lane byte clock period.
As the resulting LPX timing is an unsigned 7-bit value, the user must take care of underflow
when the value is negative (complement 2 format).

44.18.9

DSI D-PHY data lane 1 band control register (DSI_DPDL1BCR)
Address offset: 0x0D08
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

17

16

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

Res.

Res.

Res.

Res.

Res.

4

3

2

1

0

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

5

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

BC[4:0]
rw

Bits 31:5 Reserved, must be kept at reset value.
Bits 4:0 BC[4:0]: Band control
This field selects the frequency band used by the D-PHY.
00000: 80 to 100 MHz
00001: 100 to 120 MHz
00010: 120 to 160 MHz
00011: 160 to 200 MHz
00100: 200 to 240 MHz
00101: 240 to 320 MHz
00110: 320 to 390 MHz
00111: 390 to 450 MHz
01000: 450 to 510 MHz
Others: Reserved

<!-- pagebreak -->

RM0456 Rev 6

rw

rw

RM0456

DSI Host (DSI)

44.18.10 DSI D-PHY data lane 1 skew rate control register
(DSI_DPDL1SRCR)
Address offset: 0x0D38
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

17

16

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

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SRC[7:0]
rw

rw

rw

rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 SRC[7:0]: Slew rate control
This field selects the slew rate for HS-TX speed.
0x0E: 80 to 750 Mbit/s
Others: Reserved

RM0456 Rev 6

<!-- pagebreak -->

