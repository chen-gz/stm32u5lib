Byte address

31:24

23:16

15:8

7:0

0

Dn+3[7:0]

Dn+2[7:0]

Dn+1[7:0]

Dn[7:0]

4

Dn+7[7:0]

Dn+6[7:0]

Dn+5[7:0]

Dn+4[7:0]

RM0456 Rev 6

RM0456

Parallel synchronous slave interface (PSSI)

16-bit data
When EDM[1:0] in PSSI_CR are programmed to 11, the interface transfers 16 bits using the
D[15:0] pins. In this case, two PSSI_PDCK cycles are required to transfer a 32-bit word.
The least-significant half word (bits 15:0) correspond to the first half word transferred, and
the most-significant half-word (bits 31:16) corresponds to the second half word transferred.
Table 425 illustrates the positioning of the data in two 32-bit words.
Table 425. Positioning of captured data bytes in 32-bit words (16-bit width)
Byte address

31:16

15:0

0

Dn+1[15:0]

Dn[15:0]

4

Dn+3[15:0]

Dn+2[15:0]

FIFO data buffer and error conditions
An eight-word FIFO helps improving performance and avoids overruns and underruns.
If the ready signal (PSSI_RDY) is disabled in receive mode, an overrun error is generated
when a clock active edge occurs when the FIFO is full. In this case, the input data is lost.
If the data enable signal (PSSI_DE) is disabled in transmit mode, an underrun error is
generated when a clock active edge occurs when the FIFO is empty. In this case,
unpredictable data are output.
The OVR_RIS status bit indicates that either an overrun or an underrun occurred. An
interrupt can be generated when these events occur.

42.3.5

PSSI optional control signals
Data Enable (PSSI_DE) alternate function input
The data enable signal, PSSI_DE, is an optional signal. It is driven by the data
source/transmitter in order to indicate that the data is valid to be transferred during the
current cycle. When PSSI_DE is inactive, it means that the data must not be sampled by the
receiver at the next clock edge.
This alternate function signal can be enabled using the DERDYCFG (bits 20:18 of
PSSI_CR) control bits. PSSI_DE polarity is configured through DEPOL control bit (bit 6 of
PSSI_CR). PSSI_DE is active low when DEPOL is cleared to 0, and high when DEPOL is et
to 1.
The direction of the PSSI_DE signal is defined by the OUTEN value. It is the same as the
data direction.
If the PSSI_DE alternate function input is enabled (through DERDYCFG) in receive mode
(OUTEN cleared to 0), the PSSI samples PSSI_DE on the same PSSI_PDCK edge as the
one used for sampling the data (D[15:0]). If PSSI_DE is active, the sampled data is saved in
the FIFO. Otherwise, the sampled data is considered invalid and discarded. The
transmitting device can use PSSI_DE as a data valid signal, driving it inactive when the data
in the current cycle is not valid. This flow control function allows avoiding underrun errors.

RM0456 Rev 6

<!-- pagebreak -->

1710

Parallel synchronous slave interface (PSSI)

RM0456

Figure 402. Data enable in receive mode waveform diagram (CKPOL=0)

PSSI_PDCK

PSSI_D[15:0]

PSSI_DE

MSv48846V2

If the PSSI_DE alternate output function is enabled (through DERDYCFG) in transmit mode
(OUTEN=1), the PSSI drives PSSI_DE on the same PSSI_PDCK edge that the one used to
drive the data (D[15:0]). If a new 8 or 16-bit data (as programmed in the EDM[1:0] control
bits in PSSI_CR) is available for transmission in the internal FIFO, this data is output on the
data outputs (D[15:0]) and the PSSI_DE output becomes active on the current PSSI_PDCK
edge. Otherwise (if the TX FIFO is empty), the D[15:0] outputs remains unchanged on the
next clock edge and the PSSI_DE output becomes inactive.
Figure 403. Data enable waveform diagram in transmit mode (CKPOL=0)

PSSI_PDCK

PSSI_D[15:0]

PSSI_DE

MSv48847V2

Ready (PSSI_RDY) alternate function output
The ready signal, PSSI_RDY, is an optional signal. It is driven by the receiving device and
indicates whether data is being accepted in the current cycle. When PSSI_RDY is inactive,
it means that the data must not be sampled by the receiver at the next clock edge.
This alternate function signal can be enabled using the DERDYCFG control bits (bits 20:18
of PSSI_CR). PSSI_RDY polarity is configured through the RDYPOL control bit (bit 6 of
PSSI_CR). PSSI_RDY is active low when RDYPOL is cleared to 0, and high when RDYPOL
set to 1.
The direction of the PSSI_RDY signal is defined by the OUTEN (bit 31 of PSSI_CR). It is set
in the opposite direction compared to the PSSI_DE and data signals.
If the PSSI_RDY alternate output function is enabled (through DERDYCFG) in receive
mode (OUTEN=0), the PSSI drives PSSI_RDY one PSSI_PDCK half cycle after it samples

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Parallel synchronous slave interface (PSSI)
the data (D[15:0]). If the FIFO has enough free space to receive more data, the PSSI drives
the PSSI_RDY signal active. Otherwise, if the FIFO is full and cannot accept more data, the
PSSI drives the PSSI_RDY signal inactive. The transmitting device must repeat the current
data in the next cycle when it detects that PSSI_RDY is inactive. This flow control function
allows the PSSI to avoid overrun errors when the system (via the DMA) is unable to keep up
with the data flow.
Figure 404. Ready in receive mode waveform diagram (CKPOL=0)

PSSI_PDCK

PSSI_D[15:0]

PSSI_DE

PSSI_RDY

MSv48848V2

If the PSSI_RDY alternate input function is enabled (through DERDYCFG) in transmit mode
(OUTEN=1), the PSSI samples the PSSI_RDY signal on the opposite PSSI_PDCK edge to
the one at which D[15:0] are driven. If the PSSI_RDY signal is inactive, the PSSI keeps the
same data (D[15:0]) and PSSI_DE signals that valid data are available during the next
PSSI_PDCK clock cycle. Otherwise, if PSSI_RDY signal is sampled as active, the next data
from the TX FIFO (if available) is output on the data outputs (D[15:0]). If no new data are
available in the TX FIFO, the PSSI keeps the data output values and outputs the PSSI_DE
signal as inactive (if enabled).
The receiving device uses the PSSI_RDY to control the data flow and avoid overrun errors
when the system (via the DMA) is unable to keep up with the data flow.

Bidirectional PSSI_DE/PSSI_RDY signal
A single pin can be used for both data enable (PSSI_DE) and ready (PSSI_RDY) functions
if DEPOL and RDYPOL are both set to 1 and DERDYCFG is set to 111 or 100 in the
PSSI_CR register. In this case, the GPIO corresponding to selected alternate function
(PSSI_DE when DERDYCFG=111 or PSSI_RDY when DERDYCFG=100) must be
configured as open-drain. The other device must also be configured to drive the line as
open-drain, and a weak pull-up must be applied to the line.
The signal thus becomes bidirectional. If either the sender drives the line low (to indicate
that the data is not valid) or the receiver drives the line low (to indicate that it is not sampling
the current data), then both devices know that the data is not being transferred in the current
cycle.

RM0456 Rev 6

<!-- pagebreak -->

