RM0456 Rev 6

RM0456

66.5.9

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Tolerance of the USART receiver to clock deviation
The USART asynchronous receiver operates correctly only if the total clock system
deviation is less than the tolerance of the USART receiver.
The causes which contribute to the total deviation are:
•

DTRA: deviation due to the transmitter error (which also includes the deviation of the
transmitter’s local oscillator)

•

DQUANT: error due to the baud rate quantization of the receiver

•

DREC: deviation of the receiver local oscillator

•

DTCL: deviation due to the transmission line (generally due to the transceivers which
can introduce an asymmetry between the low-to-high transition timing and the high-tolow transition timing)
DTRA + DQUANT + DREC + DTCL + DWU < USART receiver tolerance

where
DWU is the error due to sampling point deviation when the wake-up from lowpower mode is used.
when M[1:0] = 01:
t WUUSART
DWU = -------------------------11 × Tbit

when M[1:0] = 00:
t WUUSART
DWU = -------------------------10 × Tbit

when M[1:0] = 10:
t WUUSART
DWU = -------------------------9 × Tbit

tWUUSART is the time between the detection of the start bit falling edge and the
instant when the clock (requested by the peripheral) is ready and reaching the
peripheral, and the regulator is ready.
The USART receiver can receive data correctly at up to the maximum tolerated deviation
specified in Table 679, Table 680, depending on the following settings:
•

9-, 10- or 11-bit character length defined by the M bits in the USART_CR1 register

•

Oversampling by 8 or 16 defined by the OVER8 bit in the USART_CR1 register

•

Bits BRR[3:0] of USART_BRR register are equal to or different from 0000.

•

Use of 1 bit or 3 bits to sample the data, depending on the value of the ONEBIT bit in
the USART_CR3 register.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Table 679. Tolerance of the USART receiver when BRR [3:0] = 0000
OVER8 bit = 0

OVER8 bit = 1

M bits
ONEBIT = 0

ONEBIT = 1

ONEBIT = 0

ONEBIT = 1

00

3.75%

4.375%

2.50%

3.75%

01

3.41%

3.97%

2.27%

3.41%

10

4.16%

4.86%

2.77%

4.16%

Table 680. Tolerance of the USART receiver when BRR[3:0] is different from 0000
OVER8 bit = 0

OVER8 bit = 1

M bits
ONEBIT = 0

ONEBIT = 1

ONEBIT = 0

ONEBIT = 1

00

3.33%

3.88%

2%

3%

01

3.03%

3.53%

1.82%

2.73%

10

3.7%

4.31%

2.22%

3.33%

Note:

The data specified in Table 679 and Table 680 may slightly differ in the special case when
the received frames contain some Idle frames of exactly 10-bit times when M bits = 00 (11bit times when M= 01 or 9- bit times when M = 10).

66.5.10

USART auto baud rate detection
The USART can detect and automatically set the USART_BRR register value based on the
reception of one character. Automatic baud rate detection is useful under two
circumstances:
•

The communication speed of the system is not known in advance.

•

The system is using a relatively low accuracy clock source and this mechanism
enables the correct baud rate to be obtained without measuring the clock deviation.

The clock source frequency must be compatible with the expected communication speed.
•

When oversampling by 16, the baud rate ranges from usart_ker_ck_pres/65535 and
usart_ker_ck_pres/16.

•

When oversampling by 8, the baud rate ranges from usart_ker_ck_pres/32763 and
usart_ker_ck_pres/8.

Before activating the auto baud rate detection, the auto baud rate detection mode must be
selected through the ABRMOD[1:0] field in the USART_CR2 register. There are four modes
based on different character patterns. In these auto baud rate modes, the baud rate is
measured several times during the synchronization data reception and each measurement
is compared to the previous one.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)
These modes are the following:
•

Mode 0: Any character starting with a bit at 1.
In this case the USART measures the duration of the start bit (falling edge to rising
edge).

•

Mode 1: Any character starting with a 10xx bit pattern.
In this case, the USART measures the duration of the Start and of the 1st data bit. The
measurement is done falling edge to falling edge, to ensure a better accuracy in the
case of slow signal slopes.

•

Mode 2: A 0x7F character frame (it may be a 0x7F character in LSB first mode or a
0xFE in MSB first mode).
In this case, the baud rate is updated first at the end of the start bit (BRs), then at the
end of bit 6 (based on the measurement done from falling edge to falling edge: BR6).
Bit0 to bit6 are sampled at BRs while further bits of the character are sampled at BR6.

•

Mode 3: A 0x55 character frame.
In this case, the baud rate is updated first at the end of the start bit (BRs), then at the
end of bit0 (based on the measurement done from falling edge to falling edge: BR0),
and finally at the end of bit6 (BR6). Bit 0 is sampled at BRs, bit 1 to bit 6 are sampled at
BR0, and further bits of the character are sampled at BR6. In parallel, another check is
performed for each intermediate RX line transition. An error is generated if the
transitions on RX are not sufficiently synchronized with the receiver (the receiver being
based on the baud rate calculated on bit 0).

Prior to activating the auto baud rate detection, the USART_BRR register must be initialized
by writing a non-zero baud rate value.
The automatic baud rate detection is activated by setting the ABREN bit in the USART_CR2
register. The USART then waits for the first character on the RX line. The auto baud rate
operation completion is indicated by the setting of the ABRF flag in the USART_ISR
register. If the line is noisy, the correct baud rate detection cannot be guaranteed. In this
case the BRR value may be corrupted and the ABRE error flag is set. This also happens if
the communication speed is not compatible with the automatic baud rate detection range
(bit duration not between 16 and 65536 clock periods (oversampling by 16) and not between
8 and 65536 clock periods (oversampling by 8)).
The auto baud rate detection can be re-launched later by resetting the ABRF flag (by writing
a 0).
When FIFO management is disabled and an auto baud rate error occurs, the ABRE flag is
set through RXNE and FE bits.
When FIFO management is enabled and an auto baud rate error occurs, the ABRE flag is
set through RXFNE and FE bits.
If the FIFO mode is enabled, the auto baud rate detection must be made using the data on
the first RXFIFO location. So, prior to launching the auto baud rate detection, make sure
that the RXFIFO is empty by checking the RXFNE flag in USART_ISR register.
Note:

The BRR value might be corrupted if the USART is disabled (UE = 0) during an auto baud
rate operation.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

66.5.11

RM0456

USART multiprocessor communication
It is possible to perform USART multiprocessor communications (with several USARTs
connected in a network). For instance one of the USARTs can be the master with its TX
output connected to the RX inputs of the other USARTs, while the others are slaves with
their respective TX outputs logically ANDed together and connected to the RX input of the
master.
In multiprocessor configurations, it is often desirable that only the intended message
recipient actively receives the full message contents, thus reducing redundant USART
service overhead for all non addressed receivers.
The non-addressed devices can be placed in mute mode by means of the muting function.
To use the mute mode feature, the MME bit must be set in the USART_CR1 register.

Note:

When FIFO management is enabled and MME is already set, MME bit must not be cleared
and then set again quickly (within two usart_ker_ck cycles), otherwise mute mode might
remain active.
When the mute mode is enabled:
•

none of the reception status bits can be set;

•

all the receive interrupts are inhibited;

•

the RWU bit in USART_ISR register is set to 1. RWU can be controlled automatically
by hardware or by software, through the MMRQ bit in the USART_RQR register, under
certain conditions.

The USART can enter or exit from mute mode using one of two methods, depending on the
WAKE bit in the USART_CR1 register:
•

Idle line detection if the WAKE bit is reset,

•

Address mark detection if the WAKE bit is set.

Idle line detection (WAKE = 0)
The USART enters mute mode when the MMRQ bit is written to 1 and the RWU is
automatically set.
The USART wakes up when an Idle frame is detected. The RWU bit is then cleared by
hardware but the IDLE bit is not set in the USART_ISR register. An example of mute mode
behavior using Idle line detection is given in Figure 817.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)
Figure 817. Mute mode using Idle line detection

RXNE

Data 1 Data 2 Data 3 Data 4

RX

RWU
MMRQ written to 1

IDLE

Mute mode

RXNE

Data 5 Data 6

Normal mode
Idle frame detected

MSv31154V1

Note:

If the MMRQ is set while the IDLE character has already elapsed, mute mode is not entered
(RWU is not set).
If the USART is activated while the line is idle, the idle state is detected after the duration of
one IDLE frame (not only after the reception of one character frame).

4-bit/7-bit address mark detection (WAKE = 1)
In this mode, bytes are recognized as addresses if their MSB is a 1, otherwise they are
considered as data. In an address byte, the address of the targeted receiver is put in the 4
or 7 LSBs. The choice of 7 or 4 bit address detection is done using the ADDM7 bit. This 4bit/7-bit word is compared by the receiver with its own address which is programmed in the
ADD bits in the USART_CR2 register.
Note:

In 7-bit and 9-bit data modes, address detection is done on 6-bit and 8-bit addresses
(ADD[5:0] and ADD[7:0]) respectively.
The USART enters mute mode when an address character is received which does not
match its programmed address. In this case, the RWU bit is set by hardware. The RXNE
flag is not set for this address byte and no interrupt or DMA request is issued when the
USART enters mute mode. When FIFO management is enabled, the software must ensure
that there is at least one empty location in the RXFIFO before entering mute mode.
The USART also enters mute mode when the MMRQ bit is written to 1. The RWU bit is also
automatically set in this case.
The USART exits from mute mode when an address character is received which matches
the programmed address. Then the RWU bit is cleared and subsequent bytes are received
normally. The RXNE/RXFNE bit is set for the address character since the RWU bit has been
cleared.

Note:

When FIFO management is enabled, when MMRQ is set while the receiver is sampling last
bit of a data, this data may be received before effectively entering in mute mode
An example of mute mode behavior using address mark detection is given in Figure 818.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Figure 818. Mute mode using address mark detection
In this example, the current address of the receiver is 1
(programmed in the USART_CR2 register)
RXNE

IDLE

RX

Addr=0 Data 1 Data 2

RWU

IDLE

RXNE

Addr=1 Data 3 Data 4 Addr=2 Data 5

Mute mode

Normal mode
Matching address

MMRQ written to 1
(RXNE was cleared)

RXNE

Mute mode

Non-matching address

Non-matching address
MSv31155V1

66.5.12

USART Modbus communication
The USART offers basic support for the implementation of Modbus/RTU and Modbus/ASCII
protocols. Modbus/RTU is a half-duplex, block-transfer protocol. The control part of the
protocol (address recognition, block integrity control and command interpretation) must be
implemented in software.
The USART offers basic support for the end of the block detection, without software
overhead or other resources.

Modbus/RTU
In this mode, the end of one block is recognized by a “silence” (idle line) for more than 2
character times. This function is implemented through the programmable timeout function.
The timeout function and interrupt must be activated, through the RTOEN bit in the
USART_CR2 register and the RTOIE in the USART_CR1 register. The value corresponding
to a timeout of 2 character times (for example 22 x bit time) must be programmed in the
RTO register. When the receive line is idle for this duration, after the last stop bit is received,
an interrupt is generated, informing the software that the current block reception has not
completed.

Modbus/ASCII
In this mode, the end of a block is recognized by a specific (CR/LF) character sequence.
The USART manages this mechanism using the character match function.
By programming the LF ASCII code in the ADD[7:0] field and by activating the character
match interrupt (CMIE = 1), the software is informed when a LF has been received and can
check the CR/LF in the DMA buffer.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

66.5.13

Universal synchronous/asynchronous receiver transmitter (USART/UART)

USART parity control
Parity control (generation of parity bit in transmission and parity checking in reception) can
be enabled by setting the PCE bit in the USART_CR1 register. Depending on the frame
length defined by the M bits, the possible USART frame formats are as listed in Table 681.
Table 681. USART frame formats
M bits

PCE bit

USART frame(1)

00

0

| SB | 8 bit data | STB |

00

1

| SB | 7-bit data | PB | STB |

01

0

| SB | 9-bit data | STB |

01

1

| SB | 8-bit data PB | STB |

10

0

| SB | 7bit data | STB |

10

1

| SB | 6-bit data | PB | STB |

1. Legends: SB: start bit, STB: stop bit, PB: parity bit. In the data register, the PB is always taking the MSB
position (8th or 7th, depending on the M bit value).

Even parity
The parity bit is calculated to obtain an even number of “1s” inside the frame of the 6, 7 or 8
LSB bits (depending on M bit values) and the parity bit.
As an example, if data = 00110101, and 4 bits are set, then the parity bit is equal to 0 if even
parity is selected (PS bit in USART_CR1 = 0).

Odd parity
The parity bit is calculated to obtain an odd number of “1s” inside the frame made of the 6, 7
or 8 LSB bits (depending on M bit values) and the parity bit.
As an example, if data = 00110101 and 4 bits set, then the parity bit is equal to 1 if odd parity
is selected (PS bit in USART_CR1 = 1).

Parity checking in reception
If the parity check fails, the PE flag is set in the USART_ISR register and an interrupt is
generated if PEIE is set in the USART_CR1 register. The PE flag is cleared by software
writing 1 to the PECF in the USART_ICR register.

Parity generation in transmission
If the PCE bit is set in USART_CR1, then the MSB bit of the data written in the data register
is transmitted but is changed by the parity bit (even number of “1s” if even parity is selected
(PS = 0) or an odd number of “1s” if odd parity is selected (PS = 1)).

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

66.5.14

RM0456

USART LIN (local interconnection network) mode
This section is relevant only when LIN mode is supported. Refer to Section 66.4: USART
implementation.
The LIN mode is selected by setting the LINEN bit in the USART_CR2 register. In LIN
mode, the following bits must be kept cleared:
•

STOP[1:0] and CLKEN in the USART_CR2 register,

•

SCEN, HDSEL and IREN in the USART_CR3 register.

LIN transmission
The procedure described in Section 66.5.5 has to be applied for LIN master transmission. It
must be the same as for normal USART transmission with the following differences:
•

Clear the M bit to configure 8-bit word length.

•

Set the LINEN bit to enter LIN mode. In this case, setting the SBKRQ bit sends 13 zero
bits as a break character. Then 2 bits of value ‘1 are sent to enable the next start
detection.

LIN reception
When LIN mode is enabled, the break detection circuit is activated. The detection is totally
independent from the normal USART receiver. A break can be detected whenever it occurs,
during Idle state or during a frame.
When the receiver is enabled (RE = 1 in USART_CR1), the circuit looks at the RX input for
a start signal. The method for detecting start bits is the same when searching break
characters or data. After a start bit has been detected, the circuit samples the next bits
exactly like for the data (on the 8th, 9th and 10th samples). If 10 (when the LBDL = 0 in
USART_CR2) or 11 (when LBDL = 1 in USART_CR2) consecutive bits are detected as 0,
and are followed by a delimiter character, the LBDF flag is set in USART_ISR. If the LBDIE
bit = 1, an interrupt is generated. Before validating the break, the delimiter is checked for as
it signifies that the RX line has returned to a high level.
If a 1 is sampled before the 10 or 11 have occurred, the break detection circuit cancels the
current detection and searches for a start bit again.
If the LIN mode is disabled (LINEN = 0), the receiver continues working as normal USART,
without taking into account the break detection.
If the LIN mode is enabled (LINEN = 1), as soon as a framing error occurs (that is, stop bit
detected at 0, which is the case for any break frame), the receiver stops until the break
detection circuit receives either a ‘1, if the break word was not complete, or a delimiter
character if a break has been detected.
The behavior of the break detector state machine and the break flag is shown in Figure 819.
Examples of break frames are given in Figure 820.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)
Figure 819. Break detection in LIN mode (11-bit break length - LBDL bit is set)

Case 1: break signal not long enough => break discarded, LBDF is not set
Break frame

RX line
Capture strobe
Break state
machine

Idle

Bit0 Bit1 Bit2 Bit3
0

Read samples

0

0

0

Bit4 Bit5 Bit6
0

0

0

Bit7 Bit8 Bit9 Bit10
0

0

0

Idle

1

Case 2: break signal just long enough => break detected, LBDF is set
Break frame

RX line

Delimiter is immediate
Capture strobe
Break state
machine

Idle

Bit0 Bit1 Bit2 Bit3
0

Read samples

0

0

0

Bit4 Bit5 Bit6
0

0

0

Bit7 Bit8 Bit9 B10
0

0

0

Idle

0

LBDF

Case 3: break signal long enough => break detected, LBDF is set
Break frame

RX line
Capture strobe
Break state
Idle
machine
Read samples

Bit0 Bit1 Bit2 Bit3
0

0

0

0

Bit4 Bit5 Bit6
0

0

0

Bit7 Bit8 Bit9 Bit10 wait delimiter Idle
0

0

0

0

LBDF

MSv31156V1

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Figure 820. Break detection in LIN mode vs. Framing error detection
Case 1: break occurring after an Idle
RX line

data 1

IDLE

BREAK
1 data time

data 2 (0x55)

data 3 (header)

1 data time

RXNE /FE
LBDF

Case 2: break occurring while data is being received
RX line

data 1

data2

BREAK

1 data time

data 2 (0x55)

data 3 (header)

1 data time

RXNE /FE
LBDF
MSv31157V1

66.5.15

USART synchronous mode
Master mode
The synchronous master mode is selected by programming the CLKEN bit in the
USART_CR2 register to 1. In synchronous mode, the following bits must be kept cleared:
•

LINEN bit in the USART_CR2 register,

•

SCEN, HDSEL and IREN bits in the USART_CR3 register.

In this mode, the USART can be used to control bidirectional synchronous serial
communications in master mode. The CK pin is the output of the USART transmitter clock.
No clock pulses are sent to the CK pin during start bit and stop bit. Depending on the state
of the LBCL bit in the USART_CR2 register, clock pulses are, or are not, generated during
the last valid data bit (address mark). The CPOL bit in the USART_CR2 register is used to
select the clock polarity, and the CPHA bit in the USART_CR2 register is used to select the
phase of the external clock (see Figure 821, Figure 822 and Figure 823).
During the Idle state, preamble and send break, the external CK clock is not activated.
In synchronous master mode, the USART transmitter operates exactly like in asynchronous
mode. However, since CK is synchronized with TX (according to CPOL and CPHA), the
data on TX is synchronous.
In synchronous master mode, the USART receiver operates in a different way compared to
asynchronous mode. If RE is set to 1, the data are sampled on CK (rising or falling edge,
depending on CPOL and CPHA), without any oversampling. A given setup and a hold time
must be respected (which depends on the baud rate: 1/16 bit time).

<!-- pagebreak -->

RM0456 Rev 6

RM0456
Note:

Universal synchronous/asynchronous receiver transmitter (USART/UART)
In master mode, the CK pin operates in conjunction with the TX pin. Thus, the clock is
provided only if the transmitter is enabled (TE = 1) and data are being transmitted
(USART_TDR data register written). This means that it is not possible to receive
synchronous data without transmitting data.
Figure 821. USART example of synchronous master transmission

RX
TX

Data out
Data in
Synchronous device
(slave SPI)

USART

Clock

CK

MSv31158V2

Figure 822. USART data clock timing diagram in synchronous master mode
(M bits = 00)
Idle or preceding
Start
transmission

Idle or next
Stop transmission

M bits = 00 (8 data bits)

Clock (CPOL=0, CPHA=0)

*

Clock (CPOL=0, CPHA=1)

*
*

Clock (CPOL=1, CPHA=0)
*

Clock (CPOL=1, CPHA=1)
Data on TX
(from master)

0
Start

Data on RX
(from slave)

1

2

3

4

5

6

LSB
0

7
MSB Stop

1

2

3

4

5

6

7
MSB
*

LSB
Capture strobe

*LBCL bit controls last data pulse
MSv34709V2

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Figure 823. USART data clock timing diagram in synchronous master mode
(M bits = 01)
Idle or
preceding Start
transmission

M bits =01 (9 data bits)

Stop

Clock (CPOL=0,
CPHA=0)

*

Clock (CPOL=0,
CPHA=1)

*

Clock (CPOL=1,
CPHA=0)

*

Clock (CPOL=1,
CPHA=1)
Data on TX
(from master)

*

0

1

2

3

4

5

6

7

Start LSB
Data on RX
(from slave)

Idle or next
transmission

0

8
MSB

1

2

3

4

5

6

7

LSB
Capture
strobe

Stop

8
MSB
*

*LBCL bit controls last data pulse
MSv34710V1

Slave mode
The synchronous slave mode is selected by programming the SLVEN bit in the
USART_CR2 register to 1. In synchronous slave mode, the following bits must be kept
cleared:
•

LINEN and CLKEN bits in the USART_CR2 register,

•

SCEN, HDSEL and IREN bits in the USART_CR3 register.

In this mode, the USART can be used to control bidirectional synchronous serial
communications in slave mode. The CK pin is the input of the USART in slave mode.
Note:

When the peripheral is used in SPI slave mode, the frequency of peripheral clock source
(usart_ker_ck_pres) must be greater than 3 times the CK input frequency.
The CPOL bit and the CPHA bit in the USART_CR2 register are used to select the clock
polarity and the phase of the external clock, respectively (see Figure 824).
An underrun error flag is available in slave transmission mode. This flag is set when the first
clock pulse for data transmission appears while the software has not yet loaded any value to
USART_TDR.
The slave supports the hardware and software NSS management.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)
Figure 824. USART data clock timing diagram in synchronous slave mode
(M bits = 00)
M bits = 00 (8 data bits)
NSS (from Master)
Clock (CPOL=0, CPHA=0)
Clock (CPOL=0, CPHA=1)

Clock (CPOL=1, CPHA=0)
Clock (CPOL=1, CPHA=1)
Data on TX
(from slave)

0

1

2

3

4

5

6

LSB
Data on RX
(from master)

0

7
MSB

1

2

3

4

5

6

7
MSB

LSB
Capture strobe

MSv45359V1

Slave Select (NSS) pin management
The hardware or software slave select management can be set through the DIS_NSS bit in
the USART_CR2 register:
•

Software NSS management (DIS_NSS = 1)
SPI slave is always selected and NSS input pin is ignored.
The external NSS pin remains free for other application uses.

•

Hardware NSS management (DIS_NSS = 0)
The SPI slave selection depends on NSS input pin. The slave is selected when NSS is
low and deselected when NSS is high.

Note:

The LBCL (used only on SPI master mode), CPOL and CPHA bits have to be selected when
the USART is disabled (UE = 0) to ensure that the clock pulses function correctly.
In SPI slave mode, the USART must be enabled before starting the master communications
(or between frames while the clock is stable). Otherwise, if the USART slave is enabled
while the master is in the middle of a frame, it becomes desynchronized with the master.
The data register of the slave needs to be ready before the first edge of the communication
clock or before the end of the ongoing communication, otherwise the SPI slave transmits
zeros.
SPI slave underrun error
When an underrun error occurs, the UDR flag is set in the USART_ISR register, and the SPI
slave goes on sending the last data until the underrrun error flag is cleared by software.
The underrun flag is set at the beginning of the frame. An underrun error interrupt is
triggered if EIE bit is set in the USART_CR3 register.
The underrun error flag is cleared by setting bit UDRCF in the USART_ICR register.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

In case of underrun error, it is still possible to write to the TDR register. Clearing the
underrun error enables sending new data.
If an underrun error occurred and there is no new data written in TDR, then the TC flag is set
at the end of the frame.
Note:

An underrun error may occur if the moment the data is written to the USART_TDR is too
close to the first CK transmission edge. To avoid this underrun error, the USART_TDR must
be written 3 usart_ker_ck cycles before the first CK edge.

66.5.16

USART single-wire half-duplex communication
Single-wire half-duplex mode is selected by setting the HDSEL bit in the USART_CR3
register. In this mode, the following bits must be kept cleared:
•

LINEN and CLKEN bits in the USART_CR2 register,

•

SCEN and IREN bits in the USART_CR3 register.

The USART can be configured to follow a single-wire half-duplex protocol where the TX and
RX lines are internally connected. The selection between half- and full-duplex
communication is made with a control bit HDSEL in USART_CR3.
As soon as HDSEL is written to 1:
•

The TX and RX lines are internally connected.

•

The RX pin is no longer used.

•

The TX pin is always released when no data is transmitted. Thus, it acts as a standard
I/O in idle or in reception. It means that the I/O must be configured so that TX is
configured as alternate function open-drain with an external pull-up.

Apart from this, the communication protocol is similar to normal USART mode. Any conflict
on the line must be managed by software (for instance by using a centralized arbiter). In
particular, the transmission is never blocked by hardware and continues as soon as data are
written in the data register while the TE bit is set.

66.5.17

USART receiver timeout
The receiver timeout feature is enabled by setting the RTOEN bit in the USART_CR2
control register.
The timeout duration is programmed using the RTO bitfields in the USART_RTOR register.
The receiver timeout counter starts counting:
• from the end of the stop bit if STOP = 00 or STOP = 11
• from the end of the second stop bit if STOP = 10.
• from the beginning of the stop bit if STOP = 01.
When the timeout duration has elapsed, the RTOF flag in the USART_ISR register is set. A
timeout is generated if RTOIE bit in USART_CR1 register is set.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

66.5.18

Universal synchronous/asynchronous receiver transmitter (USART/UART)

USART smartcard mode
This section is relevant only when smartcard mode is supported. Refer to Section 66.4:
USART implementation.
Smartcard mode is selected by setting the SCEN bit in the USART_CR3 register. In
smartcard mode, the following bits must be kept cleared:
•

LINEN bit in the USART_CR2 register,

•

HDSEL and IREN bits in the USART_CR3 register.

The CLKEN bit can also be set to provide a clock to the smartcard.
The smartcard interface is designed to support asynchronous smartcard protocol as defined
in the ISO 7816-3 standard. Both T = 0 (character mode) and T = 1 (block mode) are
supported.
The USART must be configured as:
•

8 bits plus parity: M = 1 and PCE = 1 in the USART_CR1 register

•

1.5 stop bits when transmitting and receiving data: STOP = 11 in the USART_CR2
register. It is also possible to choose 0.5 stop bit for reception.

In T = 0 (character) mode, the parity error is indicated at the end of each character during
the guard time period.
Figure 825 shows examples of what can be seen on the data line with and without parity
error.
Figure 825. ISO 7816-3 asynchronous protocol

Without Parity error
S

0

1

Guard time
2

3

4

5

6

7

p

Start bit
WithParity error
S

0

Guard time
1

2

3

4

Start bit

5

6

7

p
Line pulled low by receiver
during stop in case of parity error
MSv31162V1

When connected to a smartcard, the TX output of the USART drives a bidirectional line that
is also driven by the smartcard. The TX pin must be configured as open drain.
Smartcard mode implements a single wire half duplex communication protocol.
•

Transmission of data from the transmit shift register is guaranteed to be delayed by a
minimum of 1/2 baud clock. In normal operation a full transmit shift register starts
shifting on the next baud clock edge. In smartcard mode this transmission is further
delayed by a guaranteed 1/2 baud clock.

•

In transmission, if the smartcard detects a parity error, it signals this condition to the
USART by driving the line low (NACK). This NACK signal (pulling transmit line low for 1
baud clock) causes a framing error on the transmitter side (configured with 1.5 stop
bits). The USART can handle automatic re-sending of data according to the protocol.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

The number of retries is programmed in the SCARCNT bitfield. If the USART continues
receiving the NACK after the programmed number of retries, it stops transmitting and
signals the error as a framing error. The TXE bit (TXFNF bit in case FIFO mode is
enabled) may be set using the TXFRQ bit in the USART_RQR register.

Note:

•

Smartcard auto-retry in transmission: A delay of 2.5 baud periods is inserted between
the NACK detection by the USART and the start bit of the repeated character. The TC
bit is set immediately at the end of reception of the last repeated character (no guard
time). If the software wants to repeat it again, it must ensure the minimum 2-baud
periods required by the standard.

•

If a parity error is detected during reception of a frame programmed with a 1.5 stop bit
period, the transmit line is pulled low for a baud clock period after the completion of the
receive frame. This is to indicate to the smartcard that the data transmitted to the
USART has not been correctly received. A parity error is NACKed by the receiver if the
NACK control bit is set, otherwise a NACK is not transmitted (to be used in T = 1
mode). If the received character is erroneous, the RXNE (RXFNE in case FIFO mode
is enabled)/receive DMA request is not activated. According to the protocol
specification, the smartcard must resend the same character. If the received character
is still erroneous after the maximum number of retries specified in the SCARCNT
bitfield, the USART stops transmitting the NACK and signals the error as a parity error.

•

Smartcard auto-retry in reception: the BUSY flag remains set if the USART NACKs the
card but the card doesn’t repeat the character.

•

In transmission, the USART inserts the guard time (as programmed in the guard time
register) between two successive characters. As the guard time is measured after the
stop bit of the previous character, the GT[7:0] register must be programmed to the
desired CGT character guard time, as defined by the 7816-3 specification) minus 12
(the duration of one character).

•

The assertion of the TC flag can be delayed by programming the guard time register. In
normal operation, TC is asserted when the transmit shift register is empty and no
further transmit requests are outstanding. In smartcard mode an empty transmit shift
register triggers the guard time counter to count up to the programmed value in the
guard time register. TC is forced low during this time. When the guard time counter
reaches the programmed value TC is asserted high. The TCBGT flag can be used to
detect the end of data transfer without waiting for guard time completion. This flag is set
just after the end of frame transmission and if no NACK has been received from the
card.

•

The de-assertion of TC flag is unaffected by smartcard mode.

•

If a framing error is detected on the transmitter end (due to a NACK from the receiver),
the NACK is not detected as a start bit by the receive block of the transmitter.
According to the ISO protocol, the duration of the received NACK can be 1 or 2 baud
clock periods.

•

On the receiver side, if a parity error is detected and a NACK is transmitted the receiver
does not detect the NACK as a start bit.

Break characters are not significant in smartcard mode. A 0x00 data with a framing error is
treated as data and not as a break.
No Idle frame is transmitted when toggling the TE bit. The Idle frame (as defined for the
other configurations) is not defined by the ISO protocol.
Figure 826 shows how the NACK signal is sampled by the USART. In this example the
USART is transmitting data and is configured with 1.5 stop bits. The receiver part of the
USART is enabled in order to check the integrity of the data and the NACK signal.

<!-- pagebreak -->

