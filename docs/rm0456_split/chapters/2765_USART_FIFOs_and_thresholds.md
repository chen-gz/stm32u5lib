2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

For framing error, overrun error and noise flag, which are asserted with RXNE (RXFNE in
case FIFO mode is enabled) in single byte reception, there is a separate error flag interrupt
enable bit (EIE bit in the LPUART_CR3 register), which, if set, enables an interrupt after the
current byte if any of these errors occur.

67.4.14

RS232 hardware flow control and RS485 driver enable
It is possible to control the serial data flow between two devices by using the CTS input and
the RTS output. Figure 843 shows how to connect two devices in this mode.
Figure 843. Hardware flow control between two LPUARTs

LPUART 2

LPUART 1

TX circuit

RX circuit

TX

RX

CTS

RTS

RX

TX

RTS

CTS

RX circuit

TX circuit

MSv31892V2

RS232 RTS and CTS flow control can be enabled independently by writing the RTSE and
CTSE bits respectively to 1 (in the LPUART_CR3 register).

RS232 RTS flow control
If the RTS flow control is enabled (RTSE = 1), then RTS is deasserted (tied low) as long as
the LPUART receiver is ready to receive a new data. When the receive register is full, RTS
is asserted, indicating that the transmission is expected to stop at the end of the current
frame. Figure 844 shows an example of communication with RTS flow control enabled.
Figure 844. RS232 RTS flow control

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

<!-- pagebreak -->

When FIFO mode is enabled, RTS is asserted only when RXFIFO is full.

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

RS232 CTS flow control
If the CTS flow control is enabled (CTSE = 1), then the transmitter checks the CTS input
before transmitting the next frame. If CTS is deasserted (tied low), then the next data is
transmitted (assuming that data is to be transmitted, in other words, if TXE/TXFE = 0), else
the transmission does not occur. When CTS is asserted during a transmission, the current
transmission completes before the transmitter stops.
When CTSE = 1, the CTSIF status bit is automatically set by hardware as soon as the CTS
input toggles. It indicates when the receiver becomes ready or not ready for communication.
An interrupt is generated if the CTSIE bit in the LPUART_CR3 register is set. Figure 845
shows an example of communication with CTS flow control enabled.
Figure 845. RS232 CTS flow control
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

Stop
Start
Idle
bit
bit

Data 3

Transmission of Data 3 is
delayed until CTS = 0
MSv68793V1

Note:

For correct behavior, CTS must be deasserted at least 3 LPUART clock source periods
before the end of the current character. In addition it must be noted that the CTSCF flag may
not be set for pulses shorter than 2 x PCLK periods.

RS485 driver enable
The driver enable feature is enabled by setting bit DEM in the LPUART_CR3 control
register. This enables activating the external transceiver control, through the DE (Driver
Enable) signal. The assertion time is the time between the activation of the DE signal and
the beginning of the start bit. It is programmed using the DEAT [4:0] bitfields in the
LPUART_CR1 control register. The deassertion time is the time between the end of the last
stop bit, in a transmitted message, and the deactivation of the DE signal. It is programmed
using the DEDT [4:0] bitfields in the LPUART_CR1 control register. The polarity of the DE
signal can be configured using the DEP bit in the LPUART_CR3 control register.

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

The LPUART DEAT and DEDT are expressed in LPUART clock source (flpuart_ker_ck_pres)
cycles:
•

•

The driver enable assertion time equals
–

(1 + (DEAT x P)) x lpuart_ker_ck_pres, if P # 0

–

(1 + DEAT) x lpuart_ker_ck_pres, if P = 0

The driver enable deassertion time equals
–

(1 + (DEDT x P)) x lpuart_ker_ck_pres, if P # 0

–

(1 + DEDT) x lpuart_ker_ck_pres, if P = 0

where P = BRR[20:11]

67.4.15

LPUART autonomous mode
The LPUART peripheral can be functional in Stop mode thanks to the autonomous mode.
This mode can also be used in Run and Sleep mode. The UESM bit must be set prior to
entering low-power mode.
The APB clock is requested by the peripheral each time the LPUART status needs to be
updated. Once the LPUART receives the kernel and APB clocks, it generates either an
interrupt or a DMA request, depending on the peripheral configuration.
If an interrupt is generated, the device wakes up from Stop mode. If no interrupt is
generated, the device remains in Stop mode but the APB clock is still available for the
LPUART and all the autonomous peripherals enabled in the reset and clock controller
(RCC). If DMA requests are enabled, the data are directly transferred to/from the SRAM
thanks to the DMA while the product remains in Stop mode.

LPUART transmission mode
In transmission, the APB clock is requested only when the TE bit is set and in the following
cases:
•
If the FIFO mode is enabled, the APB clock is requested when

•

–

The TxFIFO is empty (TXFE = 1) and the corresponding interrupt is enabled
(TXFEIE = 1)

–

The TxFIFO threshold is reached (TXFT = 1) and the corresponding interrupt is
enabled (TXFTIE = 1)

–

The TxFIFO is not full (TXFNF = 1) and the corresponding interrupt or DMA is
enabled (TXFNFIE = 1 or DMAT = 1)

If the FIFO mode is disabled, the APB clock is requested as soon as data are
transferred to the shift register. The DMA or associated interrupt must be enabled.

The TE bit is set by hardware if an asynchronous trigger is detected.
A transmission is automatically launched when an asynchronous trigger is detected in Run,
Sleep, or Stop mode. The trigger is selected through the TRIGSEL bit in the
LPUART_AUTOCR register. It sets the TE bit in the LPUART_CR1 register and generates
an APB clock request to enable the transfer. The APB clock is requested until the
transmission completes and the TE bit is cleared by hardware when the programmed
number of data to be transmitted (TDN bitfield in the LPUART_AUTOCR register) is
reached. In this case, the TC flag is set when the number of data to be transmitted is
reached and the last byte is transmitted.

<!-- pagebreak -->

