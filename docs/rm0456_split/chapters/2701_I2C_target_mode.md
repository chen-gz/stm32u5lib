2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Table 683. USART interrupt requests (continued)
Interrupt
vector

USART or
UART

Interrupt event

Event
flag

Enable
Control
bit

Interrupt clear
method

Receive data
register not empty
(data ready to be
read)

RXNE

RXNEIE

Read RDR or write
1 in RXFRQ

Yes

Receive FIFO not
empty

RXFNE

Read RDR until
RXFNEIE RXFIFO empty or
write 1 in RXFRQ

Yes

Receive FIFO full

RXFF(3)

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

No

Parity error

PE

PEIE

Write 1 in PECF

Yes(4)

LIN break

LBDF

LBDIE

Write 1 in LBDCF

Noise error in
multibuffer
communication.

NE

Overrun error in
multibuffer
communication.

ORE(5)

Framing error in
multibuffer
communication.

FE

Character match

CMF

Receiver timeout

Exit from Exit from
Sleep
Stop(1)
modes
mode

Yes

No

Write 1 in NFCF

Yes(4)

Write 1 in ORECF

Yes

Write 1 in FECF

Yes(4)

CMIE

Write 1 in CMCF

Yes(6)

RTOF

RTOFIE

Write 1 in RTOCCF

No

End of Block

EOBF

EOBIE

Write 1 in EOBCF

No

SPI slave underrun
error

UDR

EIE

Write 1 in UDRCF

No

EIE

Exit from
Standby
mode

No

1. The USART can wake up the device from Stop mode only if the peripheral instance supports the wake-up from Stop mode
feature. Refer to Section 66.4: USART implementation for the list of supported Stop modes.
2. Writing to TDR clears the TXFT flag only if the number of empty locations is less than the value programmed in
TXFTCFG[2:0].
3. RXFF flag is asserted if the USART receives n+1 data (n being the RXFIFO size): n data in the RXFIFO and 1 data in
USART_RDR. In Stop mode, USART_RDR is not clocked. As a result, this register is not written and once n data are
received and written in the RXFIFO, the RXFF interrupt is asserted (RXFF flag is not set).
4. Parity/Noise/Framing error interrupts enable waking up from Stop modes when the DMA is used.
5. When OVRDIS = 0.
6. The DMA must be used when the FIFO mode is enabled.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

66.8

USART registers
Refer to Section 1.2 for a list of abbreviations used in register descriptions.
The peripheral registers have to be accessed by words (32 bits).

66.8.1

USART control register 1 (USART_CR1)
Address offset: 0x00
Reset value: 0x0000 0000
The same register can be used in FIFO mode enabled (this section) and FIFO mode
disabled (next section).
FIFO mode enable, FIFOEN = 1

31

30

29

28

27

26

RXF
FIE

TXFEIE

FIFO
EN

M1

EOBIE

RTOIE

25

24

23

22

21

20

19

DEAT[4:0]

18

17

16

rw

DEDT[4:0]

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

OVER8

CMIE

MME

M0

WAKE

PCE

PS

PEIE

TXFNFIE

TCIE

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

rw

rw

rw

rw

rw

rw

rw

RXFNEIE IDLEIE
rw

rw

Bit 31 RXFFIE: RXFIFO Full interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated when RXFF = 1 in the USART_ISR register
Bit 30 TXFEIE: TXFIFO empty interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated when TXFE = 1 in the USART_ISR register
Bit 29 FIFOEN: FIFO mode enable
This bit is set and cleared by software.
0: FIFO mode is disabled.
1: FIFO mode is enabled.
This bitfield can only be written when the USART is disabled (UE = 0).
Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode
and in smartcard modes only. It must not be enabled in IrDA and LIN modes.
Bit 28 M1: Word length
This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or
cleared by software.
M[1:0] = 00: 1 start bit, 8 Data bits, n stop bits
M[1:0] = 01: 1 start bit, 9 Data bits, n stop bits
M[1:0] = 10: 1 start bit, 7 Data bits, n stop bits
This bit can only be written when the USART is disabled (UE = 0).
Note: In 7-bits data length mode, the smartcard mode, LIN master mode and auto baud rate
(0x7F and 0x55 frames detection) are not supported.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bit 27 EOBIE: End of block interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated when the EOBF flag is set in the USART_ISR register
Note: If the USART does not support smartcard mode, this bit is reserved and must be kept at
reset value. Refer to Section 66.4: USART implementation.
Bit 26 RTOIE: Receiver timeout interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated when the RTOF bit is set in the USART_ISR register.
Note: If the USART does not support the Receiver timeout feature, this bit is reserved and
must be kept at reset value. Section 66.4: USART implementation.
Bits 25:21 DEAT[4:0]: Driver Enable assertion time
This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and
the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time,
depending on the oversampling rate).
This bitfield can only be written when the USART is disabled (UE = 0).
Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at
reset value. Refer to Section 66.4: USART implementation.
Bits 20:16 DEDT[4:0]: Driver Enable deassertion time
This 5-bit value defines the time between the end of the last stop bit, in a transmitted
message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample
time units (1/8 or 1/16 bit time, depending on the oversampling rate).
If the USART_TDR register is written during the DEDT time, the new data is transmitted only
when the DEDT and DEAT times have both elapsed.
This bitfield can only be written when the USART is disabled (UE = 0).
Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at
reset value. Refer to Section 66.4: USART implementation.
Bit 15 OVER8: Oversampling mode
0: Oversampling by 16
1: Oversampling by 8
This bit can only be written when the USART is disabled (UE = 0).
Note: In LIN, IrDA and smartcard modes, this bit must be kept cleared.
Bit 14 CMIE: Character match interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated when the CMF bit is set in the USART_ISR register.
Bit 13 MME: Mute mode enable
This bit enables the USART mute mode function. When set, the USART can switch between
active and mute mode, as defined by the WAKE bit. It is set and cleared by software.
0: Receiver in active mode permanently
1: Receiver can switch between mute mode and active mode.
Bit 12 M0: Word length
This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or
cleared by software (refer to bit 28 (M1)description).
This bit can only be written when the USART is disabled (UE = 0).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bit 11 WAKE: Receiver wake-up method
This bit determines the USART wake-up method from mute mode. It is set or cleared by
software.
0: Idle line
1: Address mark
This bitfield can only be written when the USART is disabled (UE = 0).
Bit 10 PCE: Parity control enable
This bit selects the hardware parity control (generation and detection). When the parity
control is enabled, the computed parity is inserted at the MSB position (9th bit if M = 1; 8th bit
if M=0) and the parity is checked on the received data. This bit is set and cleared by software.
Once it is set, PCE is active after the current byte (in reception and in transmission).
0: Parity control disabled
1: Parity control enabled
This bitfield can only be written when the USART is disabled (UE = 0).
Bit 9 PS: Parity selection
This bit selects the odd or even parity when the parity generation/detection is enabled (PCE
bit set). It is set and cleared by software. The parity is selected after the current byte.
0: Even parity
1: Odd parity
This bitfield can only be written when the USART is disabled (UE = 0).
Bit 8 PEIE: PE interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated whenever PE = 1 in the USART_ISR register
Bit 7 TXFNFIE: TXFIFO not full interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated whenever TXFNF = 1 in the USART_ISR register
Bit 6 TCIE: Transmission complete interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated whenever TC = 1 in the USART_ISR register
Bit 5 RXFNEIE: RXFIFO not empty interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated whenever ORE = 1 or RXFNE = 1 in the USART_ISR register
Bit 4 IDLEIE: IDLE interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated whenever IDLE = 1 in the USART_ISR register

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bit 3 TE: Transmitter enable
This bit enables the transmitter. When the autonomous mode is not used, TE bit is set and
cleared by software. When the autonomous mode is used, TE bit becomes a status bit, which
is set and cleared by hardware.
0: Transmitter is disabled
1: Transmitter is enabled
Note: When the USART acts as a transmitter, a low pulse on the TE bit (0 followed by 1)
sends a preamble (idle line) after the current word, except in smartcard mode. In order
to generate an idle character, the TE must not be immediately written to 1. To ensure the
required duration, the software can poll the TEACK bit in the USART_ISR register.
In smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission
starts.
Bit 2 RE: Receiver enable
This bit enables the receiver. It is set and cleared by software.
0: Receiver is disabled
1: Receiver is enabled and begins searching for a start bit
Bit 1 UESM: USART enable in low-power mode
When this bit is cleared, the USART cannot request its kernel clock and is not functional in
low-power mode.
When this bit is set, the USART can wake up the MCU from low-power mode.
This bit is set and cleared by software.
0: USART not functional in low-power mode.
1: USART functional in low-power mode.
Note: The UESM bit must be set at the initialization phase.
If the USART does not support the wake-up from low-power mode, this bit is reserved
and must be kept at reset value. Refer to Section 66.4: USART implementation.
Bit 0 UE: USART enable
When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all
current operations are discarded. The USART configuration is kept, but all the USART_ISR
status flags are reset. This bit is set and cleared by software.
0: USART prescaler and outputs disabled, low-power mode
1: USART enabled
Note: To enter low-power mode without generating errors on the line, the TE bit must be
previously reset and the software must wait for the TC bit in the USART_ISR to be set
before resetting the UE bit.
The DMA requests are also reset when UE = 0 so the DMA channel must be disabled
before resetting the UE bit.
In smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1,
regardless of the UE bit value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

66.8.2

USART control register 1 [alternate] (USART_CR1)
Address offset: 0x00
Reset value: 0x0000 0000
The same register can be used in FIFO mode enabled (previous section) and FIFO mode
disabled (this section).
FIFO mode disabled, FIFOEN = 0

31

30

29

28

27

26

Res.

Res.

FIFO
EN

25

24

23

22

21

20

19

M1

EOBIE

RTOIE

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

rw

rw

rw

2

1

0

OVER8

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

TE

RE

UESM

UE

rw

rw

rw

rw

DEAT[4:0]

18
DEDT[4:0]

RXNEIE IDLEIE
rw

rw

Bits 31:30 Reserved, must be kept at reset value.
Bit 29 FIFOEN: FIFO mode enable
This bit is set and cleared by software.
0: FIFO mode is disabled.
1: FIFO mode is enabled.
This bitfield can only be written when the USART is disabled (UE = 0).
Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode
and in smartcard modes only. It must not be enabled in IrDA and LIN modes.
Bit 28 M1: Word length
This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or
cleared by software.
M[1:0] = 00: 1 start bit, 8 Data bits, n stop bits
M[1:0] = 01: 1 start bit, 9 Data bits, n stop bits
M[1:0] = 10: 1 start bit, 7 Data bits, n stop bits
This bit can only be written when the USART is disabled (UE = 0).
Note: In 7-bits data length mode, the smartcard mode, LIN master mode and auto baud rate
(0x7F and 0x55 frames detection) are not supported.
Bit 27 EOBIE: End of block interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated when the EOBF flag is set in the USART_ISR register
Note: If the USART does not support smartcard mode, this bit is reserved and must be kept at
reset value. Refer to Section 66.4: USART implementation.
Bit 26 RTOIE: Receiver timeout interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated when the RTOF bit is set in the USART_ISR register.
Note: If the USART does not support the Receiver timeout feature, this bit is reserved and
must be kept at reset value. Section 66.4: USART implementation.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bits 25:21 DEAT[4:0]: Driver Enable assertion time
This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and
the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time,
depending on the oversampling rate).
This bitfield can only be written when the USART is disabled (UE = 0).
Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at
reset value. Refer to Section 66.4: USART implementation.
Bits 20:16 DEDT[4:0]: Driver Enable deassertion time
This 5-bit value defines the time between the end of the last stop bit, in a transmitted
message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample
time units (1/8 or 1/16 bit time, depending on the oversampling rate).
If the USART_TDR register is written during the DEDT time, the new data is transmitted only
when the DEDT and DEAT times have both elapsed.
This bitfield can only be written when the USART is disabled (UE = 0).
Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at
reset value. Refer to Section 66.4: USART implementation.
Bit 15 OVER8: Oversampling mode
0: Oversampling by 16
1: Oversampling by 8
This bit can only be written when the USART is disabled (UE = 0).
Note: In LIN, IrDA and smartcard modes, this bit must be kept cleared.
Bit 14 CMIE: Character match interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated when the CMF bit is set in the USART_ISR register.
Bit 13 MME: Mute mode enable
This bit enables the USART mute mode function. When set, the USART can switch between
active and mute mode, as defined by the WAKE bit. It is set and cleared by software.
0: Receiver in active mode permanently
1: Receiver can switch between mute mode and active mode.
Bit 12 M0: Word length
This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or
cleared by software (refer to bit 28 (M1)description).
This bit can only be written when the USART is disabled (UE = 0).
Bit 11 WAKE: Receiver wake-up method
This bit determines the USART wake-up method from mute mode. It is set or cleared by
software.
0: Idle line
1: Address mark
This bitfield can only be written when the USART is disabled (UE = 0).
Bit 10 PCE: Parity control enable
This bit selects the hardware parity control (generation and detection). When the parity
control is enabled, the computed parity is inserted at the MSB position (9th bit if M = 1; 8th bit
if M = 0) and the parity is checked on the received data. This bit is set and cleared by
software. Once it is set, PCE is active after the current byte (in reception and in
transmission).
0: Parity control disabled
1: Parity control enabled
This bitfield can only be written when the USART is disabled (UE = 0).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bit 9 PS: Parity selection
This bit selects the odd or even parity when the parity generation/detection is enabled (PCE
bit set). It is set and cleared by software. The parity is selected after the current byte.
0: Even parity
1: Odd parity
This bitfield can only be written when the USART is disabled (UE = 0).
Bit 8 PEIE: PE interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated whenever PE = 1 in the USART_ISR register
Bit 7 TXEIE: Transmit data register empty
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated whenever TXE = 1 in the USART_ISR register
Bit 6 TCIE: Transmission complete interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated whenever TC = 1 in the USART_ISR register
Bit 5 RXNEIE: Receive data register not empty
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated whenever ORE = 1 or RXNE = 1 in the USART_ISR register
Bit 4 IDLEIE: IDLE interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated whenever IDLE = 1 in the USART_ISR register
Bit 3 TE: Transmitter enable
This bit enables the transmitter. When the autonomous mode is not used, TE bit is set and
cleared by software. When the autonomous mode is used, TE bit becomes a status bit, which
is set and cleared by hardware.
0: Transmitter is disabled
1: Transmitter is enabled
Note: When the USART acts as a transmitted, a low pulse on the TE bit (0 followed by 1)
sends a preamble (idle line) after the current word, except in smartcard mode. In order
to generate an idle character, the TE must not be immediately written to 1. To ensure the
required duration, the software can poll the TEACK bit in the USART_ISR register.
In smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission
starts.
Bit 2 RE: Receiver enable
This bit enables the receiver. It is set and cleared by software.
0: Receiver is disabled
1: Receiver is enabled and begins searching for a start bit

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bit 1 UESM: USART enable in low-power mode
When this bit is cleared, the USART cannot request its kernel clock and is not functional in
low-power mode.
When this bit is set, the USART can wake up the MCU from low-power mode.
This bit is set and cleared by software.
0: USART not functional in low-power mode.
1: USART functional in low-power mode.
Note: The UESM bit must be set at the initialization phase.
If the USART does not support the wake-up from low-power mode, this bit is reserved
and must be kept at reset value. Refer to Section 66.4: USART implementation.
Bit 0 UE: USART enable
When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all
current operations are discarded. The USART configuration is kept, but all the USART_ISR
status flags are reset. This bit is set and cleared by software.
0: USART prescaler and outputs disabled, low-power mode
1: USART enabled
Note: To enter low-power mode without generating errors on the line, the TE bit must be
previously reset and the software must wait for the TC bit in the USART_ISR to be set
before resetting the UE bit.
The DMA requests are also reset when UE = 0 so the DMA channel must be disabled
before resetting the UE bit.
In smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1,
regardless of the UE bit value.

66.8.3

USART control register 2 (USART_CR2)
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
RTOEN

22

21

ABRMOD[1:0]

20
ABREN

19

18

17

MSBFI
DATAINV TXINV
RST

16
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

Res.

SLVEN

SWAP

LINEN

rw

rw

<!-- pagebreak -->

