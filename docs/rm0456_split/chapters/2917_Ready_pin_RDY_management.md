3086

FD controller area network (FDCAN)

RM0456
Figure 881. CAN subsystem.
Configuration

32-bit APB bus

fdcan_ker_ck

PDIV[3:0]
/ 1..30

CFG_APB

Subsystem
Configuration
register

CKDIV
fdcan_tq_ck

Interrupts
interface

fdcan1_intr0_it
fdcan1_intr1_it

CAN core
Sync

FDCAN1_RX

Ctrl_APB

Control and
Configuration
registers

FDCAN1_TX

Sync
Tx Req

Tx State Rx State
TX Handler

fdcan_pclk
TX prioritization
Frame synchronization

Message RAM
interface

RX Handler
Acceptance filter

fdcan_ts[0:15]
CANFDL

RAM_APB

RAM
Controller / Arbitrer
APB clock domain
Message RAM
Buffers
FIFOs
Filters

Kernel clock domain

MSv51817V4

<!-- pagebreak -->

RM0456 Rev 6

RM0456

70.2

FD controller area network (FDCAN)

FDCAN main features
•

Conform with CAN protocol version 2.0 part A, B, and ISO 11898-1: 2015

•

CAN FD with maximum 64 data bytes supported

•

CAN error logging

•

AUTOSAR and J1939 support

•

Improved acceptance filtering

•

Two receive FIFOs of three payloads each (up to 64 bytes per payload)

•

Separate signaling on reception of high priority messages

•

Configurable transmit FIFO/queue of three payloads (up to 64 bytes per payload)

•

Transmit event FIFO

•

Programmable loop-back test mode

•

Maskable module interrupts

•

Two clock domains: APB bus interface and CAN core kernel clock

•

Power-down support

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

RM0456

70.3

FDCAN functional description

70.3.1

FDCAN block diagram
Figure 882. FDCAN block diagram
fdcan_tq_ck

Interrupts
interface

fdcan_intr0_it
fdcan_intr1_it

CAN core
Sync

FDCAN_RX

Ctrl_APB

Control and
Configuration
registers

FDCAN_TX

Sync
Tx Req

Tx State Rx State
TX Handler

fdcan_pclk
TX prioritization
Frame Synchro

Message RAM
interface

RX Handler
Acceptance filter

fdcan_ts[0:15]
CANFDL

Kernel clock domain

APB clock domain
MSv51819V2

Dual interrupt lines
The FDCAN peripheral provides two interrupt lines, fdcan_intr0_it and fdcan_intr1_it.
By programming the EINT0 and EINT1 bits of the FDCAN_ILE register, the interrupt lines
can be independently enabled or disabled.

CAN core
The CAN core contains the protocol controller and receive/transmit shift registers. It handles
all ISO 11898-1: 2015 protocol functions and supports both 11-bit and 29-bit identifiers.

Sync
This block synchronizes signals from the APB clock domain to the CAN kernel clock domain
and vice versa.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)

Tx handler
The Tx handler controls the message transfer from the message RAM to the CAN core. A
maximum of three Tx buffers is available for transmission. The Tx buffer can be used as Tx
FIFO or as a Tx queue. Tx event FIFO stores Tx timestamps together with the
corresponding message ID. Transmit cancellation is also supported.

Rx handler
The Rx handler controls the transfer of received messages from the CAN core to the
external message RAM. The Rx handler supports two receive FIFOs, for storage of all
messages that have passed acceptance filtering. An Rx timestamp is stored together with
each message. Up to 28 filters can be defined for 11-bit IDs; up to eight filters for 29-bit IDs.

APB interface
The APB interface connects the FDCAN to the APB bus for configuration registers,
controller configuration, and RAM access.

Message RAM interface
The message RAM interface connects the FDCAN access to an external 1-Kbyte message
RAM through a RAM controller/arbiter.

70.3.2

FDCAN pins and internal signals
The CAN subsystem I/O signals and pins are detailed, respectively, in Table 717, Table 718,
and Figure 881.
Table 717. CAN subsystem I/O signals
Name
fdcan_ker_ck
fdcan_pclk
fdcan_intr0_it
fdcan_intr1_it

Type
Digital input

Digital output

fdcan_ts[0:15]

-

APB interface

Digital input/output

Description
CAN subsystem kernel clock input
CAN subsystem APB interface clock input
FDCAN interrupt0
FDCAN interrupt1
External timestamp vector
Single APP with multiple psel for configuration, control
and RAM access

Table 718. CAN subsystem I/O pins
Name

Type

Description

FDCAN_RX

Digital input

FDCAN receive pin

FDCAN_TX

Digital output

FDCAN transmit pin

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

70.3.3

RM0456

Bit timing
The bit timing logic monitors the serial bus-line and performs sampling and adjustment of
the sample point by synchronizing on the start-bit edge and resynchronizing on the following
edges.
As shown in Figure 883, this operation can be explained simply by splitting the bit time in
three segments, as follows:
•

Synchronization segment (SYNC_SEG): a bit change is expected to occur within this
time segment, having a fixed length of one time quantum (1 × tq).

•

Bit segment 1 (BS1): defines the location of the sample point. It includes the
PROP_SEG and PHASE_SEG1 of the CAN standard. Its duration is programmable
from 1 to 16 time quanta, but can be automatically lengthened to compensate for
positive phase drifts due to differences in the frequency of various nodes of the
network.

•

Bit segment 2 (BS2): defines the location of the transmit point. It represents the
PHASE_SEG2 of the CAN standard, its duration is programmable between one and
eight time quanta, but can also be automatically shortened to compensate for negative
phase drifts.
Figure 883. Bit timing

SyncSeg

Bit segment 1 (BS1)

Bit segment 2 (BS2)

tSyncSeg

tBS1

tBS2
Sample
point

Transmit
point
MS47283V1

The baud rate is the inverse of the bit time (baud rate = 1 / bit time), which, in turn, is the
sum of three components (see Figure 883):
bit time = tSyncSeg + tBS1 + tBS2
Where:
–

For the nominal bit time
tq = (NBRP[8:0] + 1) × tfdcan_tq_clk
tSyncSeg = 1 × tq
tBS1 = tq × (NTSEG1[7:0] + 1)
tBS2 = tq × (NTSEG2[6:0] + 1)
Where NBRP[8:0], NTSEG1[7:0], and NTSEG2[6:0] bitfields belong to the
FDCAN_NBTP register.

–

For the data bit time
tq = (DBRP[4:0] + 1) × tfdcan_tq_clk
tSyncSeg = 1 × tq
tBS1 = tq × (DTSEG1[4:0] + 1)
tBS2 = tq × (DTSEG2[3:0] + 1)

<!-- pagebreak -->

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)
Where DBRP[4:0], DTSEG1[4:0], and DTSEG2[3:0] belong to the FDCAN_DBTP
register.
The (re)synchronization jump width (SJW) defines an upper bound for the amount of
lengthening or shortening of the bit segments. It is programmable between one and four
time quanta.
A valid edge is defined as the first transition in a bit time from dominant to recessive bus
level, provided the controller itself does not send a recessive bit.
If a valid edge is detected in BS1 instead of SYNC_SEG, BS1 is extended by up to SJW, so
that the sample point is delayed.
Conversely, if a valid edge is detected in BS2 instead of SYNC_SEG, BS2 is shortened by
up to SJW, so that the transmit point is moved earlier.
As a safeguard against programming errors, the configuration of the bit timing register is
only possible while the device is in Standby mode. The FDCAN_DBTP and FDCAN_NBTP
registers (dedicated, respectively, to data and nominal bit timing) are only accessible when
the CCE and INIT of the FDCA_CCCR register are set.
The FDCAN requires that the CAN time quanta clock is always below or equal to the APB
clock (fdcan_tq_ck < fdcan_pclk).

Note:

For a detailed description of the CAN bit timing and resynchronization mechanism, refer to
the ISO 11898-1 standard.

70.3.4

Operating modes
Configuration
Access to peripheral version, hardware, and input clock divider configuration. When the
clock divider is set to 0, the primary input clock is used as it is.

Software initialization
Software initialization is started by setting the INIT bit of the FDCAN_CCCR register, by
software, by a hardware reset, or by entering bus-off state. While the INIT bit is set,
message transfers from and to the CAN bus are stopped, and the status of the CAN bus
output FDCAN_TX is recessive (high). The EML (error management logic) counters are
unchanged. Setting the INIT bit does not change any configuration register. Clearing INIT bit
of FDCAN_CCCR finishes the software initialization. Afterwards the bit stream processor
(BSP) synchronizes itself to the data transfer on the CAN bus by waiting for the occurrence
of a sequence of 11 consecutive recessive bits (bus-idle) before it can take part in bus
activities and start the message transfer.
Access to the FDCAN configuration registers is only enabled when the INIT bit and the CCE
bit of the FDCAN_CCCR register are both set.
The CCE bit of the FDCAN_CCCR register can only be set/cleared while the INIT bit of
FDCAN_CCCR is set. The CCE bit is automatically cleared when the INIT bit is cleared.

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

RM0456

The following registers are reset when the CCE bit of the FDCAN_CCCR register is set:
•

FDCAN_HPMS: High priority message status

•

FDCAN_RXF0S: Rx FIFO 0 status

•

FDCAN_RXF1S: Rx FIFO 1 status

•

FDCAN_TXFQS: Tx FIFO/queue status

•

FDCAN_TXBRP: Tx buffer request pending

•

FDCAN_TXBTO: Tx buffer transmission occurred

•

FDCAN_TXBCF: Tx buffer cancellation finished

•

FDCAN_TXEFS: Tx event FIFO status

The timeout counter value (TOC[15:0] bit of the FDCAN_TOCV register) is preset to the
value configured by the TOP[15:0] of the FDCAN_TOCC register when the CCE bit of the
FDCAN_CCCR is set.
In addition, the state machines of the Tx handler and Rx handler are held in idle state while
the CCE bit is set.
The following registers can be written only when the CCE bit is cleared:
•

FDCAN_TXBAR: Tx buffer add request

•

FDCAN_TXBCR: Tx buffer cancellation request

The TEST and the MON bits of the FDCAN_CCCR register can be set only by software
while the INIT and the CCE bits of the FDCAN_CCCR register are both set. Both bits can be
reset at any time. The DAR bit of FDCAN_CCCR can only be set/cleared while the INIT and
CCE bits are both set.

Normal operation
The FDCAN default operating mode after hardware reset is event-driven CAN
communication. TT operation mode is not supported.
Once the FDCAN is initialized and the INIT bit of the FDCAN_CCCR register is cleared, the
FDCAN synchronizes itself to the CAN bus and is ready for communication.
After passing the acceptance filtering, received messages including message ID and DLC
are stored into the Rx FIFO 0 or Rx FIFO 1.
For messages to be transmitted, the Tx FIFO or the Tx queue can be initialized or updated.
Automated transmission on reception of remote frames is not supported.

CAN FD operation
There are two variants in the FDCAN protocol:
•

Long frame mode (LFM), where the data field of a CAN frame may be longer than eight
bytes.

•

Fast frame mode (FFM), where the control field, data field, and CRC field of a CAN
frame are transmitted with a higher bit rate compared to the beginning and to the end of
the frame.

The fast frame mode can be used in combination with the long frame mode.
The previously reserved bit in CAN frames with 11-bit identifiers and the first previously
reserved bit in CAN frames with 29-bit identifiers are decoded as FDF bit: FDF recessive
signifies a CAN FD frame, while FDF dominant signifies a classic CAN frame.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)
In a CAN FD frame, the two bits following FDF (res and BRS) decide whether the bit rate
inside this CAN FD frame is switched. A CAN FD bit rate switch is signified by res dominant
and BRS recessive. The coding of res recessive is reserved for future expansion of the
protocol. In case the FDCAN receives a frame with FDF recessive and res recessive, it
signals a protocol exception event by setting the PXE bit of the FDCAN_PSR register. When
protocol exception handling is enabled (PXHD = 0 in FDCAN_CCCR), this causes the
operation state to change from receiver (ACT[1:0] = 10 in FDCAN_PSR) to integrating
(ACT[1:0] = 00 in FDCAN_PSR) at the next sample point. If protocol exception handling is
disabled (PXHD = 1 in FDCAN_CCCR), the FDCAN treats a recessive res bit as a form
error and responds with an error frame.
CAN FD operation is enabled by programming the FDOE bit of the FDCAN_CCCR register.
In case FDOE = 1, transmission and reception of CAN FD frames are enabled.
Transmission and reception of classic CAN frames are always possible. Whether a CAN FD
frame or a classic CAN frame is transmitted can be configured via the FDF bit in the
respective Tx buffer element. With FDOE = 0, received frames are interpreted as classic
CAN frames, which leads to the transmission of an error frame when receiving a CAN FD
frame. When CAN FD operation is disabled, no CAN FD frames are transmitted even if the
FDF bit of a Tx buffer element is set. The FDOE and BRSE bits of the FDCAN_CCCR
register can only be changed while the INIT and CCE bits are both set.
With FDOE = 0, the setting of the FDF and BRS bits is ignored, and frames are transmitted
in classic CAN format. With FDOE = 1 and BRSE = 0, only the FDF bit of a Tx buffer
element is evaluated. With FDOE = 1 and BRSE = 1, transmission of CAN FD frames with
bit rate switching is enabled. All Tx buffer elements with FDF and BRS bits set are
transmitted in CAN FD format with bit rate switching.
A mode change during CAN operation is recommended only under the following conditions:
•

The failure rate in the CAN FD data phase is significant higher than in the CAN FD
arbitration phase. In this case, disable the CAN FD bit rate switching option for
transmissions.

•

During system startup, all nodes transmit classic CAN messages until it is verified that
they are able to communicate in CAN FD format. If this is true, all nodes switch to CAN
FD operation.

•

Wake-up messages in CAN partial networking have to be transmitted in classic CAN
format.

•

End-of-line programming in case not all nodes are CAN FD capable. Non-CAN FD
nodes are held in silent mode until programming is complete. Then all nodes switch
back to classic CAN communication.

In the FDCAN format, the coding of the DLC differs from that of the standard CAN format.
The DLC codes 0 to 8 have the same coding as in standard CAN, the codes 9 to 15 (that in
standard CAN all code a data field of 8 bytes) are coded according to Table 719.
:

Table 719. DLC coding in FDCAN
DLC

9

10

11

12

13

14

15

Number of data bytes

12

16

20

24

32

48

64

In CAN FD fast frames, the bit timing is switched inside the frame, after the BRS (bit rate
switch) bit, if this bit is recessive. Before the BRS bit, in the FDCAN arbitration phase, the
standard CAN bit timing is used as defined by the FDCAN_DBTP register. In the following

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

RM0456

FDCAN data phase, the fast CAN bit timing is used as defined by the FDCAN_DBTP
register. The bit timing is switched back from the fast timing at the CRC delimiter or when an
error is detected, whichever occurs first.
The maximum configurable bit rate in the CAN FD data phase depends on the FDCAN
kernel clock frequency. For example, with an FDCAN kernel clock frequency of 20 MHz and
the shortest configurable bit time of four time quanta (tq), the bit rate in the data phase is
5 Mbit/s.
In both data frame formats (CAN FD long frames and CAN FD fast frames), the value of bit
ESI (error status indicator) is determined by the transmitter error state at the start of the
transmission. If the transmitter is error passive, ESI is transmitted recessive, else it is
transmitted dominant. In CAN FD remote frames, the ESI bit is always transmitted
dominant, independent of the transmitter error state. The data length code of CAN FD
remote frames is transmitted as 0.
In case an FDCAN Tx buffer is configured for FDCAN transmission with DLC > 8, the first
eight bytes are transmitted as configured in the Tx buffer while the remaining part of the
data field is padded with 0xCC. When the FDCAN receives a FDCAN frame with DLC > 8,
the first eight bytes of that frame are stored into the matching Rx FIFO. The remaining bytes
are discarded.

Transceiver delay compensation
During the data phase of an FDCAN transmission, only one node is transmitting, all others
are receivers. The length of the bus line has no impact. When transmitting via pin
FDCAN_TX, the protocol controller receives the transmitted data from its local CAN
transceiver via pin FDCAN_RX. The received data is delayed by the CAN transceiver loop
delay. If this delay is greater than TSEG1 (time segment before sample point), and a bit
error is detected. Without transceiver delay compensation, the bit rate in the data phase of
an FDCAN frame is limited by the transceiver loop delay.
The FDCAN implements a delay compensation mechanism to compensate the CAN
transceiver loop delay, thereby enabling transmission with higher bit rates during the
FDCAN data phase independent of the delay of a specific CAN transceiver.
To check for bit errors during the data phase of transmitting nodes, the delayed transmit
data is compared against the received data at the secondary sample point (SSP). If a bit
error is detected, the transmitter reacts on this bit error at the next following regular sample
point. During the arbitration phase, the delay compensation is always disabled.
The transmitter delay compensation enables configurations where the data bit time is
shorter than the transmitter delay. This is enabled by setting the TDC bit of the
FDCAN_DBTP register, and described in detail in the ISO11898-1 specification.
The received bit is compared against the transmitted bit at the SSP. The SSP position is
defined as the sum of the measured delay from the FDCAN transmit output pin FDCAN_TX
through the transceiver to the receive input pin FDCAN_RX plus the transmitter delay
compensation offset as configured by TDCO[6:0] of FDCAN_TDCR. The transmitter delay
compensation offset is used to adjust the position of the SSP inside the received bit (for
example, half of the bit time in the data phase). The position of the secondary sample point
is rounded down to the next integer number of mtq (minimum time quantum, one period of
fdcan_tq_ck clock).
The TDCV[6:0] bitfield of the FDCAN_PSR register shows the actual transmitter delay
compensation value. TDCV[6:0] is cleared when the INIT is set in the FDCAN_CCCR. It is

<!-- pagebreak -->

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)
updated at each transmission of an FD frame while the TDC bit of the FDCAN_DBTP
register is set.
The following boundary conditions have to be considered for the transmitter delay
compensation implemented in the FDCAN:
•

The sum of the measured delay from FDCAN_Tx to FDCAN_Rx and the configured
transmitter delay compensation offset TDCO[6:0] has to be lower than 6-bit times in the
data phase.

•

The sum of the measured delay from FDCAN_TX to FDCAN_RX and the configured
transmitter delay compensation offset TDCO[6:0] has to be lower than or equal to
127 × mtq. If the sum exceeds this value, the maximum value (127 × mtq) is used for
transmitter delay compensation.

•

The data phase ends at the sample point of the CRC delimiter, which stops checking
received bits at the SSPs.

If transmitter delay compensation is enabled by setting the TDC bit of the FDCAN_DBTP;
the measurement is started within each transmitted CAN FD frame at the falling edge of bit
FDF to bit res. The measurement is stopped when this edge is seen at the receive input pin
FDCAN_TX of the transmitter. The resolution of this measurement is one mtq.
Figure 884. Transceiver delay measurement

Res.

FDF
FDCAN_TX

FDCAN_RX

BRS

ESI

Transmitter delay

Arbitration phase

DLC
Data phase

Arbitration phase

Start

Data phase

Stop

Delay

fdcan_tq_ck

Delay counter

+

SSP position

TDCR.TDCO
Delay compensation offset
MSv41483V2

To avoid that a dominant glitch inside the received FDF bit ends the delay compensation
measurement before the falling edge of the received res bit (resulting in a too early SSP
position), the use of a transmitter delay compensation filter window can be enabled by
programming the TDCF[6:0] bitfield of the FDCAN_TDCR register. This defines a minimum
value for the SSP position. Dominant edges on FDCAN_RX that would result in an earlier
SSP position are ignored for transmitter delay measurement. The measurement is stopped
when the SSP position is at least TDCF[6:0] and FDCAN_RX is low.

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

RM0456

Restricted operation mode
In restricted operation mode, the node is able to receive data and remote frames, and to
give acknowledge to valid frames, but it does not send data frames, remote frames, active
error frames, or overload frames. In case of an error condition or overload condition, it does
not send dominant bits. Instead, it waits for the occurrence of a bus-idle condition to
resynchronize itself to the CAN communication. The error counters (REC[6:0] and TEC[7:0]
in FDCAN_ECR) are frozen while the error logging (CEL[7:0]) is active. The software can
set the FDCAN into restricted operation mode by setting the ASM bit of FDCAN_CCCR. The
bit can only be set by software when both CCE and INIT bits are set in FDCAN_CCCR. The
bit can be cleared by software at any time.
Restricted operation mode is automatically entered when the Tx handler is not able to read
data from the message RAM in time. To leave restricted operation mode, the software has to
clear the ASM bit of FDCAN_CCCR.
The restricted operation mode can be used in applications that adapt themselves to different
CAN bit rates. In this case, the application tests different bit rates and leaves the restricted
operation mode after it has received a valid frame.
Note:

The restricted operation mode must not be combined with the loop-back mode (internal or
external).

Bus monitoring mode
The FDCAN is set in bus monitoring mode by setting the MON bit of the FDCAN_CCCR
register. In bus monitoring mode (for more details refer to ISO11898-1, 10.12 bus
monitoring), the FDCAN is able to receive valid data frames and valid remote frames, but
cannot start a transmission. In this mode, it sends only recessive bits on the CAN bus. If the
FDCAN is required to send a dominant bit (ACK bit, overload flag, active error flag), the bit is
rerouted internally so that the FDCAN can monitor it, even if the CAN bus remains in
recessive state. In bus monitoring mode, the FDCAN_TXBRP register is held in reset state.
The bus monitoring mode can be used to analyze the traffic on a CAN bus without affecting
it by the transmission of dominant bits. Figure 885 shows the connection of FDCAN_TX and
FDCAN_RX signals to the FDCAN in bus monitoring mode.
Figure 885. Pin control in bus monitoring mode
FDCAN_TX

FDCAN_RX

=1

Tx

Rx

FDCAN
MS41462V1

<!-- pagebreak -->

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)

Disabled automatic retransmission mode (DAR)
According to the CAN specification (see ISO11898-1, 6.3.3 Recovery Management), the
FDCAN provides means for automatic retransmission of frames that have lost arbitration or
have been disturbed by errors during transmission. By default, automatic retransmission is
enabled. The DAR mode can be disabled through the DAR bit of the FDCAN_CCCR
register.
Frame transmission in DAR mode
In DAR mode, all transmissions are automatically canceled after they have been started on
the CAN bus. A Tx buffer Tx request pending bit (TRPx in FDCAN_TXBRP) is reset after
successful transmission, when a transmission has not yet been started at the point of
cancellation, when it has been aborted due to lost arbitration, or when an error has occurred
during frame transmission.
•

•

•

Successful transmission
–

The corresponding Tx buffer transmission occurred bit TOx is set in
FDCAN_TXBTO register.

–

The corresponding Tx buffer cancellation finished bit CFx is cleared in the
FDCAN_TXBCF register.

Successful transmission in spite of cancellation
–

The corresponding Tx buffer transmission occurred bit TOx is set in the
FDCAN_TXBTO register.

–

The corresponding Tx buffer cancellation finished bit CFx is set in the
FDCAN_TXBCF register.

Arbitration loss or frame transmission disturbed
–

The corresponding Tx buffer transmission occurred bit TOx is cleared in the
FDCAN_TXBTO register.

–

The corresponding Tx buffer cancellation finished bit CFx is set in the
FDCAN_TXBCF register.

In case of a successful frame transmission, and if the storage of Tx events is enabled, a Tx
event FIFO element is written with event type ET = 10 (transmission in spite of cancellation).

Power-down (Sleep mode)
Power-down entry
The FDCAN can be set into power-down mode controlled by setting the CSR bit of the
FDCAN_CCCR register. As long as the clock stop request is active, CSR is read as 1.
When all pending transmission requests have completed, the FDCAN waits until the busidle state is detected. The FDCAN then sets the INIT bit of the FDCAN_CCCR register to
prevent any further CAN transfers. Now, the FDCAN acknowledges that it is ready for
power-down by setting the CSA bit of the FDCAN_CCCR register. In this state, before the
clocks are switched off, further register accesses can be made. A write access to the INIT
bit has no effect. Now, the module clock inputs can be switched off.
Power-down exit
To leave power-down mode, the application has to turn on the module clocks before clearing
the CSR bit. The FDCAN acknowledges this by clearing the CSA bit. Afterwards, the
application can restart CAN communication by clearing the INIT bit.

RM0456 Rev 6

<!-- pagebreak -->

