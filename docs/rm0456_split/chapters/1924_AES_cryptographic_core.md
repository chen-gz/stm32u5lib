1973

AES hardware accelerator (AES)

RM0456

Table 469. AES internal input/output signals
Signal name

Signal type

aes_hclk

Input

AHB bus clock

aes_it

Output

AES interrupt request

aes_in_dma

Input/Output

Input DMA single request/acknowledge

aes_out_dma

Input/Output

Output DMA single request/acknowledge

aes_itamp_out

49.4.3

Description

Tamper event signal to TAMP (XOR-ed), triggered when an
unexpected hardware fault occurs. When this signal is
triggered, AES automatically clears key registers. A reset is
required for AES to be usable again.

Output

AES cryptographic core
Overview
The AES cryptographic core consists of the following components:
•

AES core algorithm (AEA)

•

multiplier over a binary Galois field (GF2mul)

•

key input

•

initialization vector (IV) input

•

chaining algorithm logic (XOR, feedback/counter, mask)

The AES core works on 128-bit data blocks (four words) with 128-bit or 256-bit key length.
Depending on the chaining mode, the AES requires zero or one 128-bit initialization vector
IV.
The AES features the following modes of operation:
•

Mode 1:
Plaintext encryption using a key stored in the AES_KEYRx registers

•

Mode 2:
ECB or CBC decryption key preparation. It must be used prior to selecting Mode 3 with
ECB or CBC chaining modes. The key prepared for decryption is stored automatically
in the AES_KEYRx registers. Now the AES peripheral is ready to switch to Mode 3 for
executing data decryption.

•

Mode 3:
Ciphertext decryption using a key stored in the AES_KEYRx registers. When ECB and
CBC chaining modes are selected, the key must be prepared beforehand, through
Mode 2.

Note:

Mode 2 is only used when performing ECB and CBC decryption.
The operating mode is selected by programming the MODE[1:0] bitfield of the AES_CR
register. It may be done only when the AES peripheral is disabled.
Special key operation is selected using the KMOD[1:0] bitfield of the AES_CR register. See
Section 49.4.13 for details.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

AES hardware accelerator (AES)

Typical data processing
Typical usage of the AES is described in Section 49.4.4: AES procedure to perform a cipher
operation on page 1929.
Note:

The outputs of the intermediate AEA stages are never revealed outside the cryptographic
boundary, with the exclusion of the IVI bitfield.

Chaining modes
The following chaining modes are supported by AES, selected through the CHMOD[2:0]
bitfield of the AES_CR register:

Note:

•

Electronic code book (ECB)

•

Cipher block chaining (CBC)

•

Counter (CTR)

•

Galois counter mode (GCM)

•

Galois message authentication code (GMAC)

•

Counter with CBC-MAC (CCM)

The chaining mode may be changed only when AES is disabled (bit EN of the AES_CR
register cleared).
Principle of each AES chaining mode is provided in the following subsections.
Detailed information is in dedicated sections, starting from Section 49.4.8: AES basic
chaining modes (ECB, CBC).

Electronic codebook (ECB) mode
Figure 460. ECB encryption and decryption principle
Encryption
Plaintext block 1

key

Encrypt

Ciphertext block 1

Plaintext block 2

key

Encrypt

Ciphertext block 2

Plaintext block 3

key

Encrypt

Ciphertext block 3

Decryption
Plaintext block 1

Legend

key

Decrypt

Plaintext block 2

key

Decrypt

Plaintext block 3

key

Decrypt

input
output
key
scheduling

Ciphertext block 1

Ciphertext block 2

Ciphertext block 3
MSv42140V1

RM0456 Rev 6

<!-- pagebreak -->

1973

AES hardware accelerator (AES)

RM0456

ECB is the simplest mode of operation. There are no chaining operations, and no special
initialization stage. The message is divided into blocks and each block is encrypted or
decrypted separately.
Note:

For decryption, a special key scheduling is required before processing the first block.

Cipher block chaining (CBC) mode
Figure 461. CBC encryption and decryption principle
Encryption
Plaintext block 1

Plaintext block 2

Plaintext block 3

initialization
vector
key

Encrypt

Ciphertext block 1

key

Encrypt

Ciphertext block 2

key

Encrypt

Ciphertext block 3

Decryption
Plaintext block 1

Plaintext block 2

Plaintext block 3

initialization
vector

Legend

key

Decrypt

key

Decrypt

key

Decrypt

input
output
key
scheduling

Ciphertext block 1

Ciphertext block 2

Ciphertext block 3
MSv42141V1

In CBC mode the output of each block chains with the input of the following block. To make
each message unique, an initialization vector is used during the first block processing.
Note:

<!-- pagebreak -->

For decryption, a special key scheduling is required before processing the first block.

RM0456 Rev 6

RM0456

AES hardware accelerator (AES)

Counter (CTR) mode
Figure 462. CTR encryption and decryption principle
Encryption
Counter

Counter

+1

value

key

value + 1

key

Encrypt

Plaintext block 1

Counter

+1

key

Encrypt

Plaintext block 2

Ciphertext block 1

value + 2

Encrypt

Plaintext block 3

Ciphertext block 2

Ciphertext block 3

Decryption
Counter

Counter

+1

value

Legend

key

value + 1

key

Decrypt

Counter

+1

value + 2

key

Decrypt

Decrypt

input
output
XOR

Plaintext block 1

Plaintext block 2

Ciphertext block 1

Plaintext block 3

Ciphertext block 2

Ciphertext block 3
MSv42142V1

The CTR mode uses the AES core to generate a key stream. The keys are then XOR-ed
with the plaintext to obtain the ciphertext as specified in NIST Special Publication 800-38A,
Recommendation for Block Cipher Modes of Operation.
Note:

Unlike with ECB and CBC modes, no key scheduling is required for the CTR decryption,
since in this chaining scheme the AES core is always used in encryption mode for producing
the key stream, or counter blocks.

RM0456 Rev 6

<!-- pagebreak -->

1973

AES hardware accelerator (AES)

RM0456

Galois/counter mode (GCM)
Figure 463. GCM encryption and authentication principle
Initialization
vector

+1

Counter
value

key

key

Init
(Encrypt)

Plaintext block 1
H

Counter

value + 1

key

Encrypt

+1

Counter

key

Encrypt

Plaintext block 2

value + 2

Encrypt

Plaintext block 3

Ciphertext block 1

Ciphertext block 2

Ciphertext block 3

GF2mul

GF2mul

GF2mul

Legend
input
output
XOR

Final

TAG
MSv42143V1

In Galois/counter mode (GCM), the plaintext message is encrypted while a message
authentication code (MAC) is computed in parallel, thus generating the corresponding
ciphertext and its MAC (also known as authentication tag). It is defined in NIST Special
Publication 800-38D, Recommendation for Block Cipher Modes of Operation Galois/Counter Mode (GCM) and GMAC.
GCM mode is based on AES in counter mode for confidentiality. It uses a multiplier over a
fixed finite field for computing the message authentication code. It requires an initial value
and a particular 128-bit block at the end of the message.

Galois message authentication code (GMAC) principle
Figure 464. GMAC authentication principle
Initialization
vector

key

Legend

Init
(Encrypt)

H

Plaintext block 1

Plaintext block 2

Plaintext block 3

GF2mul

GF2mul

GF2mul

input
output

Final

TAG

XOR
MSv42144V1

Galois message authentication code (GMAC) allows authenticating a message and
generating the corresponding message authentication code (MAC). It is defined in NIST
Special Publication 800-38D, Recommendation for Block Cipher Modes of Operation Galois/Counter Mode (GCM) and GMAC.

<!-- pagebreak -->

