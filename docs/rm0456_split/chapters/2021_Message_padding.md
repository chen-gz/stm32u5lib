RM0456 Rev 6

RM0456

Hash processor (HASH)
in HASH_STR register must be written prior to enabling the DMA (see
Section 51.4.6 for details).

4.

c)

When data are filled by DMA using multiple DMA transfers (MDMAT bit = 1):

–

Partial digest computations are triggered as for single DMA transfers. However the
final digest computation is not triggered automatically when the last block has
been transferred by DMA to the HASH_DIN register (DCAL bit is not set to 1 by
hardware). It allows the hash processor to receive a new DMA transfer as part of
this digest computation. To launch the final digest computation, the software must
set MDMAT bit to 0 before the last DMA transfer in order to trigger the final digest
computation as it is done for single DMA transfers (see description before).

Once the digest computation is complete (DCIS = 1), the resulting digest can be read
from the output registers as described in Table 486.
Table 486. Hash processor outputs
Algorithm

Valid output registers

Most significant bit Digest size (in bits)

MD5

HASH_H0 to HASH_H3

HASH_H0[31]

128

SHA-1

HASH_H0 to HASH_H4

HASH_H0[31]

160

SHA2-224

HASH_H0 to HASH_H6

SHA2-256

HASH_H0 to HASH_H7

HASH_H0[31]

224
256

For more information about HMAC detailed instructions, refer to Section 51.4.7: HMAC
operation.

51.4.6

Message padding
Overview
When computing a condensed representation of a message, the process of feeding data
into the hash processor (with automatic partial digest computation every block transfer)
loops until the last bits of the original message are written to the HASH_DIN register.
As the length (number of bits) of a message can be any integer value, the last word written
to the hash processor may have a valid number of bits between 1 and 32. This number of
valid bits in the last word, NBLW[4:0], has to be written to the HASH_STR register, so that
message padding is correctly performed before the final message digest computation.

Padding processing
Detailed padding sequences with DMA enabled or disabled are described in Section 51.4.5:
Message digest computing.

Padding example
As specified by Federal Information Processing Standards PUB 180-4, the message
padding consists in appending a “1” followed by k “0”s, itself followed by a 64-bit integer that
is equal to the length L in bits of the message. These three padding operations generate a
padded message of length L + 1 + k + 64, which by construction is a multiple of 512 bits.
For the hash processor, the “1” is added to the last word written to the HASH_DIN register at
the bit position defined by the NBLW[4:0] bitfield, and the remaining upper bits are cleared
(“0”s).

RM0456 Rev 6

<!-- pagebreak -->

2037

Hash processor (HASH)

RM0456

Example from FIPS PUB180-4
Let us assume that the original message is the ASCII binary-coded form of “abc”, of length
L = 24:
byte 0
byte 1
byte 2
byte 3
01100001 01100010 01100011 UUUUUUUU
<-- 1st word written to HASH_DIN -->

NBLW[4:0] has to be loaded with the value 24: a “1” is appended at bit location 24 in the bit
string (starting counting from left to right in the above bit string), which corresponds to bit 31
in the HASH_DIN register (little-endian convention):
01100001 01100010 01100011 1UUUUUUU

Since L = 24, the number of bits in the above bit string is 25, and 423 “0” bits are appended,
making now 448 bits.
This gives in hexadecimal (byte words in big-endian format):
61626380 00000000 00000000 00000000
00000000 00000000 00000000 00000000
00000000 00000000 00000000 00000000
00000000 00000000 00000000 00000018

The message length value, L, in two-word format (that is 00000000 00000018) is appended.
Hence the final padded message in hexadecimal (byte words in big-endian format):
61626380 00000000 00000000 00000000
00000000 00000000 00000000 00000000
00000000 00000000 00000000 00000000
00000000 00000000 00000000 00000018

If the hash processor is programmed to swap byte within HASH_DIN input register
(DATATYPE = 10 in HASH_CR), the above message has to be entered by following the
below sequence:
1.

0xUU636261 is written to the HASH_DIN register (where ‘U’ means don’t care).

2.

0x18 is written to the HASH_STR register (the number of valid bits in the last word
written to the HASH_DIN register is 24, as the original message length is 24 bits).

3.

0x10 is written to the HASH_STR register to start the message padding (described
above) and then perform the digest computation.

4.

The hash computing is complete with the message digest available in the HASH_HRx
registers (x = 0...4) for the SHA-1 algorithm. For this FIPS example, the expected value
is as follows:
HASH_HR0 = 0xA9993E36
HASH_HR1 = 0x4706816A
HASH_HR2 = 0xBA3E2571
HASH_HR3 = 0x7850C26C
HASH_HR4 = 0x9CD0D89D

<!-- pagebreak -->

