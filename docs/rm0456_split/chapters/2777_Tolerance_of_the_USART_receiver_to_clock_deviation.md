2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Bit 2 RE: Receiver enable
This bit enables the receiver. It is set and cleared by software.
0: Receiver is disabled
1: Receiver is enabled and begins searching for a start bit
Bit 1 UESM: LPUART enable in low-power mode
When this bit is cleared, the LPUART cannot request its kernel clock and is not functional in
low-power mode.
When this bit is set, the LPUART can wake up the MCU from low-power mode.
This bit is set and cleared by software.
0: LPUART not functional in low-power mode.
1: LPUART functional in low-power mode.
Note: The UESM bit must be set at the initialization phase.
If the LPUART does not support the wake-up from low-power mode, this bit is reserved
and must be kept at reset value. Refer to Section 67.3: LPUART implementation.
Bit 0 UE: LPUART enable
When this bit is cleared, the LPUART prescalers and outputs are stopped immediately, and
current operations are discarded. The configuration of the LPUART is kept, but all the status
flags, in the LPUART_ISR are reset. This bit is set and cleared by software.
0: LPUART prescaler and outputs disabled, low-power mode
1: LPUART enabled
Note: To enter low-power mode without generating errors on the line, the TE bit must be reset
before and the software must wait for the TC bit in the LPUART_ISR to be set before
resetting the UE bit.
The DMA requests are also reset when UE = 0 so the DMA channel must be disabled
before resetting the UE bit.

67.7.3

LPUART control register 2 (LPUART_CR2)
Address offset: 0x04
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

ADD[7:0]

23
Res.

22
Res.

21
Res.

20

19

18

17

16

Res.

MSBFI
RST

DATAIN
V

TXINV

RXINV

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

SWAP

Res.

STOP[1:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ADDM7

Res.

Res.

Res.

Res.

rw

<!-- pagebreak -->

rw

rw

rw

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

Bits 31:24 ADD[7:0]: Address of the LPUART node
These bits give the address of the LPUART node in mute mode or a character code to be
recognized in low-power or Run mode:
–
In mute mode: they are used in multiprocessor communication to wake up from mute
mode with 4-bit/7-bit address mark detection. The MSB of the character sent by the
transmitter must be equal to 1. In 4-bit address mark detection, only ADD[3:0] bits are
used.
–
In low-power mode: they are used for wake up from low-power mode on character match.
When a character, received during low-power mode, corresponds to the character
programmed through ADD[7:0] bitfield, the CMF flag is set and wakes up the device from
low-power mode if the corresponding interrupt is enabled by setting CMIE bit.
–
In Run mode with mute mode inactive (for example, end-of-block detection in ModBus
protocol): the whole received character (8 bits) is compared to ADD[7:0] value and CMF
flag is set on match. An interrupt is generated if the CMIE bit is set.
These bits can only be written when the LPUART is disabled (UE = 0).
Bits 23:20 Reserved, must be kept at reset value.
Bit 19 MSBFIRST: Most significant bit first
This bit is set and cleared by software.
0: data is transmitted/received with data bit 0 first, following the start bit.
1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit.
This bitfield can only be written when the LPUART is disabled (UE = 0).
Bit 18 DATAINV: Binary data inversion
This bit is set and cleared by software.
0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L)
1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The
parity bit is also inverted.
This bitfield can only be written when the LPUART is disabled (UE = 0).
Bit 17 TXINV: TX pin active level inversion
This bit is set and cleared by software.
0: TX pin signal works using the standard logic levels (VDD = 1 / idle, Gnd = 0 / mark)
1: TX pin signal values are inverted. ((VDD = 0 / mark, Gnd = 1 / idle).
This enables the use of an external inverter on the TX line.
This bitfield can only be written when the LPUART is disabled (UE = 0).
Bit 16 RXINV: RX pin active level inversion
This bit is set and cleared by software.
0: RX pin signal works using the standard logic levels (VDD = 1 / idle, Gnd = 0/mark)
1: RX pin signal values are inverted. ((VDD = 0/mark, Gnd = 1 / idle).
This enables the use of an external inverter on the RX line.
This bitfield can only be written when the LPUART is disabled (UE = 0).
Bit 15 SWAP: Swap TX/RX pins
This bit is set and cleared by software.
0: TX/RX pins are used as defined in standard pinout
1: The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired
connection to another UART.
This bitfield can only be written when the LPUART is disabled (UE = 0).
Bit 14 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Bits 13:12 STOP[1:0]: Stop bits
These bits are used for programming the stop bits.
00: 1 stop bit
01: Reserved.
10: 2 stop bits
11: Reserved
This bitfield can only be written when the LPUART is disabled (UE = 0).
Bits 11:5 Reserved, must be kept at reset value.
Bit 4 ADDM7:7-bit Address Detection/4-bit Address Detection
This bit is for selection between 4-bit address detection or 7-bit address detection.
0: 4-bit address detection
1: 7-bit address detection (in 8-bit data mode)
This bit can only be written when the LPUART is disabled (UE = 0)
Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address
(ADD[5:0] and ADD[7:0]) respectively.
Bits 3:0 Reserved, must be kept at reset value.

67.7.4

LPUART control register 3 (LPUART_CR3)
Address offset: 0x08
Reset value: 0x0000 0000
FIFO mode enabled, FIFOEN = 1

31

30

29

28

27

RXFTI
E

TXFTCFG[2:0]

26

25

RXFTCFG[2:0]

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

TXFTIE

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

CTSIE

CTSE

RTSE

DMAT

DMAR

Res.

Res.

HDSEL

Res.

Res.

EIE

rw

rw

rw

rw

rw

DEP

DEM

DDRE

OVRDI
S

rw

rw

rw

rw

rw

rw

Bits 31:29 TXFTCFG[2:0]: TXFIFO threshold configuration
000:TXFIFO reaches 1/8 of its depth.
001:TXFIFO reaches 1/4 of its depth.
110:TXFIFO reaches 1/2 of its depth.
011:TXFIFO reaches 3/4 of its depth.
100:TXFIFO reaches 7/8 of its depth.
101:TXFIFO becomes empty.
Others: Reserved, must not be used.
This bit can only be written when the LPUART is disabled (UE = 0).
Bit 28 RXFTIE: RXFIFO threshold interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: An LPUART interrupt is generated when Receive FIFO reaches the threshold
programmed in RXFTCFG[2:0].

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

Bits 27:25 RXFTCFG[2:0]: Receive FIFO threshold configuration
000:Receive FIFO reaches 1/8 of its depth.
001:Receive FIFO reaches 1/4 of its depth.
110:Receive FIFO reaches 1/2 of its depth.
011:Receive FIFO reaches 3/4 of its depth.
100:Receive FIFO reaches 7/8 of its depth.
101:Receive FIFO becomes full.
Others: Reserved, must not be used.
This bit can only be written when the LPUART is disabled (UE = 0).
Bit 24 Reserved, must be kept at reset value.
Bit 23 TXFTIE: TXFIFO threshold interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: A LPUART interrupt is generated when TXFIFO reaches the threshold programmed in
TXFTCFG[2:0].
Bits 22:16 Reserved, must be kept at reset value.
Bit 15 DEP: Driver enable polarity selection
0: DE signal is active high.
1: DE signal is active low.
This bit can only be written when the LPUART is disabled (UE = 0).
Bit 14 DEM: Driver enable mode
This bit enables the user to activate the external transceiver control, through the DE signal.
0: DE function is disabled.
1: DE function is enabled. The DE signal is output on the RTS pin.
This bit can only be written when the LPUART is disabled (UE = 0).
Bit 13 DDRE: DMA Disable on reception Error
0: DMA is not disabled in case of reception error. The corresponding error flag is set but
RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not
asserted, so the erroneous data is not transferred (no DMA request), but next correct
received data is transferred.
1: DMA is disabled following a reception error. The corresponding error flag is set, as well as
RXNE. The DMA request is masked until the error flag is cleared. This means that the
software must first disable the DMA request (DMAR = 0) or clear RXNE before clearing the
error flag.
This bit can only be written when the LPUART is disabled (UE = 0).
Note: The reception errors are: parity error, framing error or noise error.
Bit 12 OVRDIS: Overrun Disable
This bit is used to disable the receive overrun detection.
0: Overrun Error Flag, ORE is set when received data is not read before receiving new data.
1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set
the ORE flag is not set and the new received data overwrites the previous content of the
LPUART_RDR register.
This bit can only be written when the LPUART is disabled (UE = 0).
Note: This control bit enables checking the communication flow w/o reading the data.
Bit 11 Reserved, must be kept at reset value.
Bit 10 CTSIE: CTS interrupt enable
0: Interrupt is inhibited
1: An interrupt is generated whenever CTSIF = 1 in the LPUART_ISR register

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Bit 9 CTSE: CTS enable
0: CTS hardware flow control disabled
1: CTS mode enabled, data is only transmitted when the CTS input is deasserted (tied to 0).
If the CTS input is asserted while data is being transmitted, then the transmission completes
before stopping. If data is written into the data register while CTS is asserted, the
transmission is postponed until CTS is deasserted.
This bit can only be written when the LPUART is disabled (UE = 0)
Bit 8 RTSE: RTS enable
0: RTS hardware flow control disabled
1: RTS output enabled, data is only requested when there is space in the receive buffer. The
transmission of data is expected to cease after the current character has been transmitted.
The RTS output is deasserted (pulled to 0) when data can be received.
This bit can only be written when the LPUART is disabled (UE = 0).
Bit 7 DMAT: DMA enable transmitter
This bit is set/reset by software
1: DMA mode is enabled for transmission
0: DMA mode is disabled for transmission
Bit 6 DMAR: DMA enable receiver
This bit is set/reset by software
1: DMA mode is enabled for reception
0: DMA mode is disabled for reception
Bits 5:4 Reserved, must be kept at reset value.
Bit 3 HDSEL: Half-duplex selection
Selection of single-wire half-duplex mode
0: Half-duplex mode is not selected
1: Half-duplex mode is selected
This bit can only be written when the LPUART is disabled (UE = 0).
Bits 2:1 Reserved, must be kept at reset value.
Bit 0 EIE: Error interrupt enable
Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing
error, overrun error or noise flag (FE = 1 or ORE = 1 or NE = 1 in the LPUART_ISR register).
0: Interrupt is inhibited
1: An interrupt is generated when FE = 1 or ORE = 1 or NE = 1 in the LPUART_ISR register.

67.7.5

LPUART control register 3 [alternate] (LPUART_CR3)
Address offset: 0x08
Reset value: 0x0000 0000
FIFO mode disabled, FIFOEN = 0

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

CTSIE

CTSE

RTSE

DMAT

DMAR

Res.

Res.

HDSEL

Res.

Res.

EIE

rw

rw

rw

rw

rw

DEP

DEM

DDRE

OVRDI
S

rw

rw

rw

rw

<!-- pagebreak -->

RM0456 Rev 6

rw

rw

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

Bits 31:23 Reserved, must be kept at reset value.
Bits 22:16 Reserved, must be kept at reset value.
Bit 15 DEP: Driver enable polarity selection
0: DE signal is active high.
1: DE signal is active low.
This bit can only be written when the LPUART is disabled (UE = 0).
Bit 14 DEM: Driver enable mode
This bit enables the user to activate the external transceiver control, through the DE signal.
0: DE function is disabled.
1: DE function is enabled. The DE signal is output on the RTS pin.
This bit can only be written when the LPUART is disabled (UE = 0).
Bit 13 DDRE: DMA Disable on reception Error
0: DMA is not disabled in case of reception error. The corresponding error flag is set but
RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not
asserted, so the erroneous data is not transferred (no DMA request), but next correct
received data is transferred.
1: DMA is disabled following a reception error. The corresponding error flag is set, as well as
RXNE. The DMA request is masked until the error flag is cleared. This means that the
software must first disable the DMA request (DMAR = 0) or clear RXNE before clearing the
error flag.
This bit can only be written when the LPUART is disabled (UE = 0).
Note: The reception errors are: parity error, framing error or noise error.
Bit 12 OVRDIS: Overrun Disable
This bit is used to disable the receive overrun detection.
0: Overrun Error Flag, ORE is set when received data is not read before receiving new data.
1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set
the ORE flag is not set and the new received data overwrites the previous content of the
LPUART_RDR register.
This bit can only be written when the LPUART is disabled (UE = 0).
Note: This control bit enables checking the communication flow w/o reading the data.
Bit 11 Reserved, must be kept at reset value.
Bit 10 CTSIE: CTS interrupt enable
0: Interrupt is inhibited
1: An interrupt is generated whenever CTSIF = 1 in the LPUART_ISR register
Bit 9 CTSE: CTS enable
0: CTS hardware flow control disabled
1: CTS mode enabled, data is only transmitted when the CTS input is deasserted (tied to 0).
If the CTS input is asserted while data is being transmitted, then the transmission completes
before stopping. If data is written into the data register while CTS is asserted, the
transmission is postponed until CTS is deasserted.
This bit can only be written when the LPUART is disabled (UE = 0)
Bit 8 RTSE: RTS enable
0: RTS hardware flow control disabled
1: RTS output enabled, data is only requested when there is space in the receive buffer. The
transmission of data is expected to cease after the current character has been transmitted.
The RTS output is deasserted (pulled to 0) when data can be received.
This bit can only be written when the LPUART is disabled (UE = 0).

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Bit 7 DMAT: DMA enable transmitter
This bit is set/reset by software
1: DMA mode is enabled for transmission
0: DMA mode is disabled for transmission
Bit 6 DMAR: DMA enable receiver
This bit is set/reset by software
1: DMA mode is enabled for reception
0: DMA mode is disabled for reception
Bits 5:4 Reserved, must be kept at reset value.
Bit 3 HDSEL: Half-duplex selection
Selection of single-wire half-duplex mode
0: Half-duplex mode is not selected
1: Half-duplex mode is selected
This bit can only be written when the LPUART is disabled (UE = 0).
Bits 2:1 Reserved, must be kept at reset value.
Bit 0 EIE: Error interrupt enable
Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing
error, overrun error or noise flag (FE = 1 or ORE = 1 or NE = 1 in the LPUART_ISR register).
0: Interrupt is inhibited
1: An interrupt is generated when FE = 1 or ORE = 1 or NE = 1 in the LPUART_ISR register.

67.7.6

LPUART baud rate register (LPUART_BRR)
This register can only be written when the LPUART is disabled (UE = 0). It may be
automatically updated by hardware in auto baud rate detection mode.
Address offset: 0x0C
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

Res.

Res.

Res.

Res.

Res.

19

18

17

16

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

rw

rw

rw

rw

rw

rw

rw

rw

BRR[19:16]

BRR[15:0]
rw

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:0 BRR[19:0]: LPUART baud rate division (LPUARTDIV)

Note:

It is forbidden to write values lower than 0x300 in the LPUART_BRR register.
Provided that LPUART_BRR must be ≥ 0x300 and LPUART_BRR is 20 bits, a care must be
taken when generating high baud rates using high lpuart_ker_ck_pres values.
lpuart_ker_ck_pres must be in the range [3 x baud rate..4096 x baud rate].

67.7.7

LPUART request register (LPUART_RQR)
Address offset: 0x18
Reset value: 0x0000 0000

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

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

0

TXFRQ RXFRQ MMRQ SBKRQ
w

w

w

Res.

w

Bits 31:5 Reserved, must be kept at reset value.
Bit 4 TXFRQ: Transmit data flush request
This bit is used when FIFO mode is enabled. TXFRQ bit is set to flush the whole FIFO. This
sets the flag TXFE (TXFIFO empty, bit 23 in the LPUART_ISR register).
Note: In FIFO mode, the TXFNF flag is reset during the flush request until TxFIFO is empty in
order to ensure that no data are written in the data register.
Bit 3 RXFRQ: Receive data flush request
Writing 1 to this bit clears the RXNE flag.
This enables discarding the received data without reading it, and avoid an overrun condition.
Bit 2 MMRQ: Mute mode request
Writing 1 to this bit puts the LPUART in Mute mode and resets the RWU flag.
Bit 1 SBKRQ: Send break request
Writing 1 to this bit sets the SBKF flag and request to send a BREAK on the line, as soon as
the transmit machine is available.
Note: If the application needs to send the break character following all previously inserted
data, including the ones not yet transmitted, the software must wait for the TXE flag
assertion before setting the SBKRQ bit.
Bit 0 Reserved, must be kept at reset value.

67.7.8

LPUART interrupt and status register (LPUART_ISR)
Address offset: 0x1C
Reset value: 0x0080 00C0
The same register can be used in FIFO mode enabled (this section) and FIFO mode
disabled (next section).
FIFO mode enabled, FIFOEN = 1

31

30

29

28

27

26

25

24

23

Res.

Res.

Res.

Res.

TXFT

RXFT

Res.

RXFF

TXFE

r

r

r

r

22

21

REACK TEACK
r

20

19

18

17

16

Res.

RWU

SBKF

CMF

BUSY

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

CTS

CTSIF

Res.

TXFNF

TC

RXFNE

IDLE

ORE

NE

FE

PE

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

Bits 31:28 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Bit 27 TXFT: TXFIFO threshold flag
This bit is set by hardware when the number of empty locations in the TXFIFO is greater
than the threshold programmed in the TXFTCFG[2:0] bitfield of LPUART_CR3 register.An
interrupt is generated if the TXFTIE bit = 1 (bit 31) in the LPUART_CR3 register.
0: TXFIFO does not reach the programmed threshold.
1: TXFIFO reached the programmed threshold.
Bit 26 RXFT: RXFIFO threshold flag
This bit is set by hardware when the RXFIFO reaches the threshold programmed in the
RXFTCFG[2:0] bitfield of the LPUART_CR3 register, that is, the Receive FIFO contains
RXFTCFG[2:0] data. An interrupt is generated if the RXFTIE bit = 1 (bit 27) in the
LPUART_CR3 register.
0: Receive FIFO does not reach the programmed threshold.
1: Receive FIFO reached the programmed threshold.
Note: When the RXFTCFG[2:0] threshold is configured to 101, the RXFT flag is set if RXFIFO
size data are available, that is, (RXFIFO size – 1) data in the RXFIFO and 1 data in the
LPUART_RDR. Consequently, the (RXFIFO size + 1) th received data does not cause
an overrun error. The overrun error occurs after receiving the (RXFIFO size + 2) th
data.
Bit 25 Reserved, must be kept at reset value.
Bit 24 RXFF: RXFIFO Full
This bit is set by hardware when the number of received data corresponds to
RXFIFO size + 1 (RXFIFO full + 1 data in the LPUART_RDR register.
An interrupt is generated if the RXFFIE bit = 1 in the LPUART_CR1 register.
0: RXFIFO is not Full.
1: RXFIFO is Full.
Bit 23 TXFE: TXFIFO Empty
This bit is set by hardware when TXFIFO is Empty. When the TXFIFO contains at least one
data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4)
in the LPUART_RQR register.
An interrupt is generated if the TXFEIE bit = 1 (bit 30) in the LPUART_CR1 register.
0: TXFIFO is not empty.
1: TXFIFO is empty.
Bit 22 REACK: Receive enable acknowledge flag
This bit is set/reset by hardware, when the Receive Enable value is taken into account by the
LPUART.
It can be used to verify that the LPUART is ready for reception before entering low-power
mode.
Note: If the LPUART does not support the wake-up from Stop feature, this bit is reserved and
kept at reset value.
Bit 21 TEACK: Transmit enable acknowledge flag
This bit is set/reset by hardware, when the Transmit Enable value is taken into account by
the LPUART.
It can be used when an idle frame request is generated by writing TE = 0, followed by TE = 1
in the LPUART_CR1 register, in order to respect the TE = 0 minimum period.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

Bit 20 Reserved, must be kept at reset value.
Bit 19 RWU: Receiver wake-up from mute mode
This bit indicates if the LPUART is in mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The mute mode control sequence (address or IDLE) is
selected by the WAKE bit in the LPUART_CR1 register.
When wake-up on IDLE mode is selected, this bit can only be set by software, writing 1 to
the MMRQ bit in the LPUART_RQR register.
0: Receiver in active mode
1: Receiver in mute mode
Note: If the LPUART does not support the wake-up from Stop feature, this bit is reserved and
kept at reset value.
Bit 18 SBKF: Send break flag
This bit indicates that a send break character was requested. It is set by software, by writing
1 to the SBKRQ bit in the LPUART_CR3 register. It is automatically reset by hardware
during the stop bit of break transmission.
0: No break character transmitted
1: Break character transmitted
Bit 17 CMF: Character match flag
This bit is set by hardware, when a the character defined by ADD[7:0] is received. It is
cleared by software, writing 1 to the CMCF in the LPUART_ICR register.
An interrupt is generated if CMIE = 1in the LPUART_CR1 register.
0: No Character match detected
1: Character match detected
Bit 16 BUSY: Busy flag
This bit is set and reset by hardware. It is active when a communication is ongoing on the
RX line (successful start bit detected). It is reset at the end of the reception (successful or
not).
0: LPUART is idle (no reception)
1: reception ongoing
Bits 15:11 Reserved, must be kept at reset value.
Bit 10 CTS: CTS flag
This bit is set/reset by hardware. It is an inverted copy of the status of the CTS input pin.
0: CTS line set
1: CTS line reset
Note: If the hardware flow control feature is not supported, this bit is reserved and kept at
reset value.
Bit 9 CTSIF: CTS interrupt flag
This bit is set by hardware when the CTS input toggles, if the CTSE bit is set. It is cleared by
software, by writing 1 to the CTSCF bit in the LPUART_ICR register.
An interrupt is generated if CTSIE = 1 in the LPUART_CR3 register.
0: No change occurred on the CTS status line
1: A change occurred on the CTS status line
Note: If the hardware flow control feature is not supported, this bit is reserved and kept at
reset value.
Bit 8 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Bit 7 TXFNF: TXFIFO not full
TXFNF is set by hardware when TXFIFO is not full, and so data can be written in the
LPUART_TDR. Every write in the LPUART_TDR places the data in the TXFIFO. This flag
remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating
that data can not be written into the LPUART_TDR.
The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the
flush request (by setting TXFRQ bit), the flag TXFNF must be checked prior to writing in
TXFIFO (TXFNF and TXFE are set at the same time).
An interrupt is generated if the TXFNFIE bit = 1 in the LPUART_CR1 register.
0: Data register is full/Transmit FIFO is full.
1: Data register/Transmit FIFO is not full.
Note: This bit is used during single buffer transmission.
Bit 6 TC: Transmission complete
This bit indicates that the last data written in the LPUART_TDR has been transmitted out of
the shift register.
The TC flag behaves as follows:
–
When TDN = 0, the TC flag is set when the transmission of a frame containing data
has completed and when TXE/TXFE is set.
–
When TDN is equal to the number of data in the TXFIFO, the TC flag is set when
TXFIFO is empty and TDN is reached.
–
When TDN is greater than the number of data in the TXFIFO, TC remains cleared
until the TXFIFO is filled again to reach the programmed number of data to be
transferred.
–
When TDN is less than the number of data in the TXFIFO, TC is set when TDN is
reached even if the TXFIFO is not empty.
An interrupt is generated if TCIE = 1 in the LPUART_CR1 register.
The TC bit is cleared by software, by writing 1 to the TCCF of the LPUART_ICR register, or
by writing to the LPUART_TDR register.
Bit 5 RXFNE: RXFIFO not empty
RXFNE bit is set by hardware when the RXFIFO is not empty, and so data can be read from
the LPUART_RDR register. Every read of the LPUART_RDR frees a location in the RXFIFO.
It is cleared when the RXFIFO is empty.
The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the LPUART_RQR
register.
An interrupt is generated if RXFNEIE = 1 in the LPUART_CR1 register.
0: Data is not received
1: Received data is ready to be read.
Bit 4 IDLE: Idle line detected
This bit is set by hardware when an Idle line is detected. An interrupt is generated if
IDLEIE = 1 in the LPUART_CR1 register. It is cleared by software, writing 1 to the IDLECF in
the LPUART_ICR register.
0: No Idle line is detected
1: Idle line is detected
Note: The IDLE bit is not set again until the RXFNE bit has been set (that is, a new idle line
occurs).
If mute mode is enabled (MME = 1), IDLE is set if the LPUART is not mute (RWU = 0),
whatever the mute mode selected by the WAKE bit. If RWU = 1, IDLE is not set.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

Bit 3 ORE: Overrun error
This bit is set by hardware when the data currently being received in the shift register is
ready to be transferred into the LPUART_RDR register while RXFF = 1. It is cleared by a
software, writing 1 to the ORECF, in the LPUART_ICR register.
An interrupt is generated if RXFNEIE = 1 in the LPUART_CR1 register, or EIE = 1 in the
LPUART_CR3 register.
1: Overrun error is detected
Note: When this bit is set, the LPUART_RDR register content is not lost but the shift register
is overwritten. An interrupt is generated if the ORE flag is set during multi buffer
communication if the EIE bit is set.
This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in
the LPUART_CR3 register.
Bit 2 NE: Start bit noise detection flag
This bit is set by hardware when noise is detected on the start bit of a received frame. It is
cleared by software, writing 1 to the NFCF bit in the LPUART_ICR register.
0: No noise is detected
1: Noise is detected
Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit
which itself generates an interrupt. An interrupt is generated when the NE flag is set
during multi buffer communication if the EIE bit is set.
This error is associated with the character in the LPUART_RDR.
Bit 1 FE: Framing error
This bit is set by hardware when a de-synchronization, excessive noise or a break character
is detected. It is cleared by software, writing 1 to the FECF bit in the LPUART_ICR register.
When transmitting data in smartcard mode, this bit is set when the maximum number of
transmit attempts is reached without success (the card NACKs the data frame).
An interrupt is generated if EIE = 1 in the LPUART_CR3 register.
0: No Framing error is detected
1: Framing error or break character is detected
Note: This error is associated with the character in the LPUART_RDR.
Bit 0 PE: Parity error
This bit is set by hardware when a parity error occurs in reception mode. It is cleared by
software, writing 1 to the PECF in the LPUART_ICR register.
An interrupt is generated if PEIE = 1 in the LPUART_CR1 register.
0: No parity error
1: Parity error
Note: This error is associated with the character in the LPUART_RDR.

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

67.7.9

RM0456

LPUART interrupt and status register [alternate] (LPUART_ISR)
Address offset: 0x1C
Reset value: 0x0000 00C0
The same register can be used in FIFO mode enabled (previous section) and FIFO mode
disabled (this section).
FIFO mode disabled, FIFOEN = 0

31

30

29

28

27

26

25

24

23

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

22

21

20

19

18

17

16

Res.

RWU

SBKF

CMF

BUSY

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

r

r

r

r

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

CTS

CTSIF

Res.

TXE

r

r

r

TC

RXNE

IDLE

ORE

NE

FE

PE

r

r

r

r

r

r

r

REACK TEACK

Bits 31:23 Reserved, must be kept at reset value.
Bit 22 REACK: Receive enable acknowledge flag
This bit is set/reset by hardware, when the Receive Enable value is taken into account by the
LPUART.
It can be used to verify that the LPUART is ready for reception before entering low-power
mode.
Note: If the LPUART does not support the wake-up from Stop feature, this bit is reserved and
kept at reset value.
Bit 21 TEACK: Transmit enable acknowledge flag
This bit is set/reset by hardware, when the Transmit Enable value is taken into account by
the LPUART.
It can be used when an idle frame request is generated by writing TE = 0, followed by TE = 1
in the LPUART_CR1 register, in order to respect the TE = 0 minimum period.
Bit 20 Reserved, must be kept at reset value.
Bit 19 RWU: Receiver wake-up from mute mode
This bit indicates if the LPUART is in mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The mute mode control sequence (address or IDLE) is
selected by the WAKE bit in the LPUART_CR1 register.
When wake-up on IDLE mode is selected, this bit can only be set by software, writing 1 to
the MMRQ bit in the LPUART_RQR register.
0: Receiver in active mode
1: Receiver in mute mode
Note: If the LPUART does not support the wake-up from Stop feature, this bit is reserved and
kept at reset value.
Bit 18 SBKF: Send break flag
This bit indicates that a send break character was requested. It is set by software, by writing
1 to the SBKRQ bit in the LPUART_CR3 register. It is automatically reset by hardware
during the stop bit of break transmission.
0: No break character transmitted
1: Break character transmitted

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

Bit 17 CMF: Character match flag
This bit is set by hardware, when a the character defined by ADD[7:0] is received. It is
cleared by software, writing 1 to the CMCF in the LPUART_ICR register.
An interrupt is generated if CMIE = 1in the LPUART_CR1 register.
0: No Character match detected
1: Character match detected
Bit 16 BUSY: Busy flag
This bit is set and reset by hardware. It is active when a communication is ongoing on the
RX line (successful start bit detected). It is reset at the end of the reception (successful or
not).
0: LPUART is idle (no reception)
1: Reception ongoing
Bits 15:11 Reserved, must be kept at reset value.
Bit 10 CTS: CTS flag
This bit is set/reset by hardware. It is an inverted copy of the status of the CTS input pin.
0: CTS line set
1: CTS line reset
Note: If the hardware flow control feature is not supported, this bit is reserved and kept at
reset value.
Bit 9 CTSIF: CTS interrupt flag
This bit is set by hardware when the CTS input toggles, if the CTSE bit is set. It is cleared by
software, by writing 1 to the CTSCF bit in the LPUART_ICR register.
An interrupt is generated if CTSIE = 1 in the LPUART_CR3 register.
0: No change occurred on the CTS status line
1: A change occurred on the CTS status line
Note: If the hardware flow control feature is not supported, this bit is reserved and kept at
reset value.
Bit 8 Reserved, must be kept at reset value.
Bit 7 TXE: Transmit data register empty
TXE is set by hardware when the content of the LPUART_TDR register has been transferred
into the shift register. It is cleared by a write to the LPUART_TDR register.
An interrupt is generated if the TXEIE bit = 1 in the LPUART_CR1 register.
0: Data register full
1: Data register empty
Note: This bit is used during single buffer transmission.
Bit 6 TC: Transmission complete
This bit indicates that the last data written in the LPUART_TDR has been transmitted out of
the shift register. The TC flag is set when the transmission of a frame containing data has
completed and when TXE is set.
An interrupt is generated if TCIE = 1 in the LPUART_CR1 register.
TC bit is cleared by software by writing 1 to the TCCF in the LPUART_ICR register or by
writing to the LPUART_TDR register.

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Bit 5 RXNE: Read data register not empty
RXNE bit is set by hardware when the content of the LPUART_RDR shift register has been
transferred to the LPUART_RDR register. It is cleared by a read to the LPUART_RDR
register. The
RXNE flag can also be cleared by writing 1 to the RXFRQ in the LPUART_RQR register.
The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the LPUART_RQR
register.
An interrupt is generated if RXNEIE = 1 in the LPUART_CR1 register.
0: Data is not received
1: Received data is ready to be read.
Bit 4 IDLE: Idle line detected
This bit is set by hardware when an Idle Line is detected. An interrupt is generated if
IDLEIE = 1 in the LPUART_CR1 register. It is cleared by software, writing 1 to the IDLECF in
the LPUART_ICR register.
0: No Idle line is detected
1: Idle line is detected
Note: The IDLE bit is not set again until the RXNE bit has been set (that is, a new idle line
occurs).
If mute mode is enabled (MME = 1), IDLE is set if the LPUART is not mute (RWU = 0),
whatever the mute mode selected by the WAKE bit. If RWU = 1, IDLE is not set.
Bit 3 ORE: Overrun error
This bit is set by hardware when the data currently being received in the shift register is
ready to be transferred into the LPUART_RDR register while RXNE = 1 (RXFF = 1 in case
FIFO mode is enabled). It is cleared by a software, writing 1 to the ORECF, in the
LPUART_ICR register.
An interrupt is generated if RXNEIE = 1 in the LPUART_CR1 register, or EIE = 1 in the
LPUART_CR3 register.
1: Overrun error is detected
Note: When this bit is set, the LPUART_RDR register content is not lost but the shift register
is overwritten. An interrupt is generated if the ORE flag is set during multi buffer
communication if the EIE bit is set.
This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in
the LPUART_CR3 register.
Bit 2 NE: Start bit noise detection flag
This bit is set by hardware when noise is detected on the start bit of a received frame. It is
cleared by software, writing 1 to the NFCF bit in the LPUART_ICR register.
0: No noise is detected
1: Noise is detected
Note: This bit does not generate an interrupt as it appears at the same time as the
RXNE/RXFNE bit which itself generates an interrupt. An interrupt is generated when
the NE flag is set during multi buffer communication if the EIE bit is set.
In FIFO mode, this error is associated with the character in the LPUART_RDR.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

Bit 1 FE: Framing error
This bit is set by hardware when a de-synchronization, excessive noise or a break character
is detected. It is cleared by software, writing 1 to the FECF bit in the LPUART_ICR register.
When transmitting data in smartcard mode, this bit is set when the maximum number of
transmit attempts is reached without success (the card NACKs the data frame).
An interrupt is generated if EIE = 1 in the LPUART_CR3 register.
0: No Framing error is detected
1: Framing error or break character is detected
Note: In FIFO mode, this error is associated with the character in the LPUART_RDR.
Bit 0 PE: Parity error
This bit is set by hardware when a parity error occurs in reception mode. It is cleared by
software, writing 1 to the PECF in the LPUART_ICR register.
An interrupt is generated if PEIE = 1 in the LPUART_CR1 register.
0: No parity error
1: Parity error
Note: In FIFO mode, this error is associated with the character in the LPUART_RDR.

67.7.10

LPUART interrupt flag clear register (LPUART_ICR)
Address offset: 0x20
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

CMCF

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

CTSCF

Res.

Res.

TCCF

Res.

NECF

FECF

PECF

w

w

w

w

w

w

IDLECF ORECF
w

w

Bits 31:18 Reserved, must be kept at reset value.
Bit 17 CMCF: Character match clear flag
Writing 1 to this bit clears the CMF flag in the LPUART_ISR register.
Bits 16:10 Reserved, must be kept at reset value.
Bit 9 CTSCF: CTS clear flag
Writing 1 to this bit clears the CTSIF flag in the LPUART_ISR register.
Bit 8 Reserved, must be kept at reset value.
Bit 7 Reserved, must be kept at reset value.
Bit 6 TCCF: Transmission complete clear flag
Writing 1 to this bit clears the TC flag in the LPUART_ISR register.
Bit 5 Reserved, must be kept at reset value.
Bit 4 IDLECF: Idle line detected clear flag
Writing 1 to this bit clears the IDLE flag in the LPUART_ISR register.
Bit 3 ORECF: Overrun error clear flag
Writing 1 to this bit clears the ORE flag in the LPUART_ISR register.

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Bit 2 NECF: Noise detected clear flag
Writing 1 to this bit clears the NE flag in the LPUART_ISR register.
Bit 1 FECF: Framing error clear flag
Writing 1 to this bit clears the FE flag in the LPUART_ISR register.
Bit 0 PECF: Parity error clear flag
Writing 1 to this bit clears the PE flag in the LPUART_ISR register.

67.7.11

LPUART receive data register (LPUART_RDR)
Address offset: 0x24
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
r

r

r

r

r

r

r

r

RDR[8:0]
r

Bits 31:9 Reserved, must be kept at reset value.
Bits 8:0 RDR[8:0]: Receive data value
Contains the received data character.
The RDR register provides the parallel interface between the input shift register and the
internal bus (see Section 67.4.1: LPUART block diagram).
When receiving with the parity enabled, the value read in the MSB bit is the received parity
bit.

67.7.12

LPUART transmit data register (LPUART_TDR)
Address offset: 0x28
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
rw

rw

rw

rw

rw

rw

rw

rw

TDR[8:0]

Bits 31:9 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

Bits 8:0 TDR[8:0]: Transmit data value
Contains the data character to be transmitted.
The TDR register provides the parallel interface between the internal bus and the output shift
register (see Section 67.4.1: LPUART block diagram).
When transmitting with the parity enabled (PCE bit set to 1 in the LPUART_CR1 register),
the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect
because it is replaced by the parity.
Note: This register must be written only when TXE/TXFNF = 1.

67.7.13

LPUART prescaler register (LPUART_PRESC)
This register can only be written when the LPUART is disabled (UE = 0).
Address offset: 0x2C
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

PRESCALER[3:0]
rw

rw

rw

rw

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 PRESCALER[3:0]: Clock prescaler
The LPUART input clock can be divided by a prescaler:
0000: input clock not divided
0001: input clock divided by 2
0010: input clock divided by 4
0011: input clock divided by 6
0100: input clock divided by 8
0101: input clock divided by 10
0110: input clock divided by 12
0111: input clock divided by 16
1000: input clock divided by 32
1001: input clock divided by 64
1010: input clock divided by 128
1011: input clock divided by 256
Others: Reserved, must not be used.
Note: When PRESCALER is programmed with a value different of the allowed ones,
programmed prescaler value is equal to 1011, that is, input clock divided by 256.
If the prescaler is not supported, this bitfield is reserved and must be kept at reset
value. Refer to Section 67.3: LPUART implementation.

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

67.7.14

RM0456

LPUART autonomous mode control register (LPUART_AUTOCR)
Address offset: 0x30
Reset value: 0x8000 0000

31

30

29

28

27

26

25

24

23

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

rw

rw

rw

rw

rw

rw

rw

rw

22

21

20

19

18

TRIGSEL[3:0]

17

16

IDLEDIS TRIGEN TRIGPOL

rw

rw

rw

rw

rw

rw

rw

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

TDN[15:0]
rw

Bits 31:23 Reserved, must be kept at reset value.
Bits 22:19 TRIGSEL[3:0]: Trigger selection bits
Refer to Description LPUART interconnections.
This bitfield can be written only when the UE bit is cleared in LPUART_CR1 register.
0000: lpuart_trg0 selected
0001: lpuart_trg1 selected
...
1111: lpuart_trg15 selected
Note: This bitfield can be written only when the UE bit of LPUART_CR1 register is cleared.
Bit 18 IDLEDIS: Idle frame transmission disable bit after enabling the transmitter
0: Idle frame sent after enabling the transmitter (TE = 1 in LPUART_CR1)
1: Idle frame not sent after enabling the transmitter
Note: This bitfield can be written only when the UE bit of LPUART_CR1 register is cleared.
Bit 17 TRIGEN: Trigger enable bit
0: Trigger disabled
1: Trigger enabled
Note: This bitfield can be written only when the UE bit of LPUART_CR1 register is cleared.
When a trigger is detected, TE is set to 1 in LPUART_CR1 and the data transfer is
launched.
Bit 16 TRIGPOL: Trigger polarity bit
This bitfield can be written only when the UE bit is cleared in LPUART_CR1 register.
0: Trigger active on rising edge
1: Trigger active on falling edge
Bits 15:0 TDN[15:0]: TDC transmission data number
This bitfield enables the programming of the number of data to be transmitted. It can be
written only when UE is cleared in LPUART_CR1.

67.7.15

LPUART register map

<!-- pagebreak -->

RM0456 Rev 6

7

6

5

4

3

2

1

0

TCIE

RXFNEIE

IDLEIE

TE

RE

UESM

UE

0

8

0

TXFNFIE

0

9

11

10
PCE

0

PS

12

WAKE

0

PEIE

13

M0

0

14

0

MME

0

15

0

CMIE

0

16

0

Res.

0

17

0

18

0

19

0

DEDT[4:0]

26
Res.

0

20

27
Res.

0

21

28
M1

0

22

29
FIFOEN

0

23

30
TXFEIE

Reset value

24

31

0x00

LPUART_CR1F
IFO mode
enabled

DEAT[4:0]

Register name

25

Offset

RXFFIE

Table 695. LPUART register map and reset values

0

0

0

0

0

0

0

0

0

0

0x2C

LPUART_
PRESC

Reset value

RM0456 Rev 6

Reset value

0

0
NE
FE
PE

IDLE
ORE
NE
FE
PE

0

0

0

0

5
4
3
2
1
0

IDLEIE
TE
RE
UESM
UE
0

STOP
[1:0]
Res.
Res.
Res.
Res.
Res.
Res.

Reset value

0

0

0
0
0
0
0

0

RDR[8:0]

TDR[8:0]

0

0

PRESCALE
R[3:0]

0
0
0
0

0

0

0

0

0

0

0

0

0

0

0
0
0
0
0

0
0
0
0

0

EIE

Res.

Res.

HDSEL

Res.

Res.

Res.
ADDM7
Res.
Res.
Res.
Res.

HDSEL
Res.
Res.
EIE

Res.

Res.

DMAR
DMAR

DMAT
DMAT

0

Res.

6
RXNEIE

0

RTSE
RTSE

0

SBKRQ

0

Res.

7
TCIE

0

CTSE
CTSE

0

MMRQ

0

Res.

8
TXEIE

0

CTSIE

0

RXFRQ

0

Res.

9

PS
PEIE

0

CTSIE
0

TXFRQ

0

Res.

11
10

PCE

0

0

PECF

ORE

RXFNE

0

Res.

12
WAKE

0

Res.

0

0

FECF

IDLE

0

IDLECF

TC

0

Res.

Res.

0

0

NECF

RXNE

0

Res.

TXFNF

0

Res.

13
M0

OVRDIS
OVRDIS

0

0

ORECF

TC

0

TCCF

0

TXE

Res.

CTSIF

0

Res.

14
MME

DDRE
DDRE

0

0

Res.

0
1

Res.

Res.

CTSIF

CTSCF

CTS

0

Res.

18

0

0

Res.

Reset value
Res.

0

Res.

Res.

CTS

0

Res.
0

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

0

15
0

CMIE
DEM

0

0

Res.

DEM

0

Res.

0

Res.

Res.

Res.

0

Res.

Res.

0

0

0

Res.

Res.

Res.

Res.

Res.

0

0

0

Res.

Res.

Res.

Res.

0

Res.

0

Res.

Res.

16
SWAP
0

DEP

RXINV
0

DEP

19
DEDT[4:0]

20

17

TXINV
0

Res.

Reserved
0

Res.

Res.

DATAINV
0

Res.

0
Res.

0

Res.

0

Res.

Reset value

Res.

BUSY

Res.

0

Res.

BUSY

0

Res.

Res.

MSBFIRST
0

Res.

0

Res.

Res.

CMF

Res.

21

0

Res.

Res.

Reset value

Res.

CMF

0

CMCF

0

Res.

Res.

0

Res.

SBKF

22

23

24
DEAT[4:0]

0

Res.

Res.

Res.

Res.

25

0

Res.

SBKF

0

Res.

Res.

Res.

Res.

Res.

TXFTIE

26

0

Res.

Res.

Res.

Res.

Res.

Res.

27
Res.

0

Res.

RWU

Res.

Res.

Res.

Res.

28
Res.

0

Res.

RWU

0

Res.

Res.

0x100x14

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

TEACK

TEACK

0

Res.

Res.

0

Res.

Res.

REACK

0

REACK
0

Res.

1

Res.

Res.

Res.

Reset value

Res.

TXFF

0

Res.

Res.

RXFTCFG[2:0]

29
M1

0

Res.

Res.

Res.

ADD[7:0]

Res.

Res.

RXFTIE

0

Res.

Res.

30
FIFOEN

31
Res.

Res.

0

Res.

Res.

Res.

0x08

Res.

0

Res.

0

RXFF

0

Res.

Res.

0

Res.

Res.

0

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

LPUART_CR3
FIFO mode
disabled
TXFTCFG[2:0]

0x08
LPUART_CR3
FIFO mode
enabled

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

0

Res.

LPUART_TDR

Res.

LPUART_RDR
0

Res.

0x28
LPUART_CR2

Res.

0x24
LPUART_ICR
Res.

0x1C
LPUART_ISRFI
FO mode
disabled
0

Res.

Reset value

0x1C
LPUART_ISRFI
FO mode
enabled

Res.

Reset value

Res.

0x20
LPUART_RQR
0

Res.

0x18
LPUART_BRR

Res.

0x00
LPUART_CR1F
IFO mode
disabled

Res.

0x0C

Res.

Register name

Res.

Reset value

Res.

Offset

Res.

0x04

Res.

RM0456
Low-power universal asynchronous receiver transmitter (LPUART)

Table 695. LPUART register map and reset values (continued)

0

0

0

BRR[19:0]
0

1
0
0
0
0
0
0
0

0

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

5

4

3

2

1

0

6

7

8

9

11

10

12

0

13

0

14

0

15

16

0

17

0

TRIGPOL

0

18

0

TRIGEN

19

20

21

22

24

25

26

27

28

29

23
Res.

Res.

Res.

Res.

Res.

Res.

TRIGSEL
[3:0]

IDLEDIS

Reset value

30

LPUART_
AUTOCR

Res.

0x30

Res.

Register name

Res.

Offset

31

Table 695. LPUART register map and reset values (continued)

0

0

0

0

0

0

TDN[15:0]
0

0

0

0

0

0

0

0

0

0

Refer to Section 2.3: Memory organization for the register boundary addresses.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)

68

Serial peripheral interface (SPI)

68.1

Introduction
The serial peripheral interface (SPI) can be used to communicate with external devices
while using the specific synchronous protocol. The SPI protocol supports half-duplex, fullduplex, and simplex synchronous, serial communication with external devices. The interface
can be configured as master or slave and can operate in multislave or multimaster
configurations. The device configured as master provides a communication clock (SCK) to
the slave device. The slave select (SS) and ready (RDY) signals can be applied optionally
just to set up communication with a concrete slave and to ensure it handles the data flow
properly. The Motorola data format is used by default, but some other specific modes are
supported as well.

68.2

SPI main features
•

Full-duplex synchronous transfers on three lines

•

Half-duplex synchronous transfer on two lines (with bidirectional data line)

•

Simplex synchronous transfers on two lines (with unidirectional data line)

•

From 4-bit up to 32-bit data size selection

•

Multimaster or multislave mode capability

•

Dual clock domain, the peripheral kernel clock is independent from the APB bus clock

•

Baud rate prescaler up to kernel frequency/2 or bypass from RCC in master mode

•

Protection of configuration and setting

•

Hardware or software management of SS for both master and slave

•

Adjustable minimum delays between data and between SS and data flow

•

Configurable SS signal polarity and timing, MISO x MOSI swap capability

•

Programmable clock polarity and phase

•

Programmable data order with MSB-first or LSB-first shifting

•

Programmable number of data within a transfer to control SS and CRC

•

Dedicated transmission and reception flags with interrupt capability

•

SPI Motorola and TI format support

•

Hardware CRC can verify the integrity of the communication at the end of a transfer by:
–

Adding CRC value in Tx mode

–

Automatic CRC error checking for Rx mode

•

Error detection with interrupt capability in case of data overrun, CRC error, data
underrun, the mode fault, and frame error, depending on the operating mode

•

Two 8-bit width embedded Rx and Tx FIFOs (FIFO size depends on instances)

•

Configurable FIFO thresholds (to handle the data packets)

•

Capability to handle data streams by system DMA controller

•

Configurable behavior at slave underrun condition (support of cascaded circular
buffers)

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

68.3

RM0456

•

Autonomous functionality in Stop modes (handling of the transfer flow and required
clock distribution) with wake-up from Stop capability

•

Optional status pin RDY signalizing that the slave device is ready to handle the data
flow

SPI implementation
The table below describes the SPI implementation.
Table 696. SPI features
SPI1, SPI2 (full-featured
instances)

SPI3 (limited-featured instance)

Configurable from 4 to 32 bits

8/16 bits

CRC polynomial length configurable
from 5 to 33 bits

CRC polynomial length 9/17 bits

FIFO size

16 x 8 bits

8 x 8 bits

FIFO threshold

1 - 16 data

1 - 4 data

Number of data control

Up to 65535

Up to 1023, no remaining data
counter (CTSIZE)

Autonomous in Stop mode with wakeup capability

Yes

Yes

Autonomous in Standby mode
with wake-up capability

No

No

SPI feature
Data size
CRC computation

Note:

<!-- pagebreak -->

For detailed information about instance capabilities to exit from Stop and Standby modes,
refer to Table 702: SPI wake-up and interrupt requests and Table 100: Functionalities
depending on the working mode.

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)

68.4

SPI functional description

68.4.1

SPI block diagram
The SPI enables synchronous, serial communications between the MCU and external
devices. The application software can manage the communication by polling the status flag
or using a dedicated SPI interrupt. The main SPI elements and their interactions are shown
in Figure 846.
Figure 846. SPI block diagram

SPI
Serial interface clock domain

RxFIFO

SPI_UDRDR

SPI_RXDR
SPI_AUTOCR

spi_pclk

spi_pclk
clock domain

CRC
Controller
TX Shift Reg
UDR
Controller

SYNC

SPI_TXDR

SPI_CR1
SPI_CR2
SPI_CFG[2:1]

SS

TxFIFO

32-bit APB bus

SPI_CRCPOLY
SPI_TXCRC
SPI_RXCRC

spi_trg[15:0]

RDY

SYNC

COM Controller
SPI_IER
SPI_SR
SPI_IFCR

RDY
Logic

DMA Interface

SS Logic

IRQ Interface

SYNC

spi_wkup
spi_it
spi_pclk_req
spi_tx_dma
spi_rx_dma

...

MOSI

RX Shift Reg

MISO

Clock Generator

SCK

...

Prescaler
spi_ker_ck clock domain

spi_ker_ck
spi_ker_ck_req
MSv50500V1

The simplified scheme of Figure 846 shows three fully independent clock domains:
•

The spi_pclk clock domain

•

The spi_ker_ck kernel clock domain

•

The serial interface clock domain

All the control and status signals between these domains are strictly synchronized. There is
no specific constraint concerning the frequency ratio between these clock signals. The user
has to consider a ratio compatible with the data flow speed to avoid data underrun or
overrun events.

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456

The spi_pclk clock signal feeds the peripheral bus interface. It must be active when
accesses to the SPI registers are required.
The SPI working in slave mode handles a data flow using the serial interface clock derived
from the external SCK signal provided by the external master SPI device. That is why the
SPI slave is able to receive and send data even when the spi_pclk and spi_ker_ck clock
signals are inactive. As a consequence, a specific slave logic working within the serial
interface clock domain needs some additional traffic to be set up correctly (for example
when underrun or overrun is evaluated, see Section 68.5.2 for details). This cannot be done
when the bus becomes idle. In some specific cases, the slave even requires the clock
generator working (see Section 68.5.1).
When the SPI works as a master, the RCC must provide the spi_ker_ck kernel clock to the
peripheral during communication to feed the serial interface clock via the clock generator
where it can be divided by prescaler or bypassed. The signal is then provided to the slaves
via the SCK pin and internally to the serial interface domain of the master.

68.4.2

SPI pins and internal signals
Up to five I/O pins are dedicated to SPI communication with external devices.
•

MISO: master in / slave out data. In the general case, this pin is used to transmit data in
slave mode and receive data in master mode.

•

MOSI: master out / slave in data. In the general case, this pin is used to transmit data in
master mode and receive data in slave mode.

•

SCK: serial clock output pin for SPI masters and input pin for SPI slaves.

•

SS: slave select pin. Depending on the SPI and SS settings, this pin can be used to
either:
–

Select an individual slave device for communication

–

Synchronize the data frame, or

–

Detect a conflict between multiple masters

See Section 68.4.7 for details.
•

RDY: optional status pin signaling slave FIFO occupancies and so the slave availability
to continue the transfer without any risk of data flow corruption. It can be checked by
the master to control the temporal suspension of the ongoing communication.

The SPI bus enables the communication between one master device and one or more slave
devices. The bus consists of at least two wires: one for the clock signal and the other for
synchronous data transfer. Other signals are optional and can be added depending on the
data exchange between SPI nodes and their communication control management.
Refer to Table 697 and Table 698 for the list of SPI input / output pins and internal signals.
Table 697. SPI input/output pins
Pin name

Description

MISO(1)

Input/output Master data input / slave data output

(1)

Input/output Master data output / slave data input

MOSI

<!-- pagebreak -->

I/O type

SCK

Input/output Master clock output / slave clock input

SS

Input/output Master output / slave selection input

RDY

Input/output SPI master input / slave FIFOs status occupancy output

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)

1. Functionality of MOSI and MISO pins can be swapped. Their directions may vary in SPI bidirectional halfduplex mode.

Description of SPI input/output signals
Table 698. SPI internal input/output signals
Signal name

Signal type

Description

spi_pclk

Input

SPI clock signal feeds the peripheral bus interface

spi_ker_ck

Input

SPI kernel clock

spi_ker_ck_req

Output

SPI kernel clock request

spi_pclk_req

Output

SPI clock request

spi_wkup

Output

SPI provides a wake-up interrupt

spi_it

Output

SPI global interrupt

spi_tx_dma

Input/output

SPI transmit DMA request

spi_rx_dma

Input/output

SPI receive DMA request

spi_trg[15:0]

Input

SPI triggers

Description of SPI interconnections
Table 699. SPI interconnection (SPI1 and SPI2)
Signal name

Trigger source

spi_trg0

gpdma1_ch0_tc

spi_trg1

gpdma1_ch1_tc

spi_trg2

gpdma1_ch2_tc

spi_trg3

gpdma1_ch3_tc

spi_trg4

exti4

spi_trg5

exti9

spi_trg6

lptim1_ch1

spi_trg7

lptim2_ch1

spi_trg8

comp1_out

spi_trg9

comp2_out(1)

spi_trg10

rtc_alra_trg

spi_trg11

rtc_wut_trg

spi_trg12

-

spi_trg13

-

spi_trg14

-

spi_trg15

-

1. This connection is not present in STM32U535/545 since COMP2 is not available.

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456
Table 700. SPI interconnection (SPI3)

Signal name

Trigger source

spi_trg0

lpdma1_ch0_tc

spi_trg1

lpdma1_ch1_tc

spi_trg2

lpdma1_ch2_tc

spi_trg3

lpdma1_ch3_tc

spi_trg4

exti4

spi_trg5

exti8

spi_trg6

lptim1_ch1

spi_trg7

lptim3_ch1

spi_trg8

comp1_out

spi_trg9

comp2_out(1)

spi_trg10

rtc_alra_trg

spi_trg11

rtc_wut_trg

spi_trg12

-

spi_trg13

-

spi_trg14

-

spi_trg15

-

1. This connection is not present in STM32U535/545 since COMP2 is not available.

68.4.3

SPI communication general aspects
The SPI allows the MCU to communicate using different configurations, depending on the
device targeted and the application requirements. These configurations use two or three
wires (with software SS management) or three or four wires (with hardware SS
management). The communication is always initiated and controlled by the master. The
master provides a clock signal on the SCK line and selects or synchronizes slaves for
communication by SS line when it is managed by hardware.
The data between the master and the slave flow synchronously on the MOSI and/or MISO
lines.

68.4.4

Communications between one master and one slave
The communication flow can use one of three possible modes: the full-duplex (three wires)
mode, half-duplex (two wires) mode, or the simplex (two wires) mode. The SS signal is
optional in single master-slave configuration and is often not connected between the two
communication nodes. Nevertheless, the SS signal can be helpful in this configuration to
synchronize the data flow and it is used by default for some specific SPI modes (for example
the TI mode).
The next optional RDY signal can help to ensure the correct management of all the
transferred data at slave side.

<!-- pagebreak -->

