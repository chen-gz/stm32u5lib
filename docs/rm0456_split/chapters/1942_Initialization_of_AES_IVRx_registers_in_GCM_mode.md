1973

AES hardware accelerator (AES)

RM0456

GCM processing
Figure 477 describes the GCM implementation in the AES peripheral. The GCM is selected
by writing 011 to the CHMOD[2:0] bitfield of the AES_CR register.
Figure 477. GCM authenticated encryption
(3) Payload

Block 1
AES_IVRx
ICB + (32-bit counter = 0x02)

(1) Init
AES_KEYRx (KEY)

Block n
AES_IVRx

CBn

Counter
increment (+1)

CB1
[0]128

CBn

AES_KEYRx (KEY)

AES_KEYRx (KEY)
Encrypt

Encrypt

AES_DINR (plaintext P1)

H

Swap
management

Swap
management

AES_DINR (plaintext Pn)

DATATYPE
[1:0]

DATATYPE
[1:0]

(2) Header
AES_DINR (AAD 0)

GF2mul

Swap
management

Swap
management

DATATYPE
[1:0]

Swap
management

DATATYPE[1:0]

AES_DINR (AAD i)

DATATYPE
[1:0]

Swap
management

H

GF2mul

AES_DOUTR
(ciphertext Cn)

AES_DOUTR
(ciphertext C1)

H
H

Encrypt

GF2mul

(4) Final

GF2mul

H

GF2mul

AES_DINR
Len(A)64 || Len(C)64

AES_IVRx
(IV + 32-bit counter (= 0x1))

Legend

H

S

input

Encrypt

output
XOR
AES_KEYRx (key)

AES_DOUTR
(Authentication TAG T)

The mechanism for the confidentiality of the plaintext in GCM mode is similar to that in the
Counter mode, with a particular increment function (denoted 32-bit increment) that
generates the sequence of input counter blocks.
AES_IVRx registers keeping the counter block of data are used for processing each data
block. The AES peripheral automatically increments the Counter[31:0] bitfield. The first
counter block (CB1) is derived from the initial counter block ICB by the application software
(see Table 472).
Table 472. Initialization of AES_IVRx registers in GCM mode

Note:

<!-- pagebreak -->

AES_IVR3[31:0]

AES_IVR2[31:0]

AES_IVR1[31:0]

AES_IVR0[31:0]

ICB[127:96]

ICB[95:64]

ICB[63:32]

ICB[31:0]
32-bit counter = 0x0002

In this mode, the setting 01 of the MODE[1:0] bitfield (key derivation) is forbidden.

RM0456 Rev 6

RM0456

AES hardware accelerator (AES)
The authentication mechanism in GCM mode is based on a hash function called GF2mul
that performs multiplication by a fixed parameter, called hash subkey (H), within a binary
Galois field.
A GCM message is processed through the following phases, further described in next
subsections:
•

Init phase: AES prepares the GCM hash subkey (H).

•

Header phase: AES processes the additional authenticated data (AAD), with hash
computation only.

•

Payload phase: AES processes the plaintext (P) with hash computation, counter block
encryption and data XOR-ing. It operates in a similar way for ciphertext (C).

•

Final phase: AES generates the authenticated tag (T) using the last block of the
message.

GCM init phase
During this first step, the GCM hash subkey (H) is calculated and saved internally, to be
used for processing all the blocks. The recommended sequence is:
1.
2.

Disable the AES peripheral by clearing the EN bit of the AES_CR register.
Select GCM chaining mode, by setting to 011 the CHMOD[2:0] bitfield of the AES_CR
register, and optionally, set the DATATYPE[1:0] bitfield.

3.

Indicate the Init phase, by setting to 00 the GCMPH[1:0] bitfield of the AES_CR
register.

4.

Set the MODE[1:0] bitfield of the AES_CR register to 00 or 10. Although the bitfield is
only used in payload phase, it is recommended to set it in the Init phase and keep it
unchanged in all subsequent phases.

5.

Initialize the AES_KEYRx registers with a key, and initialize AES_IVRx registers with
the information as defined in Table 472.

6.

Start the calculation of the hash key, by setting to 1 the EN bit of the AES_CR register
(EN is automatically reset when the calculation finishes).

7.

Wait until the end of computation, indicated by the CCF flag of the AES_SR transiting
to 1. Alternatively, use the corresponding interrupt.

8.

Clear the CCF flag of the AES_SR register, by setting the CCF bit of the AES_ICR
register.

GCM header phase
This phase coming after the GCM Init phase must be completed before the payload phase.
The sequence to execute, identical for encryption and decryption, is:

Note:

1.

Indicate the header phase, by setting to 01 the GCMPH[1:0] bitfield of the AES_CR
register. Do not modify the MODE[1:0] bitfield as set in the Init phase.

2.

Enable the AES peripheral by setting the EN bit of the AES_CR register.

3.

If it is the last block and the AAD size in the block is inferior to 128 bits, pad the
remainder of the block with zeros. Then append the data block into AES in one of ways
described in Section 49.4.4: AES procedure to perform a cipher operation. No data is
read during this phase.

4.

Repeat the step 3 until the last additional authenticated data block is processed.

The header phase can be skipped if there is no AAD, that is, Len(A) = 0.

RM0456 Rev 6

<!-- pagebreak -->

1973

AES hardware accelerator (AES)

RM0456

GCM payload phase
This phase, identical for encryption and decryption, is executed after the GCM header
phase. During this phase, the encrypted/decrypted payload is stored in the AES_DOUTR
register. The sequence to execute is:

Note:

1.

Indicate the payload phase, by setting to 10 the GCMPH[1:0] bitfield of the AES_CR
register. Do not modify the MODE[1:0] bitfield as set in the Init phase.

2.

If the header phase was skipped, enable the AES peripheral by setting the EN bit of the
AES_CR register.

3.

If it is the last block and the plaintext (encryption) or ciphertext (decryption) size in the
block is inferior to 128 bits, pad the remainder of the block with zeros.

4.

Append the data block into AES in one of ways described in Section 49.4.4: AES
procedure to perform a cipher operation on page 1929, and read the result.

5.

Repeat the previous step till the second-last plaintext block is encrypted or till the last
block of ciphertext is decrypted. For the last block of plaintext (encryption only),
execute the two previous steps. For the last block, discard the bits that are not part of
the payload when the last block size is less than 16 bytes.

The payload phase can be skipped if there is no payload data, that is, Len(C) = 0 (see
GMAC mode).
GCM final phase
In this last phase, the AES peripheral generates the GCM authentication tag and stores it in
the AES_DOUTR register. The sequence to execute is:

Note:

1.

Indicate the final phase, by setting to 11 the GCMPH[1:0] bitfield of the AES_CR
register.

2.

Compose the data of the block, by concatenating the AAD bit length and the payload
bit length, as shown in Table 471. Write the block into the AES_DINR register.

3.

Wait until the end of computation, indicated by the CCF flag of the AES_SR transiting
to 1.

4.

Get the GCM authentication tag, by reading the AES_DOUTR register four times.

5.

Clear the CCF flag of the AES_SR register, by setting the CCF bit of the AES_ICR
register.

6.

Disable the AES peripheral, by clearing the bit EN of the AES_CR register. If it is an
authenticated decryption, compare the generated tag with the expected tag passed
with the message.

In the final phase, data is written to AES_DINR normally (no swapping), while swapping is
applied to tag data read from AES_DOUTR.
When transiting from the header or the payload phase to the final phase, the AES peripheral
must not be disabled, otherwise the result is wrong.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

AES hardware accelerator (AES)

Suspend/resume operations in GCM mode
To suspend the processing of a message, proceed as follows:
1.

If DMA is used, stop the AES DMA transfers to the IN FIFO by clearing the DMAINEN
bit of the AES_CR register. If DMA is not used, make sure that the current computation
is completed, which is indicated by the CCF flag of the AES_SR register set to 1.

2.

In the payload phase, if DMA is not used, read four times the AES_DOUTR register to
save the last-processed block. If DMA is used, wait until the CCF flag is set in the
AES_SR register then stop the DMA transfers from the OUT FIFO by clearing the
DMAOUTEN bit of the AES_CR register.

3.

Clear the CCF flag of the AES_SR register, by setting the CCF bit of the AES_ICR
register.

4.

Save the AES_SUSPxR registers in the memory, where x is from 0 to 7.

5.

In the payload phase, save the AES_IVRx registers as, during the data processing,
they changed from their initial values. In the header phase, this step is not required.

6.

Disable the AES peripheral, by clearing the EN bit of the AES_CR register.

7.

Save the current AES configuration in the memory, excluding the initialization vector
registers AES_IVRx. Key registers do not need to be saved as the original key value is
known by the application.

8.

If DMA is used, save the DMA controller status (pointers for IN data transfers, number
of remaining bytes, and so on). In the payload phase, pointers for OUT data transfers
must also be saved.

To resume the processing of a message, proceed as follows:
1.

If DMA is used, configure the DMA controller in order to complete the rest of the FIFO
IN transfers. In the payload phase, the rest of the FIFO OUT transfers must also be
configured in the DMA controller.

2.

Disable the AES peripheral by clearing the EN bit of the AES_CR register.

3.

Write the suspend register values, previously saved in the memory, back into their
corresponding AES_SUSPxR registers, where x is from 0 to 7.

4.

In the payload phase, write the initialization vector register values, previously saved in
the memory, back into their corresponding AES_IVRx registers. In the header phase,
write initial setting values back into the AES_IVRx registers.

5.

Restore the initial setting values in the AES_CR and AES_KEYRx registers.

6.

Enable the AES peripheral by setting the EN bit of the AES_CR register.

If DMA is used, enable AES DMA requests by setting the DMAINEN bit (and DMAOUTEN
bit if in payload phase) of the AES_CR register.

49.4.11

AES Galois message authentication code (GMAC)
Overview
The Galois message authentication code (GMAC) allows the authentication of a plaintext,
generating the corresponding tag information (also known as message authentication
code). It is based on GCM algorithm, as defined in NIST Special Publication 800-38D,
Recommendation for Block Cipher Modes of Operation - Galois/Counter Mode (GCM) and
GMAC.

RM0456 Rev 6

<!-- pagebreak -->

1973

AES hardware accelerator (AES)

RM0456

A typical message construction for GMAC is given in Figure 478.
Figure 478. Message construction in GMAC mode
[Len(A)]64

[0]64

Len(A)
16-byte
boundaries
Authenticated data

0

Last
block

auth.

ICB

4-byte boundaries

Authentication tag (T)
Initialization vector (IV)

Counter

Zero padding
MSv42158V2

AES GMAC processing
Figure 479 describes the GMAC mode implementation in the AES peripheral. This mode is
selected by writing 011 to the CHMOD[2:0] bitfield of the AES_CR register.
Figure 479. GMAC authentication mode
(1) Init
[0]128

(4) Final
AES_IVRx
IV + 32-bit counter (= 0x0)

AES_KEYRx (KEY)
Encrypt
H

AES_KEYRx (KEY)
Encrypt

(2) Header
AES_DINR
(message block 1)

AES_DINR
(message block n)

Swap
management

Swap
management

DATATYPE
[1:0]

AES_DINR
len(A)64 || [0]64

H
H

GF2mul

H

GF2mul

GF2mul
S

Legend
AES_DOUTR
(authentication tag T)

input
output
XOR

MSv42150V2

The GMAC algorithm corresponds to the GCM algorithm applied on a message only
containing a header. As a consequence, all steps and settings are the same as with the
GCM, except that the payload phase is omitted.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

AES hardware accelerator (AES)

Suspend/resume operations in GMAC
In GMAC mode, the sequence described for the GCM applies except that only the header
phase can be interrupted.

49.4.12

AES counter with CBC-MAC (CCM)
Overview
The AES counter with cipher block chaining-message authentication code (CCM)
algorithm allows encryption and authentication of plaintext, generating the corresponding
ciphertext and tag (also known as message authentication code). To ensure confidentiality,
the CCM algorithm is based on AES in counter mode. It uses cipher block chaining
technique to generate the message authentication code. This is commonly called CBCMAC.

Note:

NIST does not approve this CBC-MAC as an authentication mode outside the context of the
CCM specification.
CCM chaining is specified in NIST Special Publication 800-38C, Recommendation for Block
Cipher Modes of Operation - The CCM Mode for Authentication and Confidentiality. A typical
message construction for CCM is given in Figure 480.
Figure 480. Message construction in CCM mode
Len(C)
Len(A)

Len(P)

Len(T)

16-byte
boundaries
Associated data (A)

flags

Nonce (N)
Len(N)

Q

0

Enc
(T)

encrypt

4-byte boundaries

Plaintext (P)

0

authenticate

[a]32
[a]16

B0

Authenticated & encrypted ciphertext (C)

Decrypt and compare

MAC (T)

Zero padding

MSv42159V2

The structure of the message is:
•

•

16-byte first authentication block (B0), composed of three distinct fields:
–

Q: a bit string representation of the octet length of P (Len(P))

–

Nonce (N): a single-use value (that is, a new nonce must be assigned to each
new communication) of Len(N) size. The sum Len(N) + Len(P) must be equal to
15 bytes.

–

Flags: most significant octet containing four flags for control information, as
specified by the standard. It contains two 3-bit strings to encode the values t (MAC
length expressed in bytes) and Q (plaintext length such that Len(P) < 28q bytes).
The counter blocks range associated to Q is equal to 28Q-4, that is, if the maximum
value of Q is 8, the counter blocks used in cipher must be on 60 bits.

16-byte blocks (B) associated to the Associated Data (A).
This part of the message is only authenticated, not encrypted. This section has a

RM0456 Rev 6

<!-- pagebreak -->

1973

AES hardware accelerator (AES)

RM0456

known length Len(A) that can be a non-multiple of 16 bytes (see Figure 480). The
standard also states that, on MSB bits of the first message block (B1), the associated
data length expressed in bytes (a) must be encoded as follows:
–
–
–

If 0 < a < 216 - 28, then it is encoded as [a]16, that is, on two bytes.

If 216 - 28 < a < 232, then it is encoded as 0xff || 0xfe || [a]32, that is, on six bytes.
If 232 < a < 264, then it is encoded as 0xff || 0xff || [a]64, that is, on ten bytes.

•

16-byte blocks (B) associated to the plaintext message P, which is both authenticated
and encrypted as ciphertext C, with a known length Len(P). This length can be a nonmultiple of 16 bytes (see Figure 480).

•

Encrypted MAC (T) of length Len(T) appended to the ciphertext C of overall length
Len(C).

When a part of the message (A or P) has a length that is a non-multiple of 16-bytes, a
special padding scheme is required.
Note:

CCM chaining mode can also be used with associated data only (that is, no payload).
As an example, the C.1 section in NIST Special Publication 800-38C gives the following
values (hexadecimal numbers):
N: 10111213 141516 (Len(N) = 56 bits or 7 bytes)
A: 00010203 04050607 (Len(A) = 64 bits or 8 bytes)
P: 20212223 (Len(P) = 32 bits or 4 bytes)
T: 6084341B (Len(T) = 32 bits or t = 4)
B0: 4F101112 13141516 00000000 00000004
B1: 00080001 02030405 06070000 00000000
B2: 20212223 00000000 00000000 00000000
CTR0: 0710111213 141516 00000000 00000000
CTR1: 0710111213 141516 00000000 00000001
Generation of formatted input data blocks Bx (especially B0 and B1) must be managed by
the application.

<!-- pagebreak -->

