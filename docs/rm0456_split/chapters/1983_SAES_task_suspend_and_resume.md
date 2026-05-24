RM0456 Rev 6

RM0456

50.4.7

Secure AES coprocessor (SAES)

SAES task suspend and resume
A message can be suspended if another message with a higher priority must be processed.
When this highest priority message is sent, the suspended message can resume in both
encryption or decryption mode.
Suspend/resume operations do not break the chaining operation and the message
processing can resume as soon as SAES is enabled again to receive the next data block.
Figure 488 gives an example of suspend/resume operation: Message 1 is suspended in
order to send a shorter and higher-priority Message 2.
Figure 488. Example of suspend mode management
Message 1

Message 2

128-bit block 1

New higher-priority
message 2 to be
processed

128-bit block 2

128-bit block 3

AES suspend
sequence
128-bit block 1

128-bit block 2
128-bit block 4

128-bit block 5

AES resume
sequence

128-bit block 6

...

MSv42148V1

A detailed description of suspend/resume operations is in the sections dedicated to each
SAES mode.

50.4.8

SAES basic chaining modes (ECB, CBC)
Overview
This section gives a brief explanation of the four basic operation modes provided by the
SAES core: ECB encryption, ECB decryption, CBC encryption and CBC decryption. For
detailed information, refer to the FIPS publication 197 from November 26, 2001.

RM0456 Rev 6

<!-- pagebreak -->

2015

Secure AES coprocessor (SAES)

RM0456

Figure 489 illustrates the electronic codebook (ECB) encryption.
Figure 489. ECB encryption

DATATYPE[1:0]
AES_KEYRx (KEY)

Block 1

Block 2

AES_DINR (plaintext P1)

AES_DINR (plaintext P2)

Swap
management
I1

DATATYPE[1:0]
AES_KEYRx (KEY)

Encrypt

Legend

DATATYPE[1:0]

O1
Swap
management

Swap
management
I2
Encrypt

AES core

DATATYPE[1:0]

O2
Swap
management

input
output

AES_DOUTR (ciphertext C1)

AES_DOUTR (ciphertext C2)
MSv19105V2

In ECB encrypt mode, the 128-bit plaintext input data block Px in the SAES_DINR register
first goes through bit/byte/half-word swapping. The swap result Ix is processed with the AES
core set in encrypt mode, using a 128- or 256-bit key. The encryption result Ox goes through
bit/byte/half-word swapping, then is stored in the SAES_DOUTR register as 128-bit
ciphertext output data block Cx. The ECB encryption continues in this way until the last
complete plaintext block is encrypted.
Figure 490 illustrates the electronic codebook (ECB) decryption.
Figure 490. ECB decryption

DATATYPE[1:0]

Block 1

Block 2

AES_DINR (ciphertext C1)

AES_DINR (ciphertext C2)

Swap
management
I1

AES_KEYRx (KEY)

DATATYPE[1:0]
AES_KEYRx (KEY)

Decrypt

Legend
DATATYPE[1:0]
input
output

Swap
management
I2

O1
Swap
management

AES_DOUTR (plaintext P1)

Decrypt

DATATYPE[1:0]

O2
Swap
management

AES_DOUTR (plaintext P2)
MSv19106V2

To perform an AES decryption in the ECB mode, the secret key has to be prepared by
collecting the last-round encryption key (which requires to first execute the complete key
schedule for encryption), and using it as the first-round key for the decryption of the
ciphertext. This preparation is supported by the AES core.
In ECB decrypt mode, the 128-bit ciphertext input data block C1 in the SAES_DINR register
first goes through bit/byte/half-word swapping. The keying sequence is reversed compared
to that of the ECB encryption. The swap result I1 is processed with the AES core set in
decrypt mode, using the formerly prepared decryption key. The decryption result goes
through bit/byte/half-word swapping, then is stored in the SAES_DOUTR register as 128-bit
plaintext output data block P1. The ECB decryption continues in this way until the last
complete ciphertext block is decrypted.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure AES coprocessor (SAES)
Figure 491 illustrates the cipher block chaining (CBC) encryption.
Figure 491. CBC encryption

DATATYPE[1:0]
AES_IVRx (init. vector)

Block 1

Block 2

AES_DINR (plaintext P1)

AES_DINR (plaintext P2)

Swap
management
P1'
IVI
I1

AES_KEYRx (KEY)

Block cipher
encryption

DATATYPE[1:0]

I2

AES_KEYRx (KEY)

Block cipher
encryption

O1

Legend
input

DATATYPE[1:0]

Swap
management

Swap
management
P2'

O2
DATATYPE[1:0]

Swap
management

output
AES_DOUTR (ciphertext C1)

XOR

AES_DOUTR (ciphertext C2)
MSv19107V2

In CBC encrypt mode, the first plaintext input block, after bit/byte/half-word swapping (P1’),
is XOR-ed with a 128-bit IVI bitfield (initialization vector and counter), producing the I1 input
data for encrypt with the AES core, using a 128- or 256-bit key. The resulting 128-bit output
block O1, after swapping operation, is used as ciphertext C1. The O1 data is then XOR-ed
with the second-block plaintext data P2’ to produce the I2 input data for the AES core to
produce the second block of ciphertext data. The chaining of data blocks continues in this
way until the last plaintext block in the message is encrypted.
If the message size is not a multiple of 128 bits, the final partial data block is encrypted in
the way explained in Section 50.4.6: SAES ciphertext stealing and data padding.
Figure 492 illustrates the cipher block chaining (CBC) decryption.
Figure 492. CBC decryption

DATATYPE[1:0]

Block 1

Block 2

AES_DINR (ciphertext C1)

AES_DINR (ciphertext C2)

Swap
management
I1

AES_KEYRx (KEY)

AES_IVRx (IV)

IVI

Legend
DATATYPE[1:0]
input
output

DATATYPE[1:0]

Swap
management
I2

AES_KEYRx (KEY)

Decrypt

Decrypt

O1

O2

P1'
Swap
management

AES_DOUTR (plaintext P1)

XOR

DATATYPE[1:0]

P2'
Swap
management

AES_DOUTR (plaintext P2)
MSv19104V2

In CBC decrypt mode, like in ECB decrypt mode, the secret key must be prepared to
perform an AES decryption.
After the key preparation process, the decryption goes as follows: the first 128-bit ciphertext
block (after the swap operation) is used directly as the AES core input block I1 for decrypt
operation, using the 128-bit or 256-bit key. Its output O1 is XOR-ed with the 128-bit IVI field
(that must be identical to that used during encryption) to produce the first plaintext block P1.

RM0456 Rev 6

<!-- pagebreak -->

2015

Secure AES coprocessor (SAES)

RM0456

The second ciphertext block is processed in the same way as the first block, except that the
I1 data from the first block is used in place of the initialization vector.
The decryption continues in this way until the last complete ciphertext block is decrypted.
If the message size is not a multiple of 128 bits, the final partial data block is decrypted in
the way explained in Section 50.4.6: SAES ciphertext stealing and data padding.
For more information on data swapping, refer to Section 50.4.11: SAES data registers and
data swapping.

ECB/CBC encryption sequence
The sequence of events to perform an ECB/CBC encryption (more detail in Section 50.4.4):
1.

Verify that BUSY = 0 in SAES_SR (no RNG random number fetch in progress).

2.

Disable the SAES peripheral by clearing the EN bit of the SAES_CR register.

3.

Select the Mode 1 by setting to 00 the MODE[1:0] bitfield of the SAES_CR register and
select ECB or CBC chaining mode by setting the CHMOD[2:0] bitfield of the SAES_CR
register to 000 or 001, respectively. Data type can also be defined, using
DATATYPE[1:0] bitfield. Select normal key mode by setting KMOD[1:0] to 00. For
encryption with other KMOD[1:0] values, refer to Section 50.4.10 and Section 50.4.9.

4.

Select 128- or 256-bit key length through the KEYSIZE bit of the SAES_CR register. If
the key must not be shared with a different security context (different secure attribute),
the KEYPROT bit of the SAES_CR register must also be set.

5.

Write the SAES_KEYRx registers (128 or 256 bits) with encryption key. Alternatively,
select a key source different from the key registers, through KEYSEL[2:0]. Refer to
Section 50.4.12: SAES key registers for details. Fill the SAES_IVRx registers with the
initialization vector data if CBC mode has been selected.

6.

Enable the SAES peripheral by setting the EN bit of the SAES_CR register.

7.

Write the SAES_DINR register four times to input the plaintext (MSB first), as shown in
Figure 493.

8.

Wait until the CCF flag is set in the SAES_SR register.

9.

Read the SAES_DOUTR register four times to get the ciphertext (MSB first) as shown
in Figure 493. Then clear the CCF flag by setting the CCF bit of the SAES_ISR register.

10. Repeat steps 7-8-9 to process all the blocks with the same encryption key.
Figure 493. ECB/CBC encryption (Mode 1)
WR
PT3
MSB

WR
PT2

WR
PT1

WR
PT0

Wait until flag CCF = 1

LSB

RD
CT3
MSB

Computation phase

Input phase
4 write operations into
AES_DINR[31:0]

RD
CT2

RD
CT1

RD
CT0
LSB

Output phase
4 read operations of
AES_DOUTR[31:0]

PT = plaintext = 4 words (PT3, … , PT0)
CT = ciphertext = 4 words (CT3, … , CT0)
MS18936V3

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure AES coprocessor (SAES)

ECB/CBC decryption sequence
The sequence of events to perform an AES ECB/CBC decryption is as follows (More detail
in Section 50.4.4). Select normal key mode by setting KMOD[1:0] to 00. For decryption with
other KMOD[1:0] values, refer to Section 50.4.10 and Section 50.4.9.
1.

Follow the steps described in Section 50.4.5: SAES decryption round key preparation,
in order to prepare the decryption key in AES core.

2.

Select the Mode 3 by setting to 10 the MODE[1:0] bitfield of the SAES_CR register and
select ECB or CBC chaining mode by setting the CHMOD[2:0] bitfield of the SAES_CR
register to 000 or 001, respectively. Data type can also be defined, using
DATATYPE[1:0] bitfield. KEYSIZE and KMOD[1:0] bitfields must be kept as-is.

3.

Write the SAES_IVRx registers with the initialization vector (required in CBC mode
only).

4.

Enable SAES by setting the EN bit of the SAES_CR register.

5.

Write the SAES_DINR register four times to input the cipher text (MSB first), as shown
in Figure 494.

6.

Wait until the CCF flag is set in the SAES_SR register.

7.

Read the SAES_DOUTR register four times to get the plain text (MSB first), as shown
in Figure 494. Then clear the CCF flag by setting the CCF bit of the SAES_ISR register.

8.

Repeat steps 5-6-7 to process all the blocks encrypted with the same key.
Figure 494. ECB/CBC decryption (Mode 3)
WR
CT3
MSB

WR
CT2

WR
CT1

WR
CT0

RD
PT3

Wait until flag CCF = 1

LSB

RD
PT2

MSB

Input phase
4 write operations into
AES_DINR[31:0]

Computation phase

RD
PT1

RD
PT0
LSB

Output phase
4 read operations from
AES_DOUTR[31:0]

PT = plaintext = 4 words (PT3, … , PT0)
CT = ciphertext = 4 words (CT3, … , CT0)
MS18938V3

Suspend/resume operations in ECB/CBC modes
The following sequences are valid for normal key mode (KMOD[1:0] = 00).
To suspend the processing of a message, proceed as follows:
1.

If DMA is used, stop the SAES DMA transfers to the IN FIFO by clearing the DMAINEN
bit of the SAES_CR register.

2.

If DMA is not used, read four times the SAES_DOUTR register to save the last
processed block. If DMA is used, wait until the CCF flag is set in the SAES_SR register
then stop the DMA transfers from the OUT FIFO by clearing the DMAOUTEN bit of the
SAES_CR register.

3.

If DMA is not used, poll the CCF flag of the SAES_SR register until it becomes 1
(computation completed).

4.

Clear the CCF flag by setting the CCF bit of the SAES_ISR register.

RM0456 Rev 6

<!-- pagebreak -->

