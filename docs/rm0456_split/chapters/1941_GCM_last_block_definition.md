RM0456 Rev 6

RM0456

AES hardware accelerator (AES)
Figure 476. Message construction in GCM
[Len(A)]64 [Len(C)]64
Len(A)

Len(P) = Len(C)

Initialization vector (IV)

Counter

0

Last
block

0

encrypt

4-byte boundaries

Plaintext (P)

0

Authenticated & encrypted ciphertext (C)

auth.

Additional authenticated data
(AAD)

authenticate

ICB

authenticate

16-byte
boundaries

Authentication tag (T)
Zero padding / zeroed bits
MSv42157V1

The message has the following structure:
•

16-byte initial counter block (ICB), composed of two distinct fields:
–

Initialization vector (IV): a 96-bit value that must be unique for each encryption
cycle with a given key. Note that the GCM standard supports IVs with less than 96
bits, but in this case strict rules apply.

–

Counter: a 32-bit big-endian integer that is incremented each time a block
processing is completed. According to NIST specification, the counter value is 0x2
when processing the first block of payload.

•

Authenticated header AAD (also knows as additional authentication data) has a
known length Len(A) that may be a non-multiple of 16 bytes, and must not exceed
264 – 1 bits. This part of the message is only authenticated, not encrypted.

•

Plaintext message P is both authenticated and encrypted as ciphertext C, with a
known length Len(P) that may be non-multiple of 16 bytes, and cannot exceed 232 - 2
128-bit blocks.

•

Last block contains the AAD header length (bits [32:63]) and the payload length (bits
[96:127]) information, as shown in Table 471.

The GCM standard specifies that ciphertext C has the same bit length as the plaintext P.
When a part of the message (AAD or P) has a length that is a non-multiple of 16-bytes a
special padding scheme is required.
Table 471. GCM last block definition
Endianness

Bit[0] ---------- Bit[31]

Bit[32]---------- Bit[63]

Bit[64] -------- Bit[95]

Bit[96] --------- Bit[127]

Input data

0x0

AAD length[31:0]

0x0

Payload length[31:0]

RM0456 Rev 6

<!-- pagebreak -->

