1111

Octo-SPI I/O manager (OCTOSPIM)

RM0456
Figure 163. OCTOSPIM block diagram
OCTOSPI I/O manager
Control
register

AHB
interface

AHB

Static
muxing

OCTOSPIM_P1_CLK

Port 1

OCTOSPI1
bus signals
OCTOSPI1

PnCR
P1CR

ACK1

OCTOSPIM_P1_DQS
OCTOSPIM_P1_NCS
OCTOSPIM_P1_IO[7:0]

REQ1
Arbiter

I/O matrix
Dynamic
muxing

OCTOSPIM_Pn_CLK

Port n

REQ2
OCTOSPI2

OCTOSPIM_P1_NCLK

ACK2

OCTOSPIM_Pn_NCLK
OCTOSPIM_Pn_DQS
OCTOSPIM_Pn_NCS
OCTOSPIM_Pn_IO[7:0]

OCTOSPI2
bus signals
MS55487V1

1. The number of ports (n) is 2.
2. Arbitration is possible for both I/O matrix input ports.

29.4.2

OCTOSPIM input/output pins
Table 261. OCTOSPIM input/output pins
Pin name(1)
OCTOSPIM_Px_NCLK

Signal type

Output

OCTOSPIM_Px_CLK

Description
OCTOSPI inverted clock to support 1.8 V
HyperBus protocol
OCTOSPI clock

OCTOSPIM_Px_IOn (n = 0 to 7)

Input/output

OCTOSPI data pins

OCTOSPIM_Px_NCS

Output

Chip select for the memory

OCTOSPIM_Px_DQS

Input/output

Data strobe/write mask signal from/to the memory

1. x = 1 to 2.

<!-- pagebreak -->

