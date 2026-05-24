RM0456 Rev 6

RM0456

FD controller area network (FDCAN)
Figure 886. Pin control in loop-back mode

FDCANx_Tx

FDCAN_Rx

FDCANx_Tx

FDCAN_Rx

=1

Tx

Rx

FDCAN

Tx

Rx

FDCAN

External loop-back mode

Internal loop-back mode
MS41463V2

Timestamp generation
For timestamp generation, the FDCAN supplies a 16-bit wrap-around counter. A prescaler
(TCP[3:0] of FDCAN_TSCC) can be configured to clock the counter in multiples of CAN bit
times (1 to 16). The counter is readable via the TCV[15:0] bitfield of the FDCAN_TSCV
register. A write access to TSCV15:0] resets the counter to 0. When the timestamp counter
wraps around, the interrupt flag (TSW bit of FDCAN_ISR) is set.
On start of frame reception/transmission, the counter value is captured and stored into the
timestamp section of an Rx FIFO (RXTS[15:0]) or Tx event FIFO (TXTS[15:0]) element.
By programming TSS[1:0] of FDCAN_TSCC, a 16-bit timestamp can be used.

Debug mode behavior
In debug mode, the set/reset on read feature is automatically disabled during the debugger
register access, and enabled during normal MCU operation

Timeout counter
To signal timeout conditions for Rx FIFO 0, Rx FIFO 1, and the Tx event FIFO the FDCAN
supplies a 16-bit timeout counter. It operates as a down-counter and uses the same
prescaler controlled by TCP[3:0] of FDCAN_TSCC as the timestamp counter. The timeout
counter is configured via the FDCAN_TOCC register. The actual counter value can be read
from the TOC[15:0] bitfield of FDCAN_TOCV. The timeout counter can only be started while
the INIT bit of FDCAN_CCCR is cleared. It is stopped when INIT is set, for example, when
the FDCAN enters bus-off state.
The operation mode is selected by TOS[1:0] of FDCAN_TOCC. When operating in
continuous mode, the counter starts when INIT is cleared. A write to FDCAN_TOCV presets
the counter to the value configured by TOP[15:0] in FDCAN_TOCC and continues downcounting.
When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the
counter to the value configured by TOP[15:0]. Down-counting is started when the first FIFO
element is stored. Writing to FDCAN_TOCV has no effect.

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

RM0456

When the counter reaches 0, the TOO interrupt flag is set in the FDCAN_IR register. In
continuous mode, the counter is immediately restarted at TOP[15:0].
Note:

The clock signal for the timeout counter is derived from the CAN core sample point signal.
Therefore, the point in time where the timeout counter is decremented may vary due to the
synchronization/resynchronization mechanism of the CAN core. If the baud rate switch
feature in FDCAN is used, the timeout counter is clocked differently in the arbitration and
data fields.

70.3.5

Error management
As described in the CAN protocol, the error management is handled entirely by hardware
using the transmit error counter (the TEC[7:0] bitfield of the FDCAN error counter register
(FDCAN_ECR)) and the receive error counter (the REC[6:0] bitfield of the FDCAN error
counter register (FDCAN_ECR)). These values are incremented or decremented according
to the error condition. For detailed information on TEC[7:0] and REC[6:0] management,
refer to the CAN standard. Both values can be read by software to determine the stability of
the network.
The bus-off state is reached when TEC[7:0] is greater than 255. This state is also indicated
by the BO flag of the FDCAN protocol status register (FDCAN_PSR). In bus-off state, the
CAN is no longer able to transmit and receive messages. It has to wait at least for the
duration of the recovery sequence specified in the CAN standard (128 occurrences of 11
consecutive recessive bits monitored on FDCAN_RX input).
Figure 887. CAN error state diagram
When TEC or REC > 127

ERROR PASSIVE

ERROR ACTIVE

When TEC and REC < 128
When 128x11 recessive bits occur:

When TEC > 255
BUS OFF

ai15903

Note:

<!-- pagebreak -->

In initialization mode, the CAN does not monitor the FDCAN_RX signal, and therefore
cannot complete the recovery sequence. To recover from an error state, the CAN must
operate in normal mode.

RM0456 Rev 6

RM0456

70.3.6

FD controller area network (FDCAN)

Message RAM
The message RAM is 32-bit wide, and the FDCAN module is configured to allocate up to
212 words in it. It is not necessary to configure each of the sections shown in Figure 888.
Figure 888. Message RAM configuration
FLSSA 0x0000

11-bit filter

28 elements / 28 words

29-bit filter

8 elements / 16 words

Rx FIFO 0

3 elements / 54 words

Rx FIFO 1

3 elements / 54 words

Tx event FIFO

3 elements / 6 words

FLESA 0x0070

0x00B0

0x0188

EFSA 0x0260

TBSA 0x0278

Tx buffers

3 elements / 54 words

32 bits
MS47278V5

When the FDCAN addresses the message RAM, it addresses 32-bit words (aligned), not a
single byte. The RAM addresses are 32-bit words, that is, only bits 15 to 2 are evaluated,
the two least significant bits are ignored.
In case of multiple instances, the RAM start address for the FDCANn is computed by end
address + 4 of FDCANn - 1, and the FDCANn end address is computed by FDCANn start
address + 0x0350 - 4.
As an example, for two instances:
•

•

FDCAN1:
–

Start address 0x0000

–

End address 0x034C (as in Figure 888)

FDCAN2:
–

Start address = 0x034C (FDCAN1 end address) + 4 = 0x0350

–

End address = 0x0350 (FDCAN2 start address) + 0x0350 - 4 = 0x069C.

Rx handling
The Rx handler controls the acceptance filtering, the transfer of received messages to one
of the two Rx FIFOs, as well as the Rx FIFO put and get indices.

Acceptance filter
The FDCAN offers the possibility to configure two sets of acceptance filters, one for
standard identifiers and another for extended identifiers. These filters can be assigned to Rx
FIFO 0 or Rx FIFO 1. For acceptance filtering, each list of filters is executed from element
#0 until the first matching element. Acceptance filtering stops at the first matching element,
and the following filter elements are not evaluated for this message.

RM0456 Rev 6

<!-- pagebreak -->

