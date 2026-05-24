RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

LPUART reception mode
•

If the FIFO mode is enabled, the APB clock is requested when
–

The RxFIFIO is full (RXFF = 1) and the corresponding interrupt is enabled
(RXFFIE = 1)

–

The RxFIFO threshold is reached (RXFT = 1) and the corresponding interrupt is
enabled (RXFTIE = 1)

–
•

Note:

The RxFIFO is not empty (RXFNE = 1) and the corresponding interrupt or DMA is
enabled (RXFNEIE= 1)
If the FIFO mode is disabled, the APB clock is requested when the LPUART finishes
sampling data and it is ready to be written in the LPUART_RDR. The DMA or the
associated interrupt must be enabled.

The APB clock is requested in reception mode when an overrun error occurs (ORE = 1).
The EIE bit must be set to enable the generation an interrupt and waking up the MCU, and
the OVRDIS bit must remain cleared. The APB clock request is kept until the interrupt flag is
cleared.
In reception mode, the APB clock is requested when a Parity/Noise/Framing error occurs
and the DMA is used for reception. The APB clock request is kept until the interrupt flag is
cleared.

Determining the maximum LPUART baud rate that enables to correctly wake
up the MCU from low-power mode
The maximum baud rate that enables to correctly wake up the MCU from low-power mode
depends on the wake-up time parameter (refer to the device datasheet) and on the LPUART
receiver tolerance (see Section 67.4.9: Tolerance of the LPUART receiver to clock
deviation).
Let us take the example of OVER8 = 0, M bits = 01, ONEBIT = 0 and BRR [3:0] = 0000.
In these conditions, according to Table 691: Tolerance of the LPUART receiver, the
LPUART receiver tolerance equals 3.41%.
DTRA + DQUANT + DREC + DTCL + DWU < LPUART receiver tolerance
DWUmax = tWULPUART/ (11 x Tbit Min)
Tbit Min = tWULPUART/ (11 x DWUmax)
where tWULPUART is the wake-up time from low-power mode.
If we consider the ideal case where DTRA, DQUANT, DREC, and DTCL parameters are at
0%, the maximum value of DWU is 3.41%. In fact, we need to consider at least the
lpuart_ker_ck inaccuracy.
For example, if HSI is used as lpuart_ker_ck, and the HSI inaccuracy is of 1%, then we
obtain:
tWULPUART = 3 µs (values provided only as examples; for correct values, refer to the
device datasheet).
DWUmax = 3.41% – 1% = 2.41%
Tbit min = 3 µs/ (11 x 2.41%) = 11.32 µs.
As a result, the maximum baud rate that enables to wake up correctly from low-power
mode is: 1/11.32 µs = 88.36 kbauds.

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

67.5

RM0456

LPUART in low-power modes
Table 693. Effect of low-power modes on the LPUART
Mode

Description

Sleep

No effect. LPUART interrupts cause the device to exit Sleep mode.

Stop(1)

The content of the LPUART registers is kept.
If the LPUART is clocked by an oscillator available in Stop mode, transfers
in asynchronous and SPI master modes are functional. DMA requests are
functional, and the interrupts cause the device to exit Stop mode.

Standby

The peripheral is powered down and must be reinitialized after exiting
Standby mode.

1. Refer to Section 67.3: LPUART implementation to know if the wake-up from Stop mode is supported for a
given peripheral instance. If an instance is not functional in a given Stop mode, it must be disabled before
entering this Stop mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

67.6

Low-power universal asynchronous receiver transmitter (LPUART)

LPUART interrupts
Refer to Table 694 for a detailed description of all LPUART interrupt requests.
Table 694. LPUART interrupt requests

Interrupt
vector

LPUART

Event
flag

Enable
Control
bit

Transmit data
register empty

TXE

TXEIE

Write TDR

Yes

Transmit FIFO Not
Full

TXFNF

TXFNFIE TXFIFO full

Yes

Transmit FIFO
Empty

TXFE

TXFEIE

Write TDR or write 1
in TXFRQ

Transmit FIFO
threshold reached

TXFT

TXFTIE

Write TDR

Yes

CTS interrupt

CTSIF

CTSIE

Write 1 in CTSCF

No

Transmission
Complete

TC

TCIE

Write TDR or write 1
in TCCF

Yes

Receive data
register not empty
(data ready to be
read)

RXNE

RXNEIE

Read RDR or write
1 in RXFRQ

Yes

Receive FIFO Not
Empty

RXFNE

Read RDR until
RXFNEIE RXFIFO empty or
write 1 in RXFRQ

Yes

Receive FIFO Full

RXFF(2)

RXFFIE

Read RDR

Yes

Receive FIFO
threshold reached

RXFT

RXFTIE

Read RDR

Yes

Overrun error
detected

ORE

RXNEIE/RX
FNEIE

Write 1 in ORECF

Yes

Idle line detected

IDLE

IDLEIE

Write 1 in IDLECF

Parity error

PE

PEIE

Write 1 in PECF

Yes(3)

Noise error in
multibuffer
communication.

NE

Write 1 in NFCF

Yes(3)

Overrun error in
multibuffer
communication.

ORE(4)

Write 1 in ORECF

Yes

Framing Error in
multibuffer
communication.

FE

Write 1 in FECF

Yes(3)

Character match

CMF

Write 1 in CMCF

Yes(5)

Interrupt event

EIE

CMIE

Interrupt clear
method

Exit from Exit from
Sleep
Stop(1)
modes
mode

Yes

.Yes

Exit from
Standby
mode

Yes

No

No

1. The LPUART can wake up the device from Stop mode only if the peripheral instance supports the wake-up from Stop mode
feature. Refer to Section 67.3: LPUART implementation for the list of supported Stop modes.

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

2. RXFF flag is asserted if the LPUART receives n+1 data (n being the RXFIFO size): n data in the RXFIFO and 1 data in
LPUART_RDR. In Stop mode, LPUART_RDR is not clocked. As a result, this register is not written and once n data are
received and written in the RXFIFO, the RXFF interrupt is asserted (RXFF flag is not set).
3. Parity/Noise/Framing error interrupts enable waking up from Stop modes when the DMA is used.
4. When OVRDIS = 0.
5. The DMA must be used when the FIFO mode is enabled.

67.7

LPUART registers
Refer to Section 1.2: List of abbreviations for registers for a list of abbreviations used in
register descriptions.
The peripheral registers have to be accessed by words (32 bits).

67.7.1

LPUART control register 1 (LPUART_CR1)
Address offset: 0x00
Reset value: 0x0000 0000
The same register can be used in FIFO mode enabled (this section) and FIFO mode
disabled (next section).
FIFO mode enabled, FIFOEN = 1
30

29

28

27

26

RXF
FIE

31

TXFEIE

FIFO
EN

M1

Res.

Res.

rw

rw

rw

rw

15

14

13

12

Res.

11

10

25

24

23

22

21

20

19

DEAT[4:0]
rw

rw

9

rw

8

7

CMIE

MME

M0

WAKE

PCE

PS

PEIE

rw

rw

rw

rw

rw

rw

rw

rw

17

16

DEDT[4:0]

rw

TXFN
FIE

18

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

TCIE

RXFN
EIE

IDLEIE

TE

RE

UESM

UE

rw

rw

rw

rw

rw

rw

rw

Bit 31 RXFFIE:RXFIFO Full interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: An LPUART interrupt is generated when RXFF = 1 in the LPUART_ISR register
Bit 30 TXFEIE:TXFIFO empty interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: An LPUART interrupt is generated when TXFE = 1 in the LPUART_ISR register
Bit 29 FIFOEN:FIFO mode enable
This bit is set and cleared by software.
0: FIFO mode is disabled.
1: FIFO mode is enabled.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

Bit 28 M1: Word length
This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or
cleared by software.
M[1:0] = 00: 1 Start bit, 8 Data bits, n stop bits
M[1:0] = 01: 1 Start bit, 9 Data bits, n stop bits
M[1:0] = ‘10: 1 Start bit, 7 Data bits, n stop bits
This bit can only be written when the LPUART is disabled (UE = 0).
Note: In 7-bit data length mode, the smartcard mode, LIN master mode and auto baud rate
(0x7F and 0x55 frames detection) are not supported.
Bits 27:26 Reserved, must be kept at reset value.
Bits 25:21 DEAT[4:0]: Driver Enable assertion time
This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and
the beginning of the start bit. It is expressed in lpuart_ker_ck clock cycles. For more details,
refer Section 66.5.21: RS232 hardware flow control and RS485 driver enable.
This bitfield can only be written when the LPUART is disabled (UE = 0).
Bits 20:16 DEDT[4:0]: Driver Enable deassertion time
This 5-bit value defines the time between the end of the last stop bit, in a transmitted
message, and the de-activation of the DE (Driver Enable) signal.It is expressed in
lpuart_ker_ck clock cycles. For more details, refer Section 67.4.14: RS232 hardware flow
control and RS485 driver enable.
If the LPUART_TDR register is written during the DEDT time, the new data is transmitted
only when the DEDT and DEAT times have both elapsed.
This bitfield can only be written when the LPUART is disabled (UE = 0).
Bit 15 Reserved, must be kept at reset value.
Bit 14 CMIE: Character match interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: A LPUART interrupt is generated when the CMF bit is set in the LPUART_ISR register.
Bit 13 MME: Mute mode enable
This bit activates the mute mode function of the LPUART. When set, the LPUART can switch
between the active and mute modes, as defined by the WAKE bit. It is set and cleared by
software.
0: Receiver in active mode permanently
1: Receiver can switch between mute mode and active mode.
Bit 12 M0: Word length
This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or
cleared by software (refer to bit 28 (M1) description).
This bit can only be written when the LPUART is disabled (UE = 0).
Bit 11 WAKE: Receiver wake-up method
This bit determines the LPUART wake-up method from mute mode. It is set or cleared by
software.
0: Idle line
1: Address mark
This bitfield can only be written when the LPUART is disabled (UE = 0).

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Bit 10 PCE: Parity control enable
This bit selects the hardware parity control (generation and detection). When the parity
control is enabled, the computed parity is inserted at the MSB position (9th bit if M = 1; 8th bit
if M = 0) and parity is checked on the received data. This bit is set and cleared by software.
Once it is set, PCE is active after the current byte (in reception and in transmission).
0: Parity control disabled
1: Parity control enabled
This bitfield can only be written when the LPUART is disabled (UE = 0).
Bit 9 PS: Parity selection
This bit selects the odd or even parity when the parity generation/detection is enabled (PCE
bit set). It is set and cleared by software. The parity is selected after the current byte.
0: Even parity
1: Odd parity
This bitfield can only be written when the LPUART is disabled (UE = 0).
Bit 8 PEIE: PE interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: An LPUART interrupt is generated whenever PE = 1 in the LPUART_ISR register
Bit 7 TXFNFIE: TXFIFO not full interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: A LPUART interrupt is generated whenever TXFNF = 1 in the LPUART_ISR register
Bit 6 TCIE: Transmission complete interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: An LPUART interrupt is generated whenever TC = 1 in the LPUART_ISR register
Bit 5 RXFNEIE: RXFIFO not empty interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: A LPUART interrupt is generated whenever ORE = 1 or RXFNE = 1 in the LPUART_ISR
register
Bit 4 IDLEIE: IDLE interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: An LPUART interrupt is generated whenever IDLE = 1 in the LPUART_ISR register
Bit 3 TE: Transmitter enable
This bit enables the transmitter. When the autonomous mode is not used, TE bit is set and
cleared by software. When the autonomous mode is used, TE bit becomes a status bit,
which is set and cleared by hardware.
0: Transmitter is disabled
1: Transmitter is enabled
Note: When the LPUART acts as a transmitter, a low pulse on the TE bit (0 followed by 1)
sends a preamble (idle line) after the current word, except in smartcard mode. In order
to generate an idle character, the TE must not be immediately written to 1. To ensure
the required duration, the software can poll the TEACK bit in the LPUART_ISR register.
In smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission
starts.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

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

67.7.2

LPUART control register 1 [alternate] (LPUART_CR1)
Address offset: 0x00
Reset value: 0x0000 0000
The same register can be used in FIFO mode enabled (previous section) and FIFO mode
disabled (this section).
FIFO mode disabled, FIFOEN = 0

31
Res.

30

29

28

27

26

Res.

FIFO
EN

M1

Res.

Res.

rw

rw

25

24

23

22

21

20

19

DEAT[4:0]
rw

rw

rw

18

17

16

rw

rw

DEDT[4:0]
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

CMIE

MME

M0

WAKE

PCE

PS

PEIE

TXEIE

TCIE

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

5

4

RXNEIE IDLEIE
rw

rw

rw

rw

3

2

1

0

TE

RE

UESM

UE

rw

rw

rw

rw

Bits 31:30 Reserved, must be kept at reset value.
Bit 29 FIFOEN:FIFO mode enable
This bit is set and cleared by software.
0: FIFO mode is disabled.
1: FIFO mode is enabled.

RM0456 Rev 6

<!-- pagebreak -->

