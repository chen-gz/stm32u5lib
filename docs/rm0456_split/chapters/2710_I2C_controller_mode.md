STOP[1:0]
rw

rw

CLKEN

CPOL

CPHA

LBCL

rw

rw

rw

rw

Res.

RM0456 Rev 6

LBDIE

LBDL

ADDM7

DIS_
NSS

rw

rw

rw

rw

rw

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bits 31:24 ADD[7:0]: Address of the USART node
These bits give the address of the USART node in mute mode or a character code to be recognized
in low-power or Run mode:
–
In mute mode: they are used in multiprocessor communication to wake up from mute mode
with 4-bit/7-bit address mark detection. The MSB of the character sent by the transmitter
must be equal to 1. In 4-bit address mark detection, only ADD[3:0] bits are used.
–
In low-power mode: they are used for wake up from low-power mode on character match.
When a character, received during low-power mode, corresponds to the character
programmed through ADD[7:0] bitfield, the CMF flag is set and wakes up the device from
low-power mode if the corresponding interrupt is enabled by setting CMIE bit.
–
In Run mode with mute mode inactive (for example, end-of-block detection in ModBus
protocol): the whole received character (8 bits) is compared to ADD[7:0] value and CMF
flag is set on match. An interrupt is generated if the CMIE bit is set.
These bits can only be written when the USART is disabled (UE = 0).
Bit 23 RTOEN: Receiver timeout enable
This bit is set and cleared by software.
0: Receiver timeout feature disabled.
1: Receiver timeout feature enabled.
When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle
(no reception) for the duration programmed in the RTOR (receiver timeout register).
Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be
kept at reset value. Refer to Section 66.4: USART implementation.
Bits 22:21 ABRMOD[1:0]: Auto baud rate mode
These bits are set and cleared by software.
00: Measurement of the start bit is used to detect the baud rate.
01: Falling edge to falling edge measurement (the received frame must start with a single bit = 1 ->
Frame = Start10xxxxxx)
10: 0x7F frame detection.
11: 0x55 frame detection
This bitfield can only be written when ABREN = 0 or the USART is disabled (UE = 0).
Note: If DATAINV = 1 and/or MSBFIRST = 1 the patterns must be the same on the line, for example
0xAA for MSBFIRST)
If the USART does not support the auto baud rate feature, this bit is reserved and must be kept
at reset value. Refer to Section 66.4: USART implementation.
Bit 20 ABREN: Auto baud rate enable
This bit is set and cleared by software.
0: Auto baud rate detection is disabled.
1: Auto baud rate detection is enabled.
Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept
at reset value. Refer to Section 66.4: USART implementation.
Bit 19 MSBFIRST: Most significant bit first
This bit is set and cleared by software.
0: data is transmitted/received with data bit 0 first, following the start bit.
1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit.
This bitfield can only be written when the USART is disabled (UE = 0).

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bit 18 DATAINV: Binary data inversion
This bit is set and cleared by software.
0: Logical data from the data register are send/received in positive/direct logic. (1 = H, 0 = L)
1: Logical data from the data register are send/received in negative/inverse logic. (1 = L, 0 = H).
The parity bit is also inverted.
This bitfield can only be written when the USART is disabled (UE = 0).
Bit 17 TXINV: TX pin active level inversion
This bit is set and cleared by software.
0: TX pin signal works using the standard logic levels (VDD = 1 / idle, Gnd = 0 / mark)
1: TX pin signal values are inverted. ((VDD = 0 / mark, Gnd = 1 / idle).
This enables the use of an external inverter on the TX line.
This bitfield can only be written when the USART is disabled (UE = 0).
Bit 16 RXINV: RX pin active level inversion
This bit is set and cleared by software.
0: RX pin signal works using the standard logic levels (VDD = 1 / idle, Gnd = 0 / mark)
1: RX pin signal values are inverted. ((VDD = 0 / mark, Gnd = 1 / idle).
This enables the use of an external inverter on the RX line.
This bitfield can only be written when the USART is disabled (UE = 0).
Bit 15 SWAP: Swap TX/RX pins
This bit is set and cleared by software.
0: TX/RX pins are used as defined in standard pinout
1: The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired
connection to another UART.
This bitfield can only be written when the USART is disabled (UE = 0).
Bit 14 LINEN: LIN mode enable
This bit is set and cleared by software.
0: LIN mode disabled
1: LIN mode enabled
The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the
SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks.
This bitfield can only be written when the USART is disabled (UE = 0).
Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value.
Refer to Section 66.4: USART implementation.
Bits 13:12 STOP[1:0]: Stop bits
These bits are used for programming the stop bits.
00: 1 stop bit
01: 0.5 stop bit.
10: 2 stop bits
11: 1.5 stop bits
This bitfield can only be written when the USART is disabled (UE = 0).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bit 11 CLKEN: Clock enable
This bit enables the user to enable the CK pin.
0: CK pin disabled
1: CK pin enabled
Note: If neither synchronous mode nor smartcard mode is supported, this bit is reserved and must
be kept at reset value. Refer to Section 66.4: USART implementation.
In smartcard mode, in order to provide correctly the CK clock to the smartcard, the steps below
must be respected:
UE = 0
SCEN = 1
GTPR configuration
CLKEN= 1
UE = 1
Bit 10 CPOL: Clock polarity
This bit enables the user to select the polarity of the clock output on the CK pin in synchronous
mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship
0: Steady low value on CK pin outside transmission window
1: Steady high value on CK pin outside transmission window
This bit can only be written when the USART is disabled (UE = 0).
Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value.
Refer to Section 66.4: USART implementation.
Bit 9 CPHA: Clock phase
This bit is used to select the phase of the clock output on the CK pin in synchronous mode. It works
in conjunction with the CPOL bit to produce the desired clock/data relationship (see Figure 815 and
Figure 816)
0: The first clock transition is the first data capture edge
1: The second clock transition is the first data capture edge
This bit can only be written when the USART is disabled (UE = 0).
Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value.
Refer to Section 66.4: USART implementation.
Bit 8 LBCL: Last bit clock pulse
This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB)
has to be output on the CK pin in synchronous mode.
0: The clock pulse of the last data bit is not output to the CK pin
1: The clock pulse of the last data bit is output to the CK pin
Caution: The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit
format selected by the M bit in the USART_CR1 register.
This bit can only be written when the USART is disabled (UE = 0).
Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value.
Refer to Section 66.4: USART implementation.
Bit 7 Reserved, must be kept at reset value.
Bit 6 LBDIE: LIN break detection interrupt enable
Break interrupt mask (break detection using break delimiter).
0: Interrupt is inhibited
1: An interrupt is generated whenever LBDF = 1 in the USART_ISR register
Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to
Section 66.4: USART implementation.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bit 5 LBDL: LIN break detection length
This bit is for selection between 11 bit or 10 bit break detection.
0: 10-bit break detection
1: 11-bit break detection
This bit can only be written when the USART is disabled (UE = 0).
Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to
Section 66.4: USART implementation.
Bit 4 ADDM7: 7-bit address detection/4-bit address detection
This bit is for selection between 4-bit address detection or 7-bit address detection.
0: 4-bit address detection
1: 7-bit address detection (in 8-bit data mode)
This bit can only be written when the USART is disabled (UE = 0)
Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address
(ADD[5:0] and ADD[7:0]) respectively.
Bit 3 DIS_NSS:
When the DIS_NSS bit is set, the NSS pin input is ignored.
0: SPI slave selection depends on NSS input pin.
1: SPI slave is always selected and NSS input pin is ignored.
This bitfield can only be written when the USART is disabled (UE = 0).
Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value.
Refer to Section 66.4: USART implementation.
Bits 2:1 Reserved, must be kept at reset value.
Bit 0 SLVEN: Synchronous slave mode enable
When the SLVEN bit is set, the synchronous slave mode is enabled.
0: Slave mode disabled.
1: Slave mode enabled.
This bitfield can only be written when the USART is disabled (UE = 0).
Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value.
Refer to Section 66.4: USART implementation.

Note:

The CPOL, CPHA, and LBCL bits must not be written while the transmitter is enabled.

66.8.4

USART control register 3 (USART_CR3)
Address offset: 0x08
Reset value: 0x0000 0000
FIFO mode enabled, FIFOEN = 1

31

30

29

28

27

RXF
TIE

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

TCBG
TIE

TXFTIE

Res.

Res.

Res.
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

IRLP

IREN

EIE

rw

rw

rw

SCARCNT[2:0]

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

ONE
BIT

CTSIE

CTSE

RTSE

DMAT

DMAR

SCEN

NACK

HD
SEL

rw

rw

rw

rw

rw

rw

rw

rw

rw

DEP

DEM

DDRE

OVR
DIS

rw

rw

rw

rw

<!-- pagebreak -->

RM0456 Rev 6

17

16
Res.

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bits 31:29 TXFTCFG[2:0]: TXFIFO threshold configuration
000:TXFIFO reaches 1/8 of its depth
001:TXFIFO reaches 1/4 of its depth
010:TXFIFO reaches 1/2 of its depth
011:TXFIFO reaches 3/4 of its depth
100:TXFIFO reaches 7/8 of its depth
101:TXFIFO becomes empty
Others: Reserved, must not be used
This bitfield can only be written when the USART is disabled (UE = 0).
Bit 28 RXFTIE: RXFIFO threshold interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated when Receive FIFO reaches the threshold programmed in
RXFTCFG[2:0].
Bits 27:25 RXFTCFG[2:0]: Receive FIFO threshold configuration
000:Receive FIFO reaches 1/8 of its depth
001:Receive FIFO reaches 1/4 of its depth
010:Receive FIFO reaches 1/2 of its depth
011:Receive FIFO reaches 3/4 of its depth
100:Receive FIFO reaches 7/8 of its depth
101:Receive FIFO becomes full
Others: Reserved, must not be used
This bitfield can only be written when the USART is disabled (UE = 0).
Bit 24 TCBGTIE: Transmission Complete before guard time, interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated whenever TCBGT = 1 in the USART_ISR register
Note: If the USART does not support the smartcard mode, this bit is reserved and must be
kept at reset value. Refer to Section 66.4: USART implementation.
Bit 23 TXFTIE: TXFIFO threshold interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated when TXFIFO reaches the threshold programmed in
TXFTCFG[2:0].
Bits 22:20 Reserved, must be kept at reset value.
Bits 19:17 SCARCNT[2:0]: Smartcard auto-retry count
This bitfield specifies the number of retries for transmission and reception in smartcard
mode.
In transmission mode, it specifies the number of automatic retransmission retries, before
generating a transmission error (FE bit set).
In reception mode, it specifies the number or erroneous reception trials, before generating a
reception error (RXNE/RXFNE and PE bits set).
This bitfield must be programmed only when the USART is disabled (UE = 0).
When the USART is enabled (UE = 1), this bitfield may only be written to 0x0, in order to
stop retransmission.
0x0: retransmission disabled - No automatic retransmission in transmission mode.
0x1 to 0x7: number of automatic retransmission attempts (before signaling error)
Note: If smartcard mode is not supported, this bit is reserved and must be kept at reset
value. Refer to Section 66.4: USART implementation.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bit 16 Reserved, must be kept at reset value.
Bit 15 DEP: Driver enable polarity selection
0: DE signal is active high.
1: DE signal is active low.
This bit can only be written when the USART is disabled (UE = 0).
Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at
reset value. Refer to Section 66.4: USART implementation on page 2758.
Bit 14 DEM: Driver enable mode
This bit enables the user to activate the external transceiver control, through the DE signal.
0: DE function is disabled.
1: DE function is enabled. The DE signal is output on the RTS pin.
This bit can only be written when the USART is disabled (UE = 0).
Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at
reset value. Section 66.4: USART implementation on page 2758.
Bit 13 DDRE: DMA Disable on reception Error
0: DMA is not disabled in case of reception error. The corresponding error flag is set but
RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not
asserted, so the erroneous data is not transferred (no DMA request), but next correct
received data is transferred. (used for smartcard mode)
1: DMA is disabled following a reception error. The corresponding error flag is set, as well
as RXNE. The DMA request is masked until the error flag is cleared. This means that the
software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case
FIFO mode is enabled) before clearing the error flag.
This bit can only be written when the USART is disabled (UE = 0).
Note: The reception errors are: parity error, framing error or noise error.
Bit 12 OVRDIS: Overrun Disable
This bit is used to disable the receive overrun detection.
0: Overrun Error Flag, ORE, is set when received data is not read before receiving new
data.
1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set
the ORE flag is not set and the new received data overwrites the previous content of the
USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data are
written directly in USART_RDR register. Even when FIFO management is enabled, the
RXNE flag is to be used.
This bit can only be written when the USART is disabled (UE = 0).
Note: This control bit enables checking the communication flow w/o reading the data
Bit 11 ONEBIT: One sample bit method enable
This bit enables the user to select the sample method. When the one sample bit method is
selected the noise detection flag (NE) is disabled.
0: Three sample bit method
1: One sample bit method
This bit can only be written when the USART is disabled (UE = 0).
Bit 10 CTSIE: CTS interrupt enable
0: Interrupt is inhibited
1: An interrupt is generated whenever CTSIF = 1 in the USART_ISR register
Note: If the hardware flow control feature is not supported, this bit is reserved and must be
kept at reset value. Refer to Section 66.4: USART implementation.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bit 9 CTSE: CTS enable
0: CTS hardware flow control disabled
1: CTS mode enabled, data is only transmitted when the CTS input is deasserted (tied to 0).
If the CTS input is asserted while data is being transmitted, then the transmission completes
before stopping. If data is written into the data register while CTS is asserted, the
transmission is postponed until CTS is deasserted.
This bit can only be written when the USART is disabled (UE = 0)
Note: If the hardware flow control feature is not supported, this bit is reserved and must be
kept at reset value. Refer to Section 66.4: USART implementation.
Bit 8 RTSE: RTS enable
0: RTS hardware flow control disabled
1: RTS output enabled, data is only requested when there is space in the receive buffer. The
transmission of data is expected to cease after the current character has been transmitted.
The RTS output is deasserted (pulled to 0) when data can be received.
This bit can only be written when the USART is disabled (UE = 0).
Note: If the hardware flow control feature is not supported, this bit is reserved and must be
kept at reset value. Refer to Section 66.4: USART implementation.
Bit 7 DMAT: DMA enable transmitter
This bit is set/reset by software
1: DMA mode is enabled for transmission
0: DMA mode is disabled for transmission
Bit 6 DMAR: DMA enable receiver
This bit is set/reset by software
1: DMA mode is enabled for reception
0: DMA mode is disabled for reception
Bit 5 SCEN: Smartcard mode enable
This bit is used for enabling smartcard mode.
0: Smartcard mode disabled
1: Smartcard mode enabled
This bitfield can only be written when the USART is disabled (UE = 0).
Note: If the USART does not support smartcard mode, this bit is reserved and must be kept
at reset value. Refer to Section 66.4: USART implementation.
Bit 4 NACK: Smartcard NACK enable
0: NACK transmission in case of parity error is disabled
1: NACK transmission during parity error is enabled
This bitfield can only be written when the USART is disabled (UE = 0).
Note: If the USART does not support smartcard mode, this bit is reserved and must be kept
at reset value. Refer to Section 66.4: USART implementation.
Bit 3 HDSEL: Half-duplex selection
Selection of single-wire half-duplex mode
0: Half-duplex mode is not selected
1: Half-duplex mode is selected
This bit can only be written when the USART is disabled (UE = 0).

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bit 2 IRLP: IrDA low-power
This bit is used for selecting between normal and low-power IrDA modes
0: Normal mode
1: Low-power mode
This bit can only be written when the USART is disabled (UE = 0).
Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value.
Refer to Section 66.4: USART implementation.
Bit 1 IREN: IrDA mode enable
This bit is set and cleared by software.
0: IrDA disabled
1: IrDA enabled
This bit can only be written when the USART is disabled (UE = 0).
Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value.
Refer to Section 66.4: USART implementation.
Bit 0 EIE: Error interrupt enable
Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing
error, overrun error noise flag or SPI slave underrun error (FE = 1 or ORE = 1 or NE = 1 or
UDR = 1 in the USART_ISR register).
0: Interrupt inhibited
1: interrupt generated when FE = 1 or ORE = 1 or NE = 1 or UDR = 1 (in SPI slave mode) in
the USART_ISR register.

66.8.5

USART control register 3 [alternate] (USART_CR3)
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TCBG
TIE

Res.

Res.

Res.

Res.
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

7

6

5

4

3

2

1

0

ONE
BIT

CTSIE

CTSE

RTSE

DMAT

DMAR

SCEN

NACK

HD
SEL

IRLP

IREN

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

rw

rw

DEP

DEM

DDRE

OVR
DIS

rw

rw

rw

rw

8

19

18

17

SCARCNT[2:0]

16
Res.

Bits 31:25 Reserved, must be kept at reset value.
Bit 24 TCBGTIE: Transmission Complete before guard time, interrupt enable
This bit is set and cleared by software.
0: Interrupt inhibited
1: USART interrupt generated whenever TCBGT = 1 in the USART_ISR register
Note: If the USART does not support the smartcard mode, this bit is reserved and must be
kept at reset value. Refer to Section 66.4: USART implementation.
Bit 23 Reserved, must be kept at reset value.
Bits 22:20 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bits 19:17 SCARCNT[2:0]: Smartcard auto-retry count
This bitfield specifies the number of retries for transmission and reception in smartcard
mode.
In transmission mode, it specifies the number of automatic retransmission retries, before
generating a transmission error (FE bit set).
In reception mode, it specifies the number or erroneous reception trials, before generating a
reception error (RXNE/RXFNE and PE bits set).
This bitfield must be programmed only when the USART is disabled (UE = 0).
When the USART is enabled (UE = 1), this bitfield may only be written to 0x0, in order to
stop retransmission.
0x0: retransmission disabled - No automatic retransmission in transmission mode.
0x1 to 0x7: number of automatic retransmission attempts (before signaling error)
Note: If smartcard mode is not supported, this bit is reserved and must be kept at reset
value. Refer to Section 66.4: USART implementation.
Bit 16 Reserved, must be kept at reset value.
Bit 15 DEP: Driver enable polarity selection
0: DE signal is active high.
1: DE signal is active low.
This bit can only be written when the USART is disabled (UE = 0).
Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at
reset value. Refer to Section 66.4: USART implementation.
Bit 14 DEM: Driver enable mode
This bit enables the user to activate the external transceiver control, through the DE signal.
0: DE function is disabled.
1: DE function is enabled. The DE signal is output on the RTS pin.
This bit can only be written when the USART is disabled (UE = 0).
Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at
reset value. Section 66.4: USART implementation.
Bit 13 DDRE: DMA Disable on reception Error
0: DMA is not disabled in case of reception error. The corresponding error flag is set but
RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not
asserted, so the erroneous data is not transferred (no DMA request), but next correct
received data is transferred. (used for smartcard mode)
1: DMA is disabled following a reception error. The corresponding error flag is set, as well
as RXNE. The DMA request is masked until the error flag is cleared. This means that the
software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case
FIFO mode is enabled) before clearing the error flag.
This bit can only be written when the USART is disabled (UE = 0).
Note: The reception errors are: parity error, framing error or noise error.
Bit 12 OVRDIS: Overrun Disable
This bit is used to disable the receive overrun detection.
0: Overrun Error Flag, ORE, is set when received data is not read before receiving new
data.
1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set
the ORE flag is not set and the new received data overwrites the previous content of the
USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data are
written directly in USART_RDR register. Even when FIFO management is enabled, the
RXNE flag is to be used.
This bit can only be written when the USART is disabled (UE = 0).
Note: This control bit enables checking the communication flow w/o reading the data

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bit 11 ONEBIT: One sample bit method enable
This bit enables the user to select the sample method. When the one sample bit method is
selected the noise detection flag (NE) is disabled.
0: Three sample bit method
1: One sample bit method
This bit can only be written when the USART is disabled (UE = 0).
Bit 10 CTSIE: CTS interrupt enable
0: Interrupt is inhibited
1: An interrupt is generated whenever CTSIF = 1 in the USART_ISR register
Note: If the hardware flow control feature is not supported, this bit is reserved and must be
kept at reset value. Refer to Section 66.4: USART implementation.
Bit 9 CTSE: CTS enable
0: CTS hardware flow control disabled
1: CTS mode enabled, data is only transmitted when the CTS input is deasserted (tied to 0).
If the CTS input is asserted while data is being transmitted, then the transmission completes
before stopping. If data is written into the data register while CTS is asserted, the
transmission is postponed until CTS is deasserted.
This bit can only be written when the USART is disabled (UE = 0)
Note: If the hardware flow control feature is not supported, this bit is reserved and must be
kept at reset value. Refer to Section 66.4: USART implementation.
Bit 8 RTSE: RTS enable
0: RTS hardware flow control disabled
1: RTS output enabled, data is only requested when there is space in the receive buffer. The
transmission of data is expected to cease after the current character has been transmitted.
The RTS output is deasserted (pulled to 0) when data can be received.
This bit can only be written when the USART is disabled (UE = 0).
Note: If the hardware flow control feature is not supported, this bit is reserved and must be
kept at reset value. Refer to Section 66.4: USART implementation.
Bit 7 DMAT: DMA enable transmitter
This bit is set/reset by software
1: DMA mode is enabled for transmission
0: DMA mode is disabled for transmission
Bit 6 DMAR: DMA enable receiver
This bit is set/reset by software
1: DMA mode is enabled for reception
0: DMA mode is disabled for reception
Bit 5 SCEN: Smartcard mode enable
This bit is used for enabling smartcard mode.
0: Smartcard mode disabled
1: Smartcard mode enabled
This bitfield can only be written when the USART is disabled (UE = 0).
Note: If the USART does not support smartcard mode, this bit is reserved and must be kept
at reset value. Refer to Section 66.4: USART implementation.
Bit 4 NACK: Smartcard NACK enable
0: NACK transmission in case of parity error is disabled
1: NACK transmission during parity error is enabled
This bitfield can only be written when the USART is disabled (UE = 0).
Note: If the USART does not support smartcard mode, this bit is reserved and must be kept
at reset value. Refer to Section 66.4: USART implementation.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bit 3 HDSEL: Half-duplex selection
Selection of single-wire half-duplex mode
0: Half-duplex mode is not selected
1: Half-duplex mode is selected
This bit can only be written when the USART is disabled (UE = 0).
Bit 2 IRLP: IrDA low-power
This bit is used for selecting between normal and low-power IrDA modes
0: Normal mode
1: Low-power mode
This bit can only be written when the USART is disabled (UE = 0).
Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value.
Refer to Section 66.4: USART implementation.
Bit 1 IREN: IrDA mode enable
This bit is set and cleared by software.
0: IrDA disabled
1: IrDA enabled
This bit can only be written when the USART is disabled (UE = 0).
Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value.
Refer to Section 66.4: USART implementation.
Bit 0 EIE: Error interrupt enable
Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing
error, overrun error noise flag or SPI slave underrun error (FE = 1 or ORE = 1 or NE = 1or
UDR = 1 in the USART_ISR register).
0: Interrupt inhibited
1: interrupt generated when FE = 1 or ORE = 1 or NE = 1 or UDR = 1 (in SPI slave mode) in
the USART_ISR register.

66.8.6

USART baud rate register (USART_BRR)
This register can only be written when the USART is disabled (UE = 0). It may be
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

BRR[15:0]
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

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bits 15:0 BRR[15:0]: USART baud rate
BRR[15:4]
BRR[15:4] correspond to USARTDIV[15:4]
BRR[3:0]
When OVER8 = 0, BRR[3:0] = USARTDIV[3:0].
When OVER8 = 1:
BRR[2:0] = USARTDIV[3:0] shifted 1 bit to the right.
BRR[3] must be kept cleared.

66.8.7

USART guard time and prescaler register (USART_GTPR)
Address offset: 0x10
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

rw

rw

rw

rw

rw

rw

rw

GT[7:0]
rw

PSC[7:0]
rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:8 GT[7:0]: Guard time value
This bitfield is used to program the Guard time value in terms of number of baud clock
periods.
This is used in smartcard mode. The transmission complete flag is set after this guard time
value.
This bitfield can only be written when the USART is disabled (UE = 0).
Note: If smartcard mode is not supported, this bit is reserved and must be kept at reset value.
Refer to Section 66.4: USART implementation.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bits 7:0 PSC[7:0]: Prescaler value
Condition: IrDA low-power and normal IrDA mode
PSC[7:0] = IrDA normal and Low-power baud rate
This bitfield is used for programming the prescaler for dividing the USART source clock to
achieve the low-power frequency:
The source clock is divided by the value given in the register (8 significant bits):
00000000: Reserved - do not program this value
00000001: divides the source clock by 1
00000010: divides the source clock by 2
...
Condition: Smartcard mode
PSC[4:0]: Prescaler value
This bitfield is used for programming the prescaler for dividing the USART source clock to
provide the smartcard clock.
The value given in the register (5 significant bits) is multiplied by 2 to give the division factor
of the source clock frequency:
00000: Reserved - do not program this value
00001: divides the source clock by 2
00010: divides the source clock by 4
00011: divides the source clock by 6
...
This bitfield can only be written when the USART is disabled (UE = 0).
Note: Bits [7:5] must be kept cleared if smartcard mode is used.
This bitfield is reserved and forced by hardware to 0 when the smartcard and IrDA
modes are not supported. Refer to Section 66.4: USART implementation.

66.8.8

USART receiver timeout register (USART_RTOR)
Address offset: 0x14
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

BLEN[7:0]

20

19

18

17

16

RTO[23:16]

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

RTO[15:0]
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

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bits 31:24 BLEN[7:0]: Block Length
This bitfield gives the block length in smartcard T = 1 reception. Its value equals the number
of information characters + the length of the Epilogue Field (1 – LEC / 2 – CRC) – 1.
Examples:
BLEN = 0 -> 0 information characters + LEC
BLEN = 1 -> 0 information characters + CRC
BLEN = 255 -> 254 information characters + CRC (total 256 characters))
In smartcard mode, the block length counter is reset when TXE = 0 (TXFE = 0 in case FIFO
mode is enabled).
This bitfield can be used also in other modes. In this case, the block length counter is reset
when RE = 0 (receiver disabled) and/or when the EOBCF bit is written to 1.
Note: This value can be programmed after the start of the block reception (using the data
from the LEN character in the Prologue Field). It must be programmed only once per
received block.
Bits 23:0 RTO[23:0]: Receiver timeout value
This bitfield gives the Receiver timeout value in terms of number of bit duration.
In Standard mode, the RTOF flag is set if, after the last received character, no new start bit is
detected for more than the RTO value.
In smartcard mode, this value is used to implement the CWT and BWT. See smartcard
chapter for more details. In the standard, the CWT/BWT measurement is done starting from
the start bit of the last received character.
Note: This value must only be programmed once per received character.

Note:

RTOR can be written on-the-fly. If the new value is lower than or equal to the counter, the
RTOF flag is set.
This register is reserved and forced by hardware to “0x00000000” when the receiver timeout
feature is not supported. Refer to Section 66.4: USART implementation.

66.8.9

USART request register (USART_RQR)
Address offset: 0x18
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

TXFRQ RXFRQ MMRQ SBKRQ ABRRQ
w

Bits 31:5 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

w

w

w

w

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bit 4 TXFRQ: Transmit data flush request
When FIFO mode is disabled, writing 1 to this bit sets the TXE flag. This enables to discard
the transmit data. This bit must be used only in smartcard mode, when data have not been
sent due to errors (NACK) and the FE flag is active in the USART_ISR register. If the
USART does not support smartcard mode, this bit is reserved and must be kept at reset
value.
When FIFO is enabled, TXFRQ bit is set to flush the whole FIFO. This sets the TXFE flag
(Transmit FIFO empty, bit 23 in the USART_ISR register). Flushing the Transmit FIFO is
supported in both UART and smartcard modes.
Note: In FIFO mode, the TXFNF flag is reset during the flush request until TxFIFO is empty in
order to ensure that no data are written in the data register.
Bit 3 RXFRQ: Receive data flush request
Writing 1 to this bit empties the entire receive FIFO, that is clears the bit RXFNE.
This enables to discard the received data without reading them, and avoid an overrun
condition.
Bit 2 MMRQ: Mute mode request
Writing 1 to this bit puts the USART in mute mode and resets the RWU flag.
Bit 1 SBKRQ: Send break request
Writing 1 to this bit sets the SBKF flag and request to send a BREAK on the line, as soon as
the transmit machine is available.
Note: When the application needs to send the break character following all previously
inserted data, including the ones not yet transmitted, the software must wait for the
TXE flag assertion before setting the SBKRQ bit.
Bit 0 ABRRQ: Auto baud rate request
Writing 1 to this bit resets the ABRF and ABRE flags in the USART_ISR and requests an
automatic baud rate measurement on the next received data frame.
Note: If the USART does not support the auto baud rate feature, this bit is reserved and must
be kept at reset value. Refer to Section 66.4: USART implementation.

66.8.10

USART interrupt and status register (USART_ISR)
Address offset: 0x1C
Reset value: 0x0XX0 00C0
XX = 28 if FIFO/smartcard mode supported
XX = 08 if FIFO supported and smartcard mode not supported
The same register can be used in FIFO mode enabled (this section) and FIFO mode
disabled (next section).
FIFO mode enabled, FIFOEN = 1
S

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

TCBGT

RXFF

TXFE

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

ABRF

ABRE

UDR

EOBF

RTOF

CTS

CTSIF

LBDF

TXFNF

r

r

r

r

r

r

r

r

r

RM0456 Rev 6

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

r

r

5

4

3

2

1

0

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

REACK TEACK

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bits 31:28 Reserved, must be kept at reset value.
Bit 27 TXFT: TXFIFO threshold flag
This bit is set by hardware when the number of empty locations in the TXFIFO is greater
than the threshold programmed in the TXFTCFG[2:0] bitfield of USART_CR3 register.
An interrupt is generated if the TXFTIE bit (bit 31) is set in the USART_CR3 register.
0: TXFIFO does not reach the programmed threshold.
1: TXFIFO reached the programmed threshold.
Bit 26 RXFT: RXFIFO threshold flag
This bit is set by hardware when the threshold programmed in the RXFTCFG[2:0] bitfield of
the USART_CR3 register is reached. This means that there are (RXFTCFG[2:0] – 1) data in
the Receive FIFO and one data in the USART_RDR register. An interrupt is generated if the
RXFTIE bit = 1 (bit 27) in the USART_CR3 register.
0: Receive FIFO does not reach the programmed threshold.
1: Receive FIFO reached the programmed threshold.
Note: When the RXFTCFG[2:0] threshold is configured to 101, the RXFT flag is set if
RXFIFO size data are available, that is, (RXFIFO size – 1) data in the RXFIFO and 1
data in the USART_RDR. Consequently, the (RXFIFO size + 1) th received data does
not cause an overrun error. The overrun error occurs after receiving the (RXFIFO
size + 2) th data.
Bit 25 TCBGT: Transmission complete before guard time flag
This bit is set when the last data written in the USART_TDR has been transmitted correctly
out of the shift register.
It is set by hardware in smartcard mode, if the transmission of a frame containing data has
completed and if the smartcard did not send back any NACK. An interrupt is generated if
TCBGTIE = 1 in the USART_CR3 register.
This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or
by a write to the USART_TDR register.
0: Transmission has not completed, or transmission has completed unsuccessfully (that is, a
NACK is received from the card)
1: Transmission has completed successfully (before Guard time completion and there is no
NACK from the smart card).
Note: If the USART does not support the smartcard mode, this bit is reserved and kept at
reset value. If the USART supports the smartcard mode and the smartcard mode is
enabled, the TCBGT reset value is 1. Refer to Section 66.4: USART implementation.
Bit 24 RXFF: RXFIFO Full
This bit is set by hardware when the number of received data corresponds to
RXFIFO size + 1 (RXFIFO full + 1 data in the USART_RDR register.
An interrupt is generated if the RXFFIE bit = 1 in the USART_CR1 register.
0: RXFIFO not full.
1: RXFIFO Full.
Bit 23 TXFE: TXFIFO Empty
This bit is set by hardware when TXFIFO is Empty. When the TXFIFO contains at least one
data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4)
in the USART_RQR register.
An interrupt is generated if the TXFEIE bit = 1 (bit 30) in the USART_CR1 register.
0: TXFIFO not empty.
1: TXFIFO empty.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bit 22 REACK: Receive enable acknowledge flag
This bit is set/reset by hardware, when the Receive Enable value is taken into account by
the USART.
It can be used to verify that the USART is ready for reception before entering low-power
mode.
Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and
kept at reset value. Refer to Section 66.4: USART implementation.
Bit 21 TEACK: Transmit enable acknowledge flag
This bit is set/reset by hardware, when the Transmit Enable value is taken into account by
the USART.
It can be used when an idle frame request is generated by writing TE = 0, followed by
TE = 1 in the USART_CR1 register, in order to respect the TE = 0 minimum period.
Bit 20 Reserved, must be kept at reset value.
Bit 19 RWU: Receiver wake-up from mute mode
This bit indicates if the USART is in mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The mute mode control sequence (address or IDLE) is
selected by the WAKE bit in the USART_CR1 register.
When wake-up on idle mode is selected, this bit can only be set by software, writing 1 to the
MMRQ bit in the USART_RQR register.
0: Receiver in active mode
1: Receiver in mute mode
Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and
kept at reset value. Refer to Section 66.4: USART implementation.
Bit 18 SBKF: Send break flag
This bit indicates that a send break character was requested. It is set by software, by writing
1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during
the stop bit of break transmission.
0: No break character transmitted
1: Break character transmitted
Bit 17 CMF: Character match flag
This bit is set by hardware, when a the character defined by ADD[7:0] is received. It is
cleared by software, writing 1 to the CMCF in the USART_ICR register.
An interrupt is generated if CMIE = 1in the USART_CR1 register.
0: No Character match detected
1: Character match detected
Bit 16 BUSY: Busy flag
This bit is set and reset by hardware. It is active when a communication is ongoing on the
RX line (successful start bit detected). It is reset at the end of the reception (successful or
not).
0: USART is idle (no reception)
1: Reception ongoing
Bit 15 ABRF: Auto baud rate flag
This bit is set by hardware when the automatic baud rate has been set (RXFNE is also set,
generating an interrupt if RXFNEIE = 1) or when the auto baud rate operation has
completed without success (ABRE = 1) (ABRE, RXFNE and FE are also set in this case)
It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to
the ABRRQ in the USART_RQR register.
Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept
at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bit 14 ABRE: Auto baud rate error
This bit is set by hardware if the baud rate measurement failed (baud rate out of range or
character comparison failed)
It is cleared by software, by writing 1 to the ABRRQ bit in the USART_RQR register.
Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept
at reset value.
Bit 13 UDR: SPI slave underrun error flag
In slave transmission mode, this flag is set when the first clock pulse for data transmission
appears while the software has not yet loaded any value into USART_TDR. This flag is
reset by setting UDRCF bit in the USART_ICR register.
0: No underrun error
1: underrun error
Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at
reset value. Refer to Section 66.4: USART implementation.
Bit 12 EOBF: End of block flag
This bit is set by hardware when a complete block has been received (for example T = 1
smartcard mode). The detection is done when the number of received bytes (from the start
of the block, including the prologue) is equal or greater than BLEN + 4.
An interrupt is generated if EOBIE = 1 in the USART_CR1 register.
It is cleared by software, writing 1 to EOBCF in the USART_ICR register.
0: End of block not reached
1: End of block (number of characters) reached
Note: If smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to
Section 66.4: USART implementation.
Bit 11 RTOF: Receiver timeout
This bit is set by hardware when the timeout value, programmed in the RTOR register has
lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in
the USART_ICR register.
An interrupt is generated if RTOIE = 1 in the USART_CR2 register.
In smartcard mode, the timeout corresponds to the CWT or BWT timings.
0: Timeout value not reached
1: Timeout value reached without any data reception
Note: If a time equal to the value programmed in RTOR register separates 2 characters,
RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8,
depending on the oversampling method), RTOF flag is set.
The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has
already elapsed when RE is set, then RTOF is set.
If the USART does not support the Receiver timeout feature, this bit is reserved and
kept at reset value.
Bit 10 CTS: CTS flag
This bit is set/reset by hardware. It is an inverted copy of the status of the CTS input pin.
0: CTS line set
1: CTS line reset
Note: If the hardware flow control feature is not supported, this bit is reserved and kept at
reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bit 9 CTSIF: CTS interrupt flag
This bit is set by hardware when the CTS input toggles, if the CTSE bit is set. It is cleared by
software, by writing 1 to the CTSCF bit in the USART_ICR register.
An interrupt is generated if CTSIE = 1 in the USART_CR3 register.
0: No change occurred on the CTS status line
1: A change occurred on the CTS status line
Note: If the hardware flow control feature is not supported, this bit is reserved and kept at
reset value.
Bit 8 LBDF: LIN break detection flag
This bit is set by hardware when the LIN break is detected. It is cleared by software, by
writing 1 to the LBDCF in the USART_ICR.
An interrupt is generated if LBDIE = 1 in the USART_CR2 register.
0: LIN Break not detected
1: LIN break detected
Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value.
Refer to Section 66.4: USART implementation.
Bit 7 TXFNF: TXFIFO not full
TXFNF is set by hardware when TXFIFO is not full meaning that data can be written in the
USART_TDR. Every write operation to the USART_TDR places the data in the TXFIFO.
This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared
indicating that data can not be written into the USART_TDR.
An interrupt is generated if the TXFNFIE bit = 1 in the USART_CR1 register.
0: Transmit FIFO is full
1: Transmit FIFO is not full
Note: The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending
the flush request (by setting TXFRQ bit), the flag TXFNF must be checked prior to
writing in TXFIFO (TXFNF and TXFE is set at the same time).
This bit is used during single buffer transmission.
Bit 6 TC: Transmission complete
This bit indicates that the last data written in the USART_TDR has been transmitted out of
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
An interrupt is generated if TCIE = 1 in the USART_CR1 register.
The TC bit is cleared by software, by writing 1 to the TCCF of the USART_ICR register, or
by writing to the USART_TDR register.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bit 5 RXFNE: RXFIFO not empty
RXFNE bit is set by hardware when the RXFIFO is not empty, meaning that data can be
read from the USART_RDR register. Every read operation from the USART_RDR frees a
location in the RXFIFO.
RXFNE is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by
writing 1 to the RXFRQ in the USART_RQR register.
An interrupt is generated if RXFNEIE = 1 in the USART_CR1 register.
0: Data is not received
1: Received data is ready to be read.
Bit 4 IDLE: Idle line detected
This bit is set by hardware when an Idle Line is detected. An interrupt is generated if
IDLEIE = 1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in
the USART_ICR register.
0: No Idle line is detected
1: Idle line is detected
Note: The IDLE bit is not set again until the RXFNE bit has been set (that is, a new idle line
occurs).
If mute mode is enabled (MME = 1), IDLE is set if the USART is not mute (RWU = 0),
whatever the mute mode selected by the WAKE bit. If RWU = 1, IDLE is not set.
Bit 3 ORE: Overrun error
This bit is set by hardware when the data currently being received in the shift register is
ready to be transferred into the USART_RDR register while RXFF = 1. It is cleared by a
software, writing 1 to the ORECF, in the USART_ICR register.
An interrupt is generated if RXFNEIE = 1 in the USART_CR1 register, or EIE = 1 in the
USART_CR3 register.
0: No overrun error
1: Overrun error is detected
Note: When this bit is set, the USART_RDR register content is not lost but the shift register is
overwritten. An interrupt is generated if the ORE flag is set during multi buffer
communication if the EIE bit is set.
This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in
the USART_CR3 register.
Bit 2 NE: Noise detection flag
This bit is set by hardware when noise is detected on a received frame. It is cleared by
software, writing 1 to the NFCF bit in the USART_ICR register.
0: No noise is detected
1: Noise is detected
Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit
which itself generates an interrupt. An interrupt is generated when the NE flag is set
during multi buffer communication if the EIE bit is set.
When the line is noise-free, the NE flag can be disabled by programming the ONEBIT
bit to 1 to increase the USART tolerance to deviations (Refer to Section 66.5.9:
Tolerance of the USART receiver to clock deviation).
This error is associated with the character in the USART_RDR.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bit 1 FE: Framing error
This bit is set by hardware when a de-synchronization, excessive noise or a break character
is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register.
When transmitting data in smartcard mode, this bit is set when the maximum number of
transmit attempts is reached without success (the card NACKs the data frame).
An interrupt is generated if EIE = 1 in the USART_CR3 register.
0: No Framing error is detected
1: Framing error or break character is detected
Note: This error is associated with the character in the USART_RDR.
Bit 0 PE: Parity error
This bit is set by hardware when a parity error occurs in reception mode. It is cleared by
software, writing 1 to the PECF in the USART_ICR register.
An interrupt is generated if PEIE = 1 in the USART_CR1 register.
0: No parity error
1: Parity error
Note: This error is associated with the character in the USART_RDR.

66.8.11

USART interrupt and status register [alternate] (USART_ISR)
Address offset: 0x1C
Reset value: 0x0XX0 00C0
XX = 28 if FIFO/smartcard mode supported
XX = 08 if FIFO supported and smartcard mode not supported)
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

22

21

20

19

18

17

16

Res.

Res.

RE
ACK

TE
ACK

Res.

RWU

SBKF

CMF

BUSY

Res.

Res.

Res.

Res.

Res.

Res.

TCBGT

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

ABRF

ABRE

UDR

EOBF

RTOF

CTS

CTSIF

LBDF

TXE

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

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bits 31:26 Reserved, must be kept at reset value.
Bit 25 TCBGT: Transmission complete before guard time flag
This bit is set when the last data written in the USART_TDR has been transmitted correctly
out of the shift register.
It is set by hardware in smartcard mode, if the transmission of a frame containing data has
completed, and if the smartcard did not send back any NACK. An interrupt is generated if
TCBGTIE = 1 in the USART_CR3 register.
This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or
by a write to the USART_TDR register.
0: Transmission has not completed or transmission has completed unsuccessfully (that is, a
NACK is received from the card)
1: Transmission has not completed successfully (before Guard time completion and there is
no NACK from the smart card).
Note: If the USART does not support the smartcard mode, this bit is reserved and kept at
reset value. If the USART supports the smartcard mode and the smartcard mode is
enabled, the TCBGT reset value is 1. Refer to Section 66.4: USART implementation.
Bits 24:23 Reserved, must be kept at reset value.
Bit 22 REACK: Receive enable acknowledge flag
This bit is set/reset by hardware, when the Receive Enable value is taken into account by
the USART.
It can be used to verify that the USART is ready for reception before entering low-power
mode.
Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and
kept at reset value. Refer to Section 66.4: USART implementation.
Bit 21 TEACK: Transmit enable acknowledge flag
This bit is set/reset by hardware, when the Transmit Enable value is taken into account by
the USART.
It can be used when an idle frame request is generated by writing TE = 0, followed by
TE = 1 in the USART_CR1 register, in order to respect the TE = 0 minimum period.
Bit 20 Reserved, must be kept at reset value.
Bit 19 RWU: Receiver wake-up from mute mode
This bit indicates if the USART is in mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The mute mode control sequence (address or IDLE) is
selected by the WAKE bit in the USART_CR1 register.
When wake-up on idle mode is selected, this bit can only be set by software, writing 1 to the
MMRQ bit in the USART_RQR register.
0: Receiver in active mode
1: Receiver in mute mode
Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and
kept at reset value. Refer to Section 66.4: USART implementation.
Bit 18 SBKF: Send break flag
This bit indicates that a send break character was requested. It is set by software, by writing
1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during
the stop bit of break transmission.
0: No break character transmitted
1: Break character transmitted

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bit 17 CMF: Character match flag
This bit is set by hardware, when a the character defined by ADD[7:0] is received. It is
cleared by software, writing 1 to the CMCF in the USART_ICR register.
An interrupt is generated if CMIE = 1 in the USART_CR1 register.
0: No Character match detected
1: Character match detected
Bit 16 BUSY: Busy flag
This bit is set and reset by hardware. It is active when a communication is ongoing on the
RX line (successful start bit detected). It is reset at the end of the reception (successful or
not).
0: USART is idle (no reception)
1: Reception ongoing
Bit 15 ABRF: Auto baud rate flag
This bit is set by hardware when the automatic baud rate has been set (RXNE is also set,
generating an interrupt if RXNEIE = 1) or when the auto baud rate operation has completed
without success (ABRE = 1) (ABRE, RXNE and FE are also set in this case)
It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to
the ABRRQ in the USART_RQR register.
Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept
at reset value.
Bit 14 ABRE: Auto baud rate error
This bit is set by hardware if the baud rate measurement failed (baud rate out of range or
character comparison failed)
It is cleared by software, by writing 1 to the ABRRQ bit in the USART_RQR register.
Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept
at reset value.
Bit 13 UDR: SPI slave underrun error flag
In slave transmission mode, this flag is set when the first clock pulse for data transmission
appears while the software has not yet loaded any value into USART_TDR. This flag is
reset by setting UDRCF bit in the USART_ICR register.
0: No underrun error
1: underrun error
Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at
reset value. Refer to Section 66.4: USART implementation.
Bit 12 EOBF: End of block flag
This bit is set by hardware when a complete block has been received (for example T = 1
smartcard mode). The detection is done when the number of received bytes (from the start
of the block, including the prologue) is equal or greater than BLEN + 4.
An interrupt is generated if EOBIE = 1 in the USART_CR1 register.
It is cleared by software, writing 1 to EOBCF in the USART_ICR register.
0: End of block not reached
1: End of block (number of characters) reached
Note: If smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to
Section 66.4: USART implementation.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bit 11 RTOF: Receiver timeout
This bit is set by hardware when the timeout value, programmed in the RTOR register has
lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in
the USART_ICR register.
An interrupt is generated if RTOIE = 1 in the USART_CR2 register.
In smartcard mode, the timeout corresponds to the CWT or BWT timings.
0: Timeout value not reached
1: Timeout value reached without any data reception
Note: If a time equal to the value programmed in RTOR register separates 2 characters,
RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8,
depending on the oversampling method), RTOF flag is set.
The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has
already elapsed when RE is set, then RTOF is set.
If the USART does not support the Receiver timeout feature, this bit is reserved and
kept at reset value.
Bit 10 CTS: CTS flag
This bit is set/reset by hardware. It is an inverted copy of the status of the CTS input pin.
0: CTS line set
1: CTS line reset
Note: If the hardware flow control feature is not supported, this bit is reserved and kept at
reset value.
Bit 9 CTSIF: CTS interrupt flag
This bit is set by hardware when the CTS input toggles, if the CTSE bit is set. It is cleared by
software, by writing 1 to the CTSCF bit in the USART_ICR register.
An interrupt is generated if CTSIE = 1 in the USART_CR3 register.
0: No change occurred on the CTS status line
1: A change occurred on the CTS status line
Note: If the hardware flow control feature is not supported, this bit is reserved and kept at
reset value.
Bit 8 LBDF: LIN break detection flag
This bit is set by hardware when the LIN break is detected. It is cleared by software, by
writing 1 to the LBDCF in the USART_ICR.
An interrupt is generated if LBDIE = 1 in the USART_CR2 register.
0: LIN Break not detected
1: LIN break detected
Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value.
Refer to Section 66.4: USART implementation.
Bit 7 TXE: Transmit data register empty
TXE is set by hardware when the content of the USART_TDR register has been transferred
into the shift register. It is cleared by writing to the USART_TDR register. The TXE flag can
also be set by writing 1 to the TXFRQ in the USART_RQR register, in order to discard the
data (only in smartcard T = 0 mode, in case of transmission failure).
An interrupt is generated if the TXEIE bit = 1 in the USART_CR1 register.
0: Data register full
1: Data register empty

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bit 6 TC: Transmission complete
This bit indicates that the last data written in the USART_TDR has been transmitted out of
the shift register. The TC flag is set when the transmission of a frame containing data has
completed and when TXE is set.
An interrupt is generated if TCIE = 1 in the USART_CR1 register.
TC bit is cleared by software by writing 1 to the TCCF in the USART_ICR register or by
writing to the USART_TDR register.
Bit 5 RXNE: Read data register not empty
RXNE bit is set by hardware when the content of the USART_RDR shift register has been
transferred to the USART_RDR register. It is cleared by reading from the USART_RDR
register. The RXNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR
register.
An interrupt is generated if RXNEIE = 1 in the USART_CR1 register.
0: Data is not received
1: Received data is ready to be read.
Bit 4 IDLE: Idle line detected
This bit is set by hardware when an Idle Line is detected. An interrupt is generated if
IDLEIE = 1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in
the USART_ICR register.
0: No Idle line is detected
1: Idle line is detected
Note: The IDLE bit is not set again until the RXNE bit has been set (that is, a new idle line
occurs).
If mute mode is enabled (MME = 1), IDLE is set if the USART is not mute (RWU = 0),
whatever the mute mode selected by the WAKE bit. If RWU = 1, IDLE is not set.
Bit 3 ORE: Overrun error
This bit is set by hardware when the data currently being received in the shift register is
ready to be transferred into the USART_RDR register while RXNE = 1. It is cleared by a
software, writing 1 to the ORECF, in the USART_ICR register.
An interrupt is generated if RXNEIE = 1 in the USART_CR1 register, or EIE = 1 in the
USART_CR3 register.
1: Overrun error is detected
Note: When this bit is set, the USART_RDR register content is not lost but the shift register is
overwritten. An interrupt is generated if the ORE flag is set during multi buffer
communication if the EIE bit is set.
This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in
the USART_CR3 register.
Bit 2 NE: Noise detection flag
This bit is set by hardware when noise is detected on a received frame. It is cleared by
software, writing 1 to the NFCF bit in the USART_ICR register.
0: No noise is detected
1: Noise is detected
Note: This bit does not generate an interrupt as it appears at the same time as the RXNE bit
which itself generates an interrupt. An interrupt is generated when the NE flag is set
during multi buffer communication if the EIE bit is set.
When the line is noise-free, the NE flag can be disabled by programming the ONEBIT
bit to 1 to increase the USART tolerance to deviations (Refer to Section 66.5.9:
Tolerance of the USART receiver to clock deviation).
This error is associated with the character in the USART_RDR.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bit 1 FE: Framing error
This bit is set by hardware when a de-synchronization, excessive noise or a break character
is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register.
When transmitting data in smartcard mode, this bit is set when the maximum number of
transmit attempts is reached without success (the card NACKs the data frame).
An interrupt is generated if EIE = 1 in the USART_CR3 register.
0: No Framing error is detected
1: Framing error or break character is detected
Note: This error is associated with the character in the USART_RDR.
Bit 0 PE: Parity error
This bit is set by hardware when a parity error occurs in reception mode. It is cleared by
software, writing 1 to the PECF in the USART_ICR register.
An interrupt is generated if PEIE = 1 in the USART_CR1 register.
0: No parity error
1: Parity error
Note: This error is associated with the character in the USART_RDR.

66.8.12

USART interrupt flag clear register (USART_ICR)
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

TCBGT
CF

TCCF

NECF

FECF

PECF

w

w

w

w

w

w

UDRCF EOBCF RTOCF
w

w

w

Res.

CTSCF LBDCF
w

w

TXFEC
IDLECF ORECF
F
w

w

w

Bits 31:18 Reserved, must be kept at reset value.
Bit 17 CMCF: Character match clear flag
Writing 1 to this bit clears the CMF flag in the USART_ISR register.
Bits 16:14 Reserved, must be kept at reset value.
Bit 13 UDRCF:SPI slave underrun clear flag
Writing 1 to this bit clears the UDRF flag in the USART_ISR register.
Note: If the USART does not support SPI slave mode, this bit is reserved and must be kept at
reset value. Refer to Section 66.4: USART implementation
Bit 12 EOBCF: End of block clear flag
Writing 1 to this bit clears the EOBF flag in the USART_ISR register.
Note: If the USART does not support smartcard mode, this bit is reserved and must be kept
at reset value. Refer to Section 66.4: USART implementation.
Bit 11 RTOCF: Receiver timeout clear flag
Writing 1 to this bit clears the RTOF flag in the USART_ISR register.
Note: If the USART does not support the Receiver timeout feature, this bit is reserved and
must be kept at reset value. Refer to Section 66.4: USART implementation.
Bit 10 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Bit 9 CTSCF: CTS clear flag
Writing 1 to this bit clears the CTSIF flag in the USART_ISR register.
Note: If the hardware flow control feature is not supported, this bit is reserved and must be
kept at reset value. Refer to Section 66.4: USART implementation.
Bit 8 LBDCF: LIN break detection clear flag
Writing 1 to this bit clears the LBDF flag in the USART_ISR register.
Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer
to Section 66.4: USART implementation.
Bit 7 TCBGTCF: Transmission complete before Guard time clear flag
Writing 1 to this bit clears the TCBGT flag in the USART_ISR register.
Bit 6 TCCF: Transmission complete clear flag
Writing 1 to this bit clears the TC flag in the USART_ISR register.
Bit 5 TXFECF: TXFIFO empty clear flag
Writing 1 to this bit clears the TXFE flag in the USART_ISR register.
Bit 4 IDLECF: Idle line detected clear flag
Writing 1 to this bit clears the IDLE flag in the USART_ISR register.
Bit 3 ORECF: Overrun error clear flag
Writing 1 to this bit clears the ORE flag in the USART_ISR register.
Bit 2 NECF: Noise detected clear flag
Writing 1 to this bit clears the NE flag in the USART_ISR register.
Bit 1 FECF: Framing error clear flag
Writing 1 to this bit clears the FE flag in the USART_ISR register.
Bit 0 PECF: Parity error clear flag
Writing 1 to this bit clears the PE flag in the USART_ISR register.

66.8.13

USART receive data register (USART_RDR)
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

9

Res.

Res.

Res.

Res.

Res.

Res.

Res.

RDR[8:0]
r

r

r

r

r

Bits 31:9 Reserved, must be kept at reset value.
Bits 8:0 RDR[8:0]: Receive data value
Contains the received data character.
The RDR register provides the parallel interface between the input shift register and the
internal bus (see Section 66.5.1: USART block diagram).
When receiving with the parity enabled, the value read in the MSB bit is the received parity
bit.

RM0456 Rev 6

<!-- pagebreak -->

