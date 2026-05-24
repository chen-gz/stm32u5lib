•

Alternatively, uses fully programmable polynomial with programmable size (7, 8, 16, 32
bits)

•

Handles 8-,16-, 32-bit data size

•

Programmable CRC initial value

•

Single input/output 32-bit data register

•

Input buffer to avoid bus stall during calculation

•

CRC computation done in 4 AHB clock cycles (HCLK) for the 32-bit data size

•

General-purpose 8-bit register (can be used for temporary storage)

•

Reversibility option on I/O data

•

Accessed through AHB slave peripheral by 32-bit words only, with the exception of
CRC_DR register that can be accessed by words, right-aligned half-words and
right-aligned bytes

RM0456 Rev 6

RM0456

Cyclic redundancy check calculation unit (CRC)

24.3

CRC functional description

24.3.1

CRC block diagram
Figure 104. CRC calculation unit block diagram

32-bit AHB bus
read access

write access

Data register
(output)

crc_hclk

32-bit accesses

Data register
(input)

CRC_INIT
CRC_CR

CRC computation
CRC_POL
CRC_IDR

MS19882V3

24.3.2

CRC internal signals
Table 195. CRC internal input/output signals

24.3.3

Signal name

Signal type

crc_hclk

Digital input

Description
AHB clock

CRC operation
The CRC calculation unit has a single 32-bit read/write data register (CRC_DR). It is used to
input new data (write access), and holds the result of the previous CRC calculation (read
access).
Each write operation to the data register creates a combination of the previous CRC value
(stored in CRC_DR) and the new one. CRC computation is done on the whole 32-bit data
word or byte by byte depending on the format of the data being written.
The CRC_DR register can be accessed by word, right-aligned half-word and right-aligned
byte. For the other registers only 32-bit accesses are allowed.
The duration of the computation depends on data width:
•

4 AHB clock cycles for 32 bits

•

2 AHB clock cycles for 16 bits

•

1 AHB clock cycles for 8 bits

An input buffer allows a second data to be immediately written without waiting for any
wait-states due to the previous CRC calculation.

RM0456 Rev 6

<!-- pagebreak -->

942

Cyclic redundancy check calculation unit (CRC)

RM0456

The data size can be dynamically adjusted to minimize the number of write accesses for a
given number of bytes. For instance, a CRC for 5 bytes can be computed with a word write
followed by a byte write.
The input data can be reversed to manage the various endianness schemes. The reversing
operation can be performed on 8 bits, 16 bits and 32 bits depending on the REV_IN[1:0] bits
in the CRC_CR register.
For example, 0x1A2B3C4D input data are used for CRC calculation as:
•

0x58D43CB2 with bit-reversal done by byte

•

0xD458B23C with bit-reversal done by half-word

•

0xB23CD458 with bit-reversal done on the full word

The output data can also be reversed by setting the REV_OUT bit in the CRC_CR register.
The operation is done at bit level. For example, 0x11223344 output data are converted to
0x22CC4488.
The CRC calculator can be initialized to a programmable value using the RESET control bit
in the CRC_CR register (the default value is 0xFFFFFFFF).
The initial CRC value can be programmed with the CRC_INIT register. The CRC_DR
register is automatically initialized upon CRC_INIT register write access.
The CRC_IDR register can be used to hold a temporary value related to CRC calculation. It
is not affected by the RESET bit in the CRC_CR register.

Polynomial programmability
The polynomial coefficients are fully programmable through the CRC_POL register, and the
polynomial size can be configured to be 7, 8, 16 or 32 bits by programming the
POLYSIZE[1:0] bits in the CRC_CR register. Even polynomials are not supported.
Note:

The type of an even polynomial is X+X2+..+Xn, while the type of an odd polynomial is
1+X+X2+..+Xn.
If the CRC data is less than 32-bit, its value can be read from the least significant bits of the
CRC_DR register.
To obtain a reliable CRC calculation, the change on-fly of the polynomial value or size can
not be performed during a CRC calculation. As a result, if a CRC calculation is ongoing, the
application must either reset it or perform a CRC_DR read before changing the polynomial.
The default polynomial value is the CRC-32 (Ethernet) polynomial: 0x4C11DB7.

<!-- pagebreak -->

