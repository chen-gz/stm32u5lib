RM0456 Rev 6

r

r

RM0456

Inter-integrated circuit interface (I2C)

65.9.11

I2C transmit data register (I2C_TXDR)
Address offset: 0x28
Reset value: 0x0000 0000
Access: no wait states

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

18

17

16

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

TXDATA[7:0]
rw

rw

rw

rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 TXDATA[7:0]: 8-bit transmit data
Data byte to be transmitted to the I²C-bus
Note: These bits can be written only when TXE = 1.

65.9.12

I2C autonomous mode control register (I2C_AUTOCR)
Address offset: 0x2C
Reset value: 0x0000 0000
Access: no wait states

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

TRIGE
N

TRIGP
OL

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

TCRD
MAEN

TCDM
AEN

Res.

Res.

Res.

Res.

Res.

Res.

rw

rw

Res.

Res.

Res.

Res.

Res.

Res.

Res.

19

TRIGSEL[3:0]

Bits 31:22 Reserved, must be kept at reset value.
Bit 21 TRIGEN: Trigger enable
0: Trigger disabled
1: Trigger enabled
When a trigger is detected, a START condition is sent and the transfer is launched as
defined in I2C_CR2.
Bit 20 TRIGPOL: Trigger polarity
0: Trigger active on rising edge
1: Trigger active on falling edge
Note: This bit can be written only when PE = 0

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

Bits 19:16 TRIGSEL[3:0]: Trigger selection (refer to Section 65.4.2: I2C pins and internal signals I2C
interconnections tables).
0000: i2c_trg0 selected
0001: i2c_trg1 selected
...
1111: i2c_trg15 selected
Note: This bit can be written only when PE = 0
Bits 15:8 Reserved, must be kept at reset value.
Bit 7 TCRDMAEN: DMA request enable on Transfer Complete Reload event
0: DMA request not generated on Transfer Complete Reload event
1: DMA request generated on Transfer Complete Reload event
Bit 6 TCDMAEN: DMA request enable on Transfer Complete event
0: DMA request not generated on Transfer Complete event
1: DMA request generated on Transfer Complete event
Bits 5:0 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

0x28

I2C_TXDR

Reset value

RM0456 Rev 6
BERR
TCR
TC
STOPF
NACKF
ADDR
RXNE
TXIS
TXE

0
0
1

Res.
Res.
STOPCF
NACKCF
ADDRCF

0
0
0

BERRCF

0

0
0
0
0
0
0

Res.

0
0
0
0

Res.
Res.
Res.
Res.
OA1MODE

14
13
12

TXDMAEN
Res.
ANFOFF
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

Reset value

Reset value
7
6
5
4
3
2
1
0

TCIE
STOPIE
NACKIE
ADDRIE
RXIE
TXIE
PE

8

9

10

ERRIE

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

SCLH[7:0]

0

0

0

0

0

OA1[9:0]

0
0

OA2[7:1]
0
0

0

0

0

0

0

SCLL[7:0]
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

Res.

15
RXDMAEN

11

16
SBC

0

PEC[7:0]

RXDATA[7:0]

TXDATA[7:0]
Res.

0

OA1EN

17
NOSTRETCH

0

Res.

0

Res.

18
WUPEN

0

DNF[3:0]

Res.

RD_WRN

0

Res.

19
GCEN

0

Res.

ADD10

0

Res.

20
SMBHEN

0

Res.

OVR
ARLO

0

OVRCF

0

ARLOCF

0

Res.
0

Res.

HEAD10R

0

Res.

21
SMBDEN

0

Res.

START

0

Res.

22

0

Res.

STOP

0

Res.

23
ALERTEN
0

Res.

Res.

NACK

0

Res.

24
PECEN
0

Res.

0

TIDLE

Res.

Res.

0

Res.

0

Res.

PECERR

0

PECCF
0

Res.

TIMEOUT

0

TIMOUTCF
0

Res.

0

Res.

OA2EN

RELOAD

0

Res.

0

Res.

Res.

0

Res.

AUTOEND

0

Res.

25

26
Res.

PECBYTE

0

Res.

Res.

27

Res.

Res.

FMP

28

Res.

Res.

0

Res.

Res.
0

ALERTCF
0

Res.

ALERT

Reset value

Res.

0

TIMOUTEN

SDADEL
[3:0]

Res.

Res.

BUSY
0

Res.

0

Res.

0

Res.

Res.

TIMEOUTB[11:0]

Res.

0

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

29

Res.

Res.

0
NBYTES[7:0]

Res.

DIR

0

Res.

ADDCODE[6:0]

Res.

0

Res.

0

Res.
0

Res.

Res.

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.
0

Res.

0

Res.

Res.

Res.

SCLDEL
[3:0]

Res.

Res.

Res.
0

Res.

Res.

0
0

Res.

0

Res.
0

Res.

Res.

0

Res.

0

Res.
0

Res.

Res.

0

Res.

Res.
0

Res.

Res.
0

Res.

Reset value

Res.

0

Res.

Res.

Res.
0

Res.

Res.

OA2MS
K [2:0]

Res.

Res.

30

Res.

I2C_CR2
Res.

ADDRACLR

0

Res.

I2C_OAR2
Res.

Res.

31

STOPFACLR

0

Res.

Res.

0

Res.

Res.

Res.

Reset value

Res.

0

Res.

0

Res.

I2C_CR1

Res.

0

Res.

Res.

Res.

Reset value

Res.

Register name

Res.

I2C_ISR

Res.

Res.

0

Res.

Res.

Reset value

Res.

Res.

Reset value

Res.

I2C_
TIMEOUTR
Res.

0

Res.

Res.

0

Res.

Res.

Res.

0

Res.

Res.

Res.

0

Res.

Reset value

Res.

I2C_RXDR

Res.

0x24

I2C_PECR

Res.

0x20
I2C_ICR

Res.

0x1C
PRESC[3:0]

TEXTEN

0x18
I2C_
TIMINGR

Res.

0x14

Res.

0x10

Res.

0x0C
I2C_OAR1

Res.

0x08

Res.

0x04

Res.

0x00

Res.

Offset

Res.

65.9.13

Res.

RM0456
Inter-integrated circuit interface (I2C)

I2C register map

The table below provides the I2C register map and the reset values.
Table 672. I2C register map and reset values

0
0
0
0
0
0
0
0

SADD[9:0]

TIMEOUTA[11:0]

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

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

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

7

9

Res.

12

11

13

Res.

10

14

Res.

Res.

15

TCDMAEN

0

8

0

TCRDMAEN

0

Res.

0

Res.

TRIGSEL
[3:0]

Res.

16

17

20
TRIGPOL
0

18

21
TRIGEN
0

19

22

24

25

23
Res.

Res.

Res.

26
Res.

28

29

27
Res.

Res.

30

Res.

Reset value

Res.

I2C_AUTOCR

Res.

0x2C

Register name

Res.

Offset

31

Table 672. I2C register map and reset values (continued)

0

0

Refer to Section 2.3: Memory organization for the register boundary addresses.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

66

Universal synchronous/asynchronous receiver
transmitter (USART/UART)

66.1

Introduction
The USART offers a flexible means to perform full-duplex data exchange with external
equipments requiring an industry standard NRZ asynchronous serial data format. A very
wide range of baud rates can be achieved through a fractional baud rate generator.
The USART supports both synchronous one-way and half-duplex single-wire
communications, as well as LIN (local interconnection network), smartcard protocol, IrDA
(infrared data association) SIR ENDEC specifications, and modem operations (CTS/RTS).
Multiprocessor communications are also supported.
High-speed data communications are possible by using the DMA (direct memory access)
for multibuffer configuration.

66.2

USART main features
•

Full-duplex asynchronous communication

•

NRZ standard format (mark/space)

•

Configurable oversampling method by 16 or 8 to achieve the best compromise
between speed and clock tolerance

•

Baud rate generator systems

•

Two internal FIFOs for transmit and receive data
Each FIFO can be enabled/disabled by software and come with a status flag.

•

A common programmable transmit and receive baud rate

•

Dual clock domain with dedicated kernel clock for peripherals independent from PCLK

•

Auto baud rate detection

•

Programmable data word length (7, 8 or 9 bits)

•

Programmable data order with MSB-first or LSB-first shifting

•

Configurable stop bits (1 or 2 stop bits)

•

Synchronous master/slave mode and clock output/input for synchronous
communications

•

SPI slave transmission underrun error flag

•

Single-wire half-duplex communications

•

Continuous communications using DMA

•

Received/transmitted bytes are buffered in reserved SRAM using centralized DMA.

•

Separate enable bits for transmitter and receiver

•

Separate signal polarity control for transmission and reception

•

Swappable Tx/Rx pin configuration

•

Hardware flow control for modem and RS-485 transceiver

•

Communication control/error detection flags

•

Parity control:
–

Transmits parity bit

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)
–

66.3

Checks parity of received data byte

•

Interrupt sources with flags

•

Multiprocessor communications: wake-up from mute mode by idle line detection or
address mark detection

•

Wake-up from Stop mode

•

Autonomous functionality in Stop mode

USART extended features
•

LIN master synchronous break send capability and LIN slave break detection capability
–

13-bit break generation and 10/11 bit break detection when USART is hardware
configured for LIN

•

IrDA SIR encoder decoder supporting 3/16 bit duration for normal mode

•

Smartcard mode

•

66.4

RM0456

–

Supports the T = 0 and T = 1 asynchronous protocols for smartcards as defined in
the ISO/IEC 7816-3 standard

–

0.5 and 1.5 stop bits for smartcard operation

Support for Modbus communication
–

Timeout feature

–

CR/LF character recognition

USART implementation
The following tables describe the USART implementation. LPUART is included for
comparison.
Table 673. Instance implementation on STM32U5 series
Instance

STM32U535/545

STM32U575/585

STM32U59x/5Ax/5Fx/5Gx

USART1

Full

Full

Full

USART2

-

Full

Full

USART3

Full

Full

Full

USART6

-

-

Full

UART4

Basic

Basic

Basic

UART5

Basic

Basic

Basic

LPUART1

Low-power

Low-power

Low-power

Table 674. USART/LPUART features
Modes/features(1)

Full feature

Basic feature

Low-power feature

Hardware flow control for modem

X

X

X

Continuous communication using DMA

X

X

X

Multiprocessor communication

X

X

X

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)
Table 674. USART/LPUART features (continued)
Modes/features(1)

Full feature

Basic feature

Low-power feature

Synchronous mode (master/slave)

X

-

-

Smartcard mode

X

-

-

Single-wire half-duplex communication

X

X

X

IrDA SIR ENDEC block

X

X

-

LIN mode

X

X

-

Dual-clock domain

X

X

X

Receiver timeout interrupt

X

X

-

Modbus communication

X

X

-

Auto baud rate detection

X

X

-

Driver enable

X

X

X

USART data length

7, 8 and 9 bits

Tx/Rx FIFO

X

Tx/Rx FIFO size

X

X

8 bytes

Wake-up from low-power mode

X(2)

X(2)

X(3)

Autonomous mode

X

X

X

1. “X” = supported, “-” = not supported.
2. Wake-up supported from Stop 0 and Stop 1 modes.
3. Wake-up supported from Stop 0, Stop 1, and Stop 2 modes.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

66.5

USART functional description

66.5.1

USART block diagram

RM0456

Figure 809. USART block diagram
USART
usart_ker_ck clock domain
usart_wkup
usart_it
usart_tx_dma
usart_rx_dma

IRQ Interface

usart_pclk
clock domain

usart_trg[15:0]

DMA Interface
COM Controller
USART_CR1
USART_ISR
USART_CR2

CK

Hardware
flow control

TxFIFO

32-bit APB bus

USART_CR3
USART_RQR
USART_ICR

RxFIFO

USART_TDR

USART_RDR
USART_
RTOR

RTS/DE

TX Shift Reg
...

TX

RX Shift Reg
...

RX

Baudrate
generator &
orversampling

USART_GTPR
USART_BRR
usart_pclk
usart_ker_ck

CTS/NSS

USART_
PRESC

usart_ker_ck_pres

MSv40854V5

66.5.2

USART pins and internal signals
Description of USART input/output pins
•

USART bidirectional communications
USART bidirectional communications require a minimum of two pins: Receive Data In
(RX) and Transmit Data Out (TX):
–

RX (Receive Data Input)
RX is the serial data input. Oversampling techniques are used for data recovery.
They discriminate between valid incoming data and noise.

–

TX (Transmit Data Output)
When the transmitter is disabled, the output pin returns to its I/O port
configuration. When the transmitter is enabled and no data needs to be
transmitted, the TX pin is High. In single-wire and smartcard modes, this I/O is
used to transmit and receive data.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)
•

RS232 hardware flow control mode
The following pins are required in RS232 hardware flow control mode:
–

CTS (Clear To Send)
When driven high, this signal blocks the data transmission at the end of the
current transfer.

–

RTS (Request To Send)
When it is low, this signal indicates that the USART is ready to receive data.

•

RS485 hardware control mode
The DE (Driver Enable) pin is required in RS485 hardware control mode. This signal
activates the transmission mode of the external transceiver.

•

Synchronous SPI master/slave mode and smartcard mode
The following pins are required in synchronous master/slave mode and smartcard
mode:
–

CK
This pin acts as clock output in synchronous SPI master and smartcard modes.
It acts as clock input in synchronous SPI slave mode.
In synchronous master mode, this pin outputs the transmitter data clock for
synchronous transmission corresponding to SPI master mode (no clock pulses on
start bit and stop bit, and a software option to send a clock pulse on the last data
bit). In parallel, data can be received synchronously on RX pin. This mechanism
can be used to control peripherals featuring shift registers (such as LCD drivers).
The clock phase and polarity are software programmable.
In smartcard mode, CK output provides the clock to the smartcard.

–

NSS
This pin acts as Slave Select input in synchronous slave mode.

Refer to Table 675 and Table 676 for the list of USART input/output pins and internal
signals.
Table 675. USART/UART input/output pins
Pin name

Signal type

USART_RX/UART_RX

Input

Serial data receive input

USART_TX/UART_TX

Output

Transmit data output

USART_CTS/UART_CTS

Input

Clear to send

USART_RTS/UART_RTS

Output

Request to send

(1)

USART_DE /UART_DE

Output

Driver enable

USART_CK

Output

Clock output in synchronous master and smartcard
modes.

USART_NSS(3)

Input

Slave select input in synchronous slave mode.

(2)

Description

1. USART_DE and USART_RTS share the same pin.
2. UART_DE and UART_RTS share the same pin.
3. USART_NSS and USART_CTS share the same pin.

RM0456 Rev 6

<!-- pagebreak -->

