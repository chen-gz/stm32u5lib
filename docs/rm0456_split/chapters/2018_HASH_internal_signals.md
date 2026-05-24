2037

Hash processor (HASH)

51.4.2

RM0456

HASH internal signals
Table 485 describes a list of useful to know internal signals available at HASH level, not at
product level (on pads).
Table 485. HASH internal input/output signals

51.4.3

Signal name

Signal type

Description

hash_hclk

digital input

AHB bus clock

hash_it

digital output

Hash processor global interrupt request

hash_dma

digital input/output

DMA transfer request/ acknowledge

About secure hash algorithms
The hash processor is a fully compliant implementation of the secure hash algorithm
defined by FIPS PUB 180-4 standard and the IETF RFC1321 publication (MD5).
With each algorithm, the HASH computes a condensed representation of a message or data
file. More specifically, when a message of any length below 264 bits is provided on input, the
HASH processing core produces respectively a fixed-length output string called a message
digest, defined as follows:
•

For MD5 digest size is 128-bit

•

For SHA-1 digest size is 160-bit

•

For SHA2-224 and SHA2-256, the digest size is 224 bits and 256 bits, respectively

The message digest can then be processed with a digital signature algorithm in order to
generate or verify the signature for the message.
Signing the message digest rather than the message often improves the efficiency of the
process because the message digest is usually much smaller in size than the message. The
verifier of a digital signature has to use the same hash algorithm as the one used by the
creator of the digital signature.
The SHA-2 functions supported by the hash processor are qualified as “secure” by NIST
because it is computationally infeasible to find a message that corresponds to a given
message digest, or to find two different messages that produce the same message digest
(SHA-1 does not qualify as secure since February 2017). Any change to a message in
transit, with very high probability, results in a different message digest, and the signature
fails to verify.

51.4.4

Message data feeding
The message (or data file) to be processed by the HASH is considered as a bit string.
Per FIPS PUB 180-4 standard this message bit string grows from left to right, with
hexadecimal words expressed in “big-endian” convention, so that within each word, the
most significant bit is stored in the left-most bit position. For example message string “abc”
with a bit string representation of “01100001 01100010 01100011” is represented by a
32-bit word 0x00636261, and 8-bit words 0x61626300.
Data are entered into the HASH one 32-bit word at a time, by writing them into the
HASH_DIN register. The current contents of the HASH_DIN register are transferred to the
16 words input FIFO each time the register is written with new data. Hence HASH_DIN and
the FIFO form a seventeen 32-bit words length FIFO (named the IN buffer).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hash processor (HASH)
In accordance to the kind of data to be processed (e.g. byte swapping when data are ASCII
text stream) there must be a bit, byte, half-word or no swapping operation to be performed
on data from the input FIFO before entering the little-endian hash processing core.
Figure 502 shows how the hash processing core 32-bit data block M0...31 is constructed
from one 32-bit words popped into input FIFO by the driver, according to the DATATYPE
bitfield in the HASH control register (HASH_CR).
HASH_DIN data endianness when bit swapping is disabled (DATATYPE = 00) can be
described as following: the least significant bit of the message has to be at MSB position in
the first word entered into the hash processor, the 32nd bit of the bit string has to be at MSB
position in the second word entered into the hash processor and so on.
Figure 502. Message data swapping feature

DATATYPE “00”: no swapping
written first!
LSB

MSB

System interface

Word0

Word2

Word1

bit31

bit0

bit31

M0

M31

M32

bit0

bit31

M64
M63
HASH core interface

LSB

Word3
bit0

bit31

bit0

M95

M96

M127
MSB

DATATYPE “01”: 16-bit or half-word swapping
written first!

LSB
bit31

M0
LSB

bit0

bit31

Word1
bit16 bit15

M15

M31

M32

M47

M16

M48

DATATYPE “10”: 8-bit or byte swapping
written first!
LSB
Word0

M0..7
LSB

bit0

bit31

bit7..0

Word2
bit16 bit15

bit0

M63
HASH core interface

bit31

Word3
bit16 bit15

M96

M111

M112

Word2

bit31..24 bit23..16 bit15..8

bit7..0

bit31..24 bit23..16 bit15..8

M127
MSB

Word3
bit7..0

bit31..24 bit23..16 bit15..8

M96..103

M8..15 M16..23 M24..31

bit0

MSB

System interface
Word1

bit31..24 bit23..16 bit15..8

MSB

System interface

Word0
bit16 bit15

HASH core interface

bit7..0

M112..119 M120..127
MSB

DATATYPE “11”: bit swapping
LSB

written first!

Word0
bit31 bit30bit29

Word1
bit2 bit1 bit0 bit31 bit30 bit29

M0 M1 M2
LSB

M29 M30 M31 M32 M33 M34

System interface
Word2
bit2 bit1 bit0

M61 M62 M63
HASH core interface

MSB
Word3
bit31bit30 bit29

M96 M97 M98

bit2 bit1 bit0

M125M126M127
MSB

MSv41984V4

RM0456 Rev 6

<!-- pagebreak -->

