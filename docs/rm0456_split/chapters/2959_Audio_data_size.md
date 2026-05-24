3086

FD controller area network (FDCAN)

RM0456

Bit 6 EW: Warning status
0: Both error counters are below the error-warning limit of 96.
1: At least one of error counter has reached the error-warning limit of 96.
Bit 5 EP: Error passive
0: The FDCAN is in the error-active state. It normally takes part in bus communication and
sends an active error flag when an error has been detected.
1: The FDCAN is in the error-passive state.
Bits 4:3 ACT[1:0]: Activity
Monitors the module’s CAN communication state.
00: Synchronizing: node is synchronizing on CAN communication.
01: Idle: node is neither receiver nor transmitter.
10: Receiver: node is operating as receiver.
11: Transmitter: node is operating as transmitter.
Bits 2:0 LEC[2:0]: Last error code
LEC[2:0] indicates the type of the last error to occur on the CAN bus. This bitfield is cleared
when a message has been transferred (reception or transmission) without error.
000: No error occurred since LEC[2:0] has been cleared by successful reception or
transmission.
001: Stuff error. More than five equal bits in a sequence have occurred in a part of a received
message where this is not allowed.
010: Form error. A fixed format part of a received frame has the wrong format.
011: Ack error. The message transmitted by the FDCAN was not acknowledged by another
node.
100: Bit1 error. During the transmission of a message (with the exception of the arbitration
field), the device wanted to send a recessive level (bit of logical value 1), but the monitored
bus value was dominant.
101: Bit0 error. During the transmission of a message (or acknowledge bit, or active error
flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical
value 0), but the monitored bus value was recessive. During bus-off recovery this status is set
each time a sequence of 11 recessive bits has been monitored. This enables the CPU to
monitor the proceeding of the bus-off recovery sequence (indicating the bus is not stuck at
dominant or continuously disturbed).
110: CRC error. The CRC check sum of a received message was incorrect. The CRC of an
incoming message does not match with the CRC calculated from the received data.
111: No change. Any read access to the protocol status register reinitializes LEC[2:0] to 7.
When the LEC[2:0] shows the value 7, no CAN bus event was detected since the last CPU
read access to the protocol status register.
Access type is rs: set on read.

Note:

When a frame in FDCAN format has reached the data phase with the BRS flag set, the next
CAN event (error or valid frame) is shown in DLEC[2:0] instead of LEC[2:0]. An error in a
fixed stuff bit of an FDCAN CRC sequence is shown as a form error, not as a stuff error.
The bus-off recovery sequence (see CAN Specification Rev. 2.0 or ISO11898-1) cannot be
shortened by setting or clearing the INIT bit of the FDCAN_CCCR register. If the device
enters bus-off, it sets the INIT bit of its own, stopping all bus activities. Once INIT has been
cleared by the CPU, the device waits for 129 occurrences of bus-idle (129 × 11 consecutive
recessive bits) before resuming normal operation. At the end of the bus-off recovery
sequence, the error management counters are reset. During the waiting time after clearing
INIT, each time a sequence of 11 recessive bits has been monitored, a bit0 error code is
written to LEC[2:0] of FDCAN_PSR, enabling the CPU to check up whether the CAN bus is

<!-- pagebreak -->

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)
stuck at dominant or continuously disturbed, and to monitor the bus-off recovery sequence.
The REC[6:0] bitfield of the FDCAN_ECR register is used to count these sequences.

70.4.14

FDCAN transmitter delay compensation register (FDCAN_TDCR)
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

Res.

TDCO[6:0]
rw

rw

rw

rw

Res.
rw

rw

rw

TDCF[6:0]
rw

rw

rw

rw

Bits 31:15 Reserved, must be kept at reset value.
Bits 14:8 TDCO[6:0]: Transmitter delay compensation offset
Offset value defining the distance between the measured delay from FDCAN_TX to
FDCAN_RX and the secondary sample point. Valid values are 0 to 127 × mtq.
This bitfield is write-protected (P), which means that write access is possible only when the
CCE and INIT bits of the FDCAN_CCCR register are both set.
Bit 7 Reserved, must be kept at reset value.
Bits 6:0 TDCF[6:0]: Transmitter delay compensation filter window length
Defines the minimum value for the SSP position, dominant edges on FDCAN_RX that would
result in an earlier SSP position are ignored for transmitter delay measurements.
This bitfield is write-protected (P), which means that write access is possible only when the
CCE and INIT bits of the FDCAN_CCCR register are both set.

70.4.15

FDCAN interrupt register (FDCAN_IR)
The flags are set when one of the listed conditions is detected (edge-sensitive). The flags
remain set until the host clears them. A flag is cleared by writing 1 to the corresponding bit
position.
Writing 0 has no effect. A hard reset clears the register. The configuration of FDCAN_IE
controls whether an interrupt is generated. The configuration of FDCAN_ILS controls on
which interrupt line an interrupt is signaled.
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

ARA

PED

PEA

WDI

BO

EW

EP

ELO

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

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

TOO

MRAF

TSW

TEFL

TEFF

TEFN

TFE

TCF

TC

HPM

RF1L

RF1F

RF1N

RF0L

RF0F

RF0N

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

Bits 31:24 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

RM0456

Bit 23 ARA: Access to reserved address
0: No access to reserved address occurred
1: Access to reserved address occurred
Bit 22 PED: Protocol error in data phase (data bit time is used)
0: No protocol error in data phase
1: Protocol error in data phase detected (DLEC[2:0] different from 0 and 7 in FDCAN_PSR)
Bit 21 PEA: Protocol error in arbitration phase (nominal bit time is used)
0: No protocol error in arbitration phase
1: Protocol error in arbitration phase detected (LEC[2:0] different from 0 and 7 in
FDCAN_PSR)
Bit 20 WDI: Watchdog interrupt
0: No message RAM watchdog event occurred
1: Message RAM watchdog event due to missing READY
Bit 19 BO: Bus-off status
0: Bus-off status unchanged
1: Bus-off status changed
Bit 18 EW: Warning status
0: Error-warning status unchanged
1: Error-warning status changed
Bit 17 EP: Error passive
0: Error-passive status unchanged
1: Error-passive status changed
Bit 16 ELO: Error logging overflow
0: CAN error logging counter did not overflow.
1: Overflow of CAN error logging counter occurred.
Bit 15 TOO: Timeout occurred
0: No timeout
1: Timeout reached
Bit 14 MRAF: Message RAM access failure
The flag is set when the Rx handler:
- has not completed acceptance filtering or storage of an accepted message until the
arbitration field of the following message has been received. In this case acceptance
filtering or message storage is aborted and the Rx handler starts processing of the
following message.
- was unable to write a message to the message RAM. In this case message storage is
aborted.
In both cases the FIFO put index is not updated. The partly stored message is overwritten
when the next message is stored to this location.
The flag is also set when the Tx handler was not able to read a message from the message
RAM in time. In this case message transmission is aborted. In case of a Tx handler access
failure, the FDCAN is switched into restricted operation mode (see Restricted operation
mode). To leave restricted operation mode, the host CPU has to clear the ASM of the
FDCAN_CCCR register.
0: No message RAM access failure occurred
1: Message RAM access failure occurred

<!-- pagebreak -->

