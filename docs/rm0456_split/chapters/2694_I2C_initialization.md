The DMAT bit must not be cleared before the DMA end of transfer.

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)
Figure 829. Transmission using DMA
Idle
preamble

Frame 1

Frame 2

Frame 3

TX line
Set by hardware
Cleared by DMA write

TXE flag

Set by hardware
Cleared by DMA write

Ignored by the DMA because the
transfer is complete

DMA request
USART_TDR

Set by hardware

F1

F2

F3
Set by
hardware

TC flag
DMA writes
to USART_TDR
Set by
hardware

DMA transfer
complete flag

Cleared by software

The software
The DMA
The DMA
The DMA
configures DMA
to send 3 data writes F1 into writes F2 into writes F3 into
USART_TDR USART_TDR USART_TDR
blocks and
enables USART

The DMA
transfer is
complete

The software waits until TC = 1

ai17192d

Note:

When FIFO management is enabled, the DMA request is triggered by transmit FIFO not full
(that is, TXFNF = 1).

Reception using DMA
DMA mode can be enabled for reception by setting the DMAR bit in the USART_CR3
register. Data are loaded from the USART_RDR register to an SRAM area configured using
the DMA peripheral (refer to section direct memory access controller (DMA)) whenever a
data byte is received. To map a DMA channel for USART reception, use the following
procedure:
1.

Write the USART_RDR register address in the DMA control register to configure it as
the source of the transfer. The data is moved from this address to the memory after
each RXNE (RXFNE in case FIFO mode is enabled) event.

2.

Write the memory address in the DMA control register to configure it as the destination
of the transfer. The data is loaded from USART_RDR to this memory area after each
RXNE (RXFNE in case FIFO mode is enabled) event.

3.

Configure the total number of bytes to be transferred to the DMA control register.

4.

Configure the channel priority in the DMA control register

5.

Configure interrupt generation after half/ full transfer as required by the application.

6.

Activate the channel in the DMA control register.

When the number of data transfers programmed in the DMA controller is reached, the DMA
controller generates an interrupt on the DMA channel interrupt vector.
Note:

The DMAR bit must not be cleared before the DMA end of transfer.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Figure 830. Reception using DMA
Frame 2

Frame 1

Frame 3

RX line
Set by hardware
Cleared by DMA read

RXNE flag
DMA request

F3

F2

F1

USART_RDR
DMA reads
USART_RDR
DMA transfer
complete flag
The software configures
the DMA to receive 3
data blocks and enables
the USART

Cleared
by
software

Set by hardware

The DMA reads F1
from USART_RDR

The DMA reads F2
from USART_RDR

The DMA reads
F3 from
USART_RDR

The DMA transfer
is complete

ai17193e

Note:

When FIFO management is enabled, the DMA request is triggered by receive FIFO not
empty (that is, RXFNE = 1).

Error flagging and interrupt generation in multibuffer communication
If any error occurs during a transaction in multibuffer communication mode, the error flag is
asserted after the current byte. An interrupt is generated if the interrupt enable flag is set.
For framing error, overrun error and noise flag, which are asserted with RXNE (RXFNE in
case FIFO mode is enabled) in single byte reception, there is a separate error flag interrupt
enable bit (EIE bit in the USART_CR3 register), which, if set, enables an interrupt after the
current byte if any of these errors occur.

66.5.21

RS232 hardware flow control and RS485 driver enable
It is possible to control the serial data flow between two devices by using the CTS input and
the RTS output. The Figure 831 shows how to connect two devices in this mode:
Figure 831. Hardware flow control between two USARTs

USART 2

USART 1

TX circuit

TX

RX

CTS

RTS

RX

TX

RTS

CTS

RX circuit

RX circuit

TX circuit

MSv31169V2

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)
RS232 RTS and CTS flow control can be enabled independently by writing the RTSE and
CTSE bits to 1 in the USART_CR3 register.

RS232 RTS flow control
If the RTS flow control is enabled (RTSE = 1), then RTS is deasserted (tied low) as long as
the USART receiver is ready to receive a new data. When the receive register is full, RTS is
asserted, indicating that the transmission is expected to stop at the end of the current frame.
Figure 832 shows an example of communication with RTS flow control enabled.
Figure 832. RS232 RTS flow control

RX

Start
bit

Stop
Start
Idle
bit
bit

Data 1

Data 2

Stop
bit

RTS

RXNE

RXNE
Data 1 read
Data 2 can now be transmitted

MSv68794V1

Note:

When FIFO mode is enabled, RTS is asserted only when RXFIFO is full.

RS232 CTS flow control
If the CTS flow control is enabled (CTSE = 1), then the transmitter checks the CTS input
before transmitting the next frame. If CTS is deasserted (tied low), then the next data is
transmitted (assuming that data is to be transmitted, in other words, if TXE/TXFE = 0), else
the transmission does not occur. When CTS is asserted during a transmission, the current
transmission completes before the transmitter stops.
When CTSE = 1, the CTSIF status bit is automatically set by hardware as soon as the CTS
input toggles. It indicates when the receiver becomes ready or not ready for communication.
An interrupt is generated if the CTSIE bit in the USART_CR3 register is set. Figure 833
shows an example of communication with CTS flow control enabled.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Figure 833. RS232 CTS flow control
CTS

CTS

CTS
Transmit data register
TDR

Data 2

TX

Data 1

Data 3

empty

Stop Start
bit bit

Data 2

Writing data 3 in TDR

empty

Start
Stop
Idle
bit
bit

Data 3

Transmission of Data 3 is
delayed until CTS = 0
MSv68793V1

Note:

For correct behavior, CTS must be deasserted at least three USART clock source periods
before the end of the current character. In addition, it must be noted that the CTSCF flag
may not be set for pulses shorter than 2 x PCLK periods.

RS485 driver enable
The driver enable feature is enabled by setting bit DEM in the USART_CR3 control register.
This enables the user to activate the external transceiver control, through the DE (driver
enable) signal. The deassertion time is the time between the end of the last stop bit, in a
transmitted message, and the de-activation of the DE signal. It is programmed using the
DEDT [4:0] bitfields in the USART_CR1 control register. The polarity of the DE signal can be
configured using the DEP bit in the USART_CR3 control register.
In USART, the DEAT and DEDT are expressed in sample time units (1/8 or 1/16 bit time,
depending on the oversampling rate).

66.5.22

USART autonomous mode
The USART peripheral can be functional in Stop mode thanks to the autonomous mode.
This mode can also be used in Run and Sleep mode. The UESM bit must be set prior to
entering low-power mode.
The APB clock is requested by the peripheral each time the USART status needs to be
updated. Once the USART receives the APB clock, it generates either an interrupt or a DMA
request, depending on the peripheral configuration.
If an interrupt is generated, the device wakes up from Stop mode. If no interrupt is
generated, the device remains in Stop mode but the kernel and APB clocks are still
available for the USART and all the autonomous peripherals enabled in the reset and clock
controller (RCC). If DMA requests are enabled, the data are directly transferred to/from the
SRAM thanks to the DMA while the product remains in Stop mode.

<!-- pagebreak -->

