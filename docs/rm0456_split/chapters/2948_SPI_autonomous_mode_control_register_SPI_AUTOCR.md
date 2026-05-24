24 23
EFEC[2:0]
EFT[1:0]

16 15
EFID1[28:0]

Res.

EFID2[28:0]

RM0456 Rev 6

8 7

0

RM0456

FD controller area network (FDCAN)
Table 730. Extended message ID filter element field description
Field

Description

Extended filter element configuration
All enabled filter elements are used for acceptance filtering of extended frames.
Acceptance filtering stops at the first matching enabled filter element or when the end
of the filter list is reached. If EFEC[2:0] = 100, 101 or 110 a match sets interrupt flag
IR[HPM] and, if enabled, an interrupt is generated. In this case register HPMS is
updated with the status of the priority match.
F0 Bits 31:29 – 000: Disable filter element
EFEC[2:0] – 001: Store in Rx FIFO 0 if filter matches
– 010: Store in Rx FIFO 1 if filter matches
– 011: Reject ID if filter matches
– 100: Set priority if filter matches
– 101: Set priority and store in FIFO 0 if filter matches
– 110: Set priority and store in FIFO 1 if filter matches
– 111: Not used
Extended filter ID 1
F0 Bits 28:0 First ID of extended ID filter element.
EFID1[28:0] When filtering for Rx FIFO, this field defines the ID of an extended message to be
stored. The received identifiers must match exactly, only XIDAM masking
mechanism.
Extended filter type
– 00: Range filter from EF1ID to EF2ID (EF2ID ≥ EF1ID)
F1 Bits 31:30
– 01: Dual ID filter for EF1ID or EF2ID
EFT[1:0]
– 10: Classic filter: EF1ID = filter, EF2ID = mask
– 11: Range filter from EF1ID to EF2ID (EF2ID ≥ EF1ID), XIDAM mask not applied
F1 Bit 29

Not used

F1 Bits 28:0 Extended filter ID 2
EFID2[28:0] Second ID of extended ID filter element.

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

RM0456

70.4

FDCAN registers

70.4.1

FDCAN core release register (FDCAN_CREL)
Address offset: 0x0000
Reset value: 0x3214 1218

31

30

29

28

27

26

REL[3:0]

25

24

23

STEP[3:0]

22

21

20

19

SUBSTEP[3:0]

18

17

16

YEAR[3:0]

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

r

21

20

19

18

17

16

MON[7:0]

DAY[7:0]

r

Bits 31:28 REL[3:0]: 3
Bits 27:24 STEP[3:0]: 2
Bits 23:20 SUBSTEP[3:0]: 1
Bits 19:16 YEAR[3:0]: 4
Bits 15:8 MON[7:0]: 12
Bits 7:0 DAY[7:0]: 18

70.4.2

FDCAN endian register (FDCAN_ENDN)
Address offset: 0x0004
Reset value: 0x8765 4321

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

ETV[31:16]

ETV[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 ETV[31:0]: Endianness test value
The endianness test value is 0x8765 4321.

Note:

The register read must give the reset value to ensure no endianness issue.

70.4.3

FDCAN data bit timing and prescaler register (FDCAN_DBTP)
Address offset: 0x000C
Reset value: 0x0000 0A33
This register is only writable if the CCE and INIT bits of the FDCAN_CCCR are set. The
CAN time quantum can be programmed in the range of 1 to 32 FDCAN clock periods:
tq = (DBRP[4:0] + 1) FDCAN clock periods.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)
DTSEG1[4:0] is the sum of PROP_SEG and PHASE_SEG1. DTSEG2[3:0] is
PHASE_SEG2. Therefore, the length of the bit time is
(programmed values) × [DTSEG1[4:0] + DTSEG2[3:0] + 3] × tq or
(functional values) × [SYNC_SEG + PROP_SEG + PHASE_SEG1 + PHASE_SEG2] × tq.
The information processing time (IPT) is 0, meaning the data for the next bit is available at
the first clock edge after the sample point.

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TDC

Res.

Res.

rw
15

14

13

Res.

Res.

Res.

12

11

10

9

8

7

DTSEG1[4:0]
rw

rw

rw

6

5

20

19

rw

rw

4

3

rw

rw

rw

rw

17

16

rw

rw

rw

2

1

0

DBRP[4:0]

DTSEG2[3:0]
rw

18

DSJW[3:0]
rw

rw

rw

rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bit 23 TDC: Transceiver delay compensation
0: Transceiver delay compensation disabled
1: Transceiver delay compensation enabled
Bits 22:21 Reserved, must be kept at reset value.
Bits 20:16 DBRP[4:0]: Data bit rate prescaler
The value by which the oscillator frequency is divided to generate the bit time quanta. The bit
time is built up from a multiple of this quantum. Valid values for the baud rate prescaler are 0
to 31. The hardware interpreters this value as the value programmed plus 1.
Bits 15:13 Reserved, must be kept at reset value.
Bits 12:8 DTSEG1[4:0]: Data time segment before sample point
Valid values are 0 to 31. The value used by the hardware is the one programmed,
incremented by 1, that is tBS1 = (DTSEG1[4:0] + 1) × tq.
Bits 7:4 DTSEG2[3:0]: Data time segment after sample point
Valid values are 0 to 15. The value used by the hardware is the one programmed,
incremented by 1, i.e. tBS2 = (DTSEG2[3:0] + 1) × tq.
Bits 3:0 DSJW[3:0]: Synchronization jump width
Valid values are 0 to 15. The value used by the hardware is the one programmed,
incremented by 1: tSJW = (DSJW[3:0] + 1) × tq.

Note:

With an FDCAN clock of 8 MHz, the reset value 0x0000 0A33 configures the FDCAN for a
fast bit rate of 500 kbit/s.
The data phase bit rate must be higher than or equal to the nominal bit rate.

70.4.4

FDCAN test register (FDCAN_TEST)
Write access to this register is enabled by setting the TEST bit of the FDCAN_CCCR
register. All register functions are set to their reset values when this bit is cleared.
Loop-back mode and software control of Tx pin FDCANx_TX are hardware test modes.
Programming TX[1:0] differently from 00 can disturb the message transfer on the CAN bus.
Address offset: 0x0010
Reset value: 0x0000 0000

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

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

6

5

15

14

13

12

11

10

9

8

7

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

RX
r

TX[1:0]
rw

4

3

2

1

0

LBCK

Res.

Res.

Res.

Res.

rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bit 7 RX: Receive pin
This bit is used to monitor the actual value of FDCANx_RX. It is synchronized with the
FDCANx_RX pin, so it is set after reset if the FDCAN is connected to a network.
0: The CAN bus is dominant (FDCANx_RX = 0)
1: The CAN bus is recessive (FDCANx_RX = 1)
Bits 6:5 TX[1:0]: Control of transmit pin
00: Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the
CAN bit time
01: Sample point can be monitored at pin FDCANx_TX
10: Dominant (0) level at pin FDCANx_TX
11: Recessive (1) at pin FDCANx_TX
Bit 4 LBCK: Loop-back mode
0: Reset value, loop-back mode is disabled
1: Loop-back mode is enabled (see Power-down (Sleep mode))
Bits 3:0 Reserved, must be kept at reset value.

70.4.5

FDCAN RAM watchdog register (FDCAN_RWD)
The RAM watchdog monitors the READY output of the message RAM. A message RAM
access starts the message RAM watchdog counter with the value configured through the
WDC[7:0] bitfield of the FDCAN_RWD register.
The counter is reloaded with WDC[7:0] when the message RAM signals successful
completion by activating its READY output. In case there is no response from the message
RAM until the counter has counted down to 0, the counter stops, and the interrupt flag WDI
is set in the FDCAN_IR register. The RAM watchdog counter is clocked by the fdcan_pclk
clock.
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

r

r

r

r

r

r

r

rw

rw

rw

rw

rw

rw

rw

WDV[7:0]
r

WDC[7:0]

Bits 31:16 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

FD controller area network (FDCAN)

Bits 15:8 WDV[7:0]: Watchdog value
Actual message RAM watchdog counter value.
Bits 7:0 WDC[7:0]: Watchdog configuration
Start value of the message RAM watchdog counter. With the reset value of 00, the counter is
disabled.
This bitfield is write-protected (P): write access is possible only when the CCE and INIT bits
of the FDCAN_CCCR register are both set.

70.4.6

FDCAN CC control register (FDCAN_CCCR)
Address offset: 0x0018
Reset value: 0x0000 0001
For details about setting and clearing single bits, see Software initialization.

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

NISO

TXP

EFBI

PXHD

Res.

Res.

BRSE

FDOE

TEST

DAR

MON

CSR

CSA

ASM

CCE

INIT

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

r

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 NISO: Non-ISO operation
If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN
FD Specification V1.0.
0: CAN FD frame format according to ISO11898-1
1: CAN FD frame format according to Bosch CAN FD Specification V1.0
Bit 14 TXP: Transmit pause enable
If this bit is set, the FDCAN pauses for two CAN bit times before starting the next
transmission after successfully transmitting a frame.
0: Disabled
1: Enabled
Bit 13 EFBI: Edge filtering during bus integration
0: Edge filtering disabled
1: Two consecutive dominant tq required to detect an edge for hard synchronization
Bit 12 PXHD: Protocol exception handling disable
0: Protocol exception handling enabled
1: Protocol exception handling disabled
Bits 11:10 Reserved, must be kept at reset value.
Bit 9 BRSE: FDCAN bit rate switching
0: Bit rate switching for transmissions disabled
1: Bit rate switching for transmissions enabled
Bit 8 FDOE: FD operation enable
0: FD operation disabled
1: FD operation enabled

RM0456 Rev 6

<!-- pagebreak -->

