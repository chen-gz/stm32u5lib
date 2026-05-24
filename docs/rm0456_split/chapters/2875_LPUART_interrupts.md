3020

Serial audio interface (SAI)

RM0456
Table 712. SOPD pattern
Preamble coding

SOPD

Description
last bit is 0

last bit is 1

B

11101000

00010111

Channel A data at the start of block

W

11100100

00011011

Channel B data somewhere in the block

M

11100010

00011101

Channel A data

The data stored in SAI_xDR has to be filled as follows:
•

SAI_xDR[26:24] contain the channel status, user, and validity bits.

•

SAI_xDR[23:0] contain the 24-bit data for the considered channel.

If the data size is 20 bits, the data must be mapped on SAI_xDR[23:4].
If the data size is 16 bits, the data must be mapped on SAI_xDR[23:8].
SAI_xDR[23] always represents the MSB.
Figure 875. SAI_xDR register ordering

SAI_xDR[26:0]
0

26
CS U

V D23 D22 D21 D20 D19 D18 D17 D16 D15 D14 D13 D12 D11 D10 D9 D8 D7 D6 D5 D4 D3 D2 D1 D0

Data[23:0]

Status
bits

MSv31174V1

Note:

The transfer is always performed with LSB first.
The SAI first sends the adequate preamble for each subframe in a block. The SAI_xDR is
then sent on the SD line (Manchester coded). The SAI ends the subframe by transferring
the parity bit calculated as described in Table 713.
Table 713. Parity bit calculation
SAI_xDR[26:0]

Parity bit P value transferred

odd number of 0

0

odd number of 1

1

The underrun is the only error flag available in the SAI_xSR register for SPDIF mode since
the SAI can only operate in transmitter mode. As a result, the following sequence must be

<!-- pagebreak -->

