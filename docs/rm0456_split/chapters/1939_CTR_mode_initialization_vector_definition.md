A 16-byte initial counter block (ICB), composed of two distinct fields:
–

Initialization vector (IV): a 96-bit value that must be unique for each encryption
cycle with a given key.

–

Counter: a 32-bit big-endian integer that is incremented each time a block
processing is completed. The initial value of the counter must be set to 1.

The plaintext P is encrypted as ciphertext C, with a known length. This length can be
non-multiple of 16 bytes, in which case a plaintext padding is required.

RM0456 Rev 6

RM0456

AES hardware accelerator (AES)

CTR encryption and decryption
Figure 474 and Figure 475 describe the CTR encryption and decryption process,
respectively, as implemented in the AES peripheral. The CTR mode is selected by writing
010 to the CHMOD[2:0] bitfield of AES_CR register.
Figure 474. CTR encryption
Block 1

Block 2

AES_IVRx

AES_IVRx

Nonce + 32-bit counter

Nonce + 32-bit counter (+1)

Counter
increment (+1)

I1
AES_KEYRx (KEY)

I2
AES_KEYRx (KEY)

Encrypt
AES_DINR (plaintext P1)

Swap
management

O1

DATATYPE[1:0]

Encrypt
AES_DINR (plaintext P2)

Swap
management

DATATYPE[1:0]

P1'

Legend
DATATYPE[1:0]

input

O2

P2'

C1'
Swap
management

DATATYPE[1:0]

C2'
Swap
management

output
AES_DOUTR (ciphertext C1)

XOR

AES_DOUTR (ciphertext C2)
MSv19102V3

Figure 475. CTR decryption
Block 1

Block 2

AES_IVRx

AES_IVRx

Nonce + 32-bit counter

Nonce + 32-bit counter (+1)

Counter
increment (+1)
I1

I2
AES_KEYRx (KEY)

AES_KEYRx (KEY)
Encrypt
AES_DINR (ciphertext C1)

Swap
management

O1

DATATYPE[1:0]

Encrypt
AES_DINR (ciphertext C2)

Swap
management

DATATYPE[1:0]

C1'

Legend
input

DATATYPE[1:0]

P1'
Swap
management

O2

C2'
DATATYPE[1:0]

P2'
Swap
management

output
AES_DOUTR (plaintext P1)

AES_DOUTR (plaintext P2)

XOR

MSv18942V2

In CTR mode, the cryptographic core output (also called keystream) Ox is XOR-ed with
relevant input block (Px' for encryption, Cx' for decryption), to produce the correct output
block (Cx' for encryption, Px' for decryption). Initialization vectors in AES must be initialized
as shown in Table 470.
Table 470. CTR mode initialization vector definition
AES_IVR3[31:0]

AES_IVR2[31:0]

AES_IVR1[31:0]

AES_IVR0[31:0]

IVI[127:96]

IVI[95:64]

IVI[63:32]

IVI[31:0}
32-bit counter = 0x0001

RM0456 Rev 6

<!-- pagebreak -->

1973

AES hardware accelerator (AES)

RM0456

Unlike in CBC mode that uses the AES_IVRx registers only once when processing the first
data block, in CTR mode AES_IVRx registers are used for processing each data block, and
the AES peripheral increments the counter bits of the initialization vector (leaving the nonce
bits unchanged).
CTR decryption does not differ from CTR encryption, since the core always encrypts the
current counter block to produce the key stream that is then XOR-ed with the plaintext (CTR
encryption) or ciphertext (CTR decryption) input. In CTR mode, the MODE[1:0] bitfield
setting 01 (key derivation) is forbidden and all the other settings default to encryption mode.
The sequence of events to perform an encryption or a decryption in CTR chaining mode:
1.
2.

Disable the AES peripheral by clearing the EN bit of the AES_CR register.
Select CTR chaining mode by setting to 010 the CHMOD[2:0] bitfield of the AES_CR
register. Set MODE[1:0] bitfield to any value other than 01.

3.

Initialize the AES_KEYRx registers, and load the AES_IVRx registers as described in
Table 470.

4.

Set the EN bit of the AES_CR register, to start encrypting the current counter (EN is
automatically reset when the calculation finishes).

5.

If it is the last block, pad the data with zeros to have a complete block, if needed.

6.

Append data in AES, and read the result. The three possible scenarios are described in
Section 49.4.4: AES procedure to perform a cipher operation.

7.

Repeat the previous step till the second-last block is processed. For the last block,
apply the two previous steps and discard the bits that are not part of the payload (if the
size of the significant data in the last input block is less than 16 bytes).

Suspend/resume operations in CTR mode
Like for the CBC mode, it is possible to interrupt a message to send a higher priority
message, and resume the message that was interrupted. Detailed CBC suspend/resume
sequence is described in Section 49.4.8: AES basic chaining modes (ECB, CBC).
Note:

Like for CBC mode, the AES_IVRx registers must be reloaded during the resume operation.

49.4.10

AES Galois/counter mode (GCM)
Overview
The AES Galois/counter mode (GCM) allows encrypting and authenticating a plaintext
message into the corresponding ciphertext and tag (also known as message authentication
code). To ensure confidentiality, GCM algorithm is based on AES counter mode. It uses a
multiplier over a fixed finite field to generate the tag.
GCM chaining is defined in NIST Special Publication 800-38D, Recommendation for Block
Cipher Modes of Operation - Galois/Counter Mode (GCM) and GMAC. A typical message
construction in GCM mode is given in Figure 476.

<!-- pagebreak -->

