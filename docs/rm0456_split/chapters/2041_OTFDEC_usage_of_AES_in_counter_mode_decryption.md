RM0456 Rev 6

RM0456

On-the-fly decryption engine (OTFDEC)
OTFDEC_RxKEYR registers, where x = 1 to 4. In OTFDEC_RxCFGR, the MODE bits
define the OTFDEC operating mode (standard or enhanced encryption).
Granularity for the region determination is 4096 bytes.

Note:

Although OTFDEC does not prevent region overlapping, it is not a valid programming and it
must be avoided by application software.
OTFDEC can decrypt incremental or wrap bursts only if they do not cross the 4096-byte
aligned address boundaries.

52.3.4

OTFDEC usage of AES in counter mode decryption
Figure 506 shows how OTFDEC uses industry standard Advanced Encryption Standard
(AES) algorithm in counter chaining mode. This mode is specified by NIST in Special
Publication 800-38A, Recommendation for Block Cipher Modes of Operation.
Figure 506. AES CTR decryption flow
AES_IV

AES_IV

Nonce Version Id Address_0

Nonce Version Id Address_1

128-bit

AES_KEY

AES_KEY
AES Block cipher
encryption

AES Block cipher
encryption

128-bit

128-bit

Keystream_0

AES_DIN (cipher text 0)

Keystream_1
128-bit

AES_DIN (cipher text 1)

128-bit

128-bit

AES_DOUT (plain text 0)

AES_DOUT (plain text 1)
MS48969V1

Every 128-bit data block, a special keystream information is computed using AES block
cipher, as defined below:

Note:

•

initialization vector AES_IV[127:0] = RxNONCER1[31:0] || RxNONCER0[31:0] ||
0b0000 0000 0000 0000 || RxCFGR[31:16] || 0b00 || (x-1) || ReadAddress[31:4]

•

key material AES_KEY[127:0] = RxKEYR3[31:0] || RxKEYR2[31:0] || RxKEYR1[31:0] ||
RxKEYR0[31:0]

Above x is the RegionID of the selected encrypted region (x=1 to 4).
ReadAddress is the AHB address of the encrypted data block, modulo 128-bit.
Resulting 128-bit keystream is XORed with 128-bit cipher text data to produce the 128-bit
clear text data.
•

AES_DIN and AES_DOUT data blocks are constructed following the rule below (“|”
represents a binary concatenation):
AES_Dx[127:0]= AHB_word(@ | 0xC)[31:0] | AHB_word(@ | 0x8)[31:0] | AHB_word(@
| 0x4)[31:0] || AHB_word(@ | 0x0)[31:0], where @ is the hexadecimal address used to
compute the keystream (ReadAddress[31:4] above).

RM0456 Rev 6

<!-- pagebreak -->

