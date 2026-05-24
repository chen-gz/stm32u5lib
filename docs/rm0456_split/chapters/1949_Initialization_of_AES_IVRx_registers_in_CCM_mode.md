RM0456 Rev 6

RM0456

AES hardware accelerator (AES)

CCM processing
Figure 481 describes the CCM implementation within the AES peripheral (encryption
example). This mode is selected by writing 100 into the CHMOD[2:0] bitfield of the AES_CR
register.
Figure 481. CCM mode authenticated encryption
Block 1
(1) Init

Block m

(3) Payload

AES_IVRx (CTR1)

AES_IVRx (CTR0)

Counter
increment (+1)

mask
AES_IVRx (First block B0)

AES_IVRx (CTRm)

Counter
increment (+1)

AES_KEYRx (KEY)

AES_KEYRx (KEY)
Encrypt

Encrypt
AES_DINR (plaintext
last block Pm )

AES_DINR (plaintext P1)

Encrypt

S1
Swap
management

Sm

DATATYPE
[1:0]

Swap
management

AES_KEYRx (KEY)
DATATYPE
[1:0]

(2) Header
AES_DINR (associated
data block B1)

DATATYPE
[1:0]

Swap
management

AES_DINR (associated
data last block Bu)

AES_DOUTR (ciphertext C1)
Swap
Swap
management
management
DATATYPE[1:0]

Swap
management

AES_DOUTR
(ciphertext last block Cm)
Br

Bu+1

DATATYPE[1:0]

DATATYPE
[1:0]

AES_KEYRx (KEY)
Encrypt

Encrypt

Encrypt

AES_KEYRx (KEY)

AES_KEYRx (KEY)

AES_KEYRx (KEY)

Encrypt
AES_DINR (CTR0)
MAC (T)

Legend

Encrypt

S0

input
output

AES_KEYRx (KEY)

AES_DOUTR
(EncTAG)

(4) Final

XOR

MSv42152V2

The data input to the generation-encryption process are a valid nonce, a valid payload
string, and a valid associated data string, all properly formatted. The CBC chaining
mechanism is applied to the formatted plaintext data to generate a MAC, with a known
length. Counter mode encryption that requires a sufficiently long sequence of counter blocks
as input, is applied to the payload string and separately to the MAC. The resulting ciphertext
C is the output of the generation-encryption process on plaintext P.
AES_IVRx registers are used for processing each data block, AES automatically
incrementing the CTR counter with a bit length defined by the first block B0. Table 473
shows how the application must load the B0 data.
Note:

The AES peripheral in CCM mode supports counters up to 64 bits, as specified by NIST.
Table 473. Initialization of AES_IVRx registers in CCM mode
AES_IVR3[31:0]

AES_IVR2[31:0]

AES_IVR1[31:0]

AES_IVR0[31:0]

B0[127:96]

B0[95:64]

B0[63:32]

B0[31:0]

RM0456 Rev 6

<!-- pagebreak -->

1973

AES hardware accelerator (AES)

Note:

RM0456

In this mode, the setting 01 of the MODE[1:0] bitfield (key derivation) is forbidden.
A CCM message is processed through the following phases, further described in next
subsections:
•

Init phase: AES processes the first block and prepares the first counter block.

•

Header phase: AES processes associated data (A), with tag computation only.

•

Payload phase: IP processes plaintext (P), with tag computation, counter block
encryption, and data XOR-ing. It works in a similar way for ciphertext (C).

•

Final phase: AES generates the message authentication code (MAC).

CCM Init phase
In this phase, the first block B0 of the CCM message is written into the AES_IVRx register.
The AES_DOUTR register does not contain any output data. The recommended sequence
is:
1.
2.

Disable the AES peripheral by clearing the EN bit of the AES_CR register.
Select CCM chaining mode, by setting to 100 the CHMOD[2:0] bitfield of the AES_CR
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
B0 data as described in Table 473.

6.

Start the calculation of the counter, by setting to 1 the EN bit of the AES_CR register
(EN is automatically reset when the calculation finishes).

7.

Wait until the end of computation, indicated by the CCF flag of the AES_SR transiting
to 1. Alternatively, use the corresponding interrupt.

8.

Clear the CCF flag in the AES_SR register, by setting to 1 the CCF bit of the AES_ICR
register.

CCM header phase
This phase coming after the GCM Init phase must be completed before the payload phase.
During this phase, the AES_DOUTR register does not contain any output data.
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

The header phase can be skipped if there is no associated data, that is, Len(A) = 0.
The first block of the associated data (B1) must be formatted by software, with the
associated data length.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

AES hardware accelerator (AES)
CCM payload phase (encryption or decryption)
This phase, identical for encryption and decryption, is executed after the CCM header
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

If it is the last data block to encrypt and the plaintext size in the block is inferior to 128
bits, pad the remainder of the block with zeros.

4.

Append the data block into AES in one of ways described in Section 49.4.4: AES
procedure to perform a cipher operation on page 1929, and read the result.

5.

Repeat the previous step till the second-last plaintext block is encrypted or till the last
block of ciphertext is decrypted. For the last block of plaintext (encryption only), apply
the two previous steps. For the last block, discard the data that is not part of the
payload when the last block size is less than 16 bytes.

The payload phase can be skipped if there is no payload data, that is, Len(P) = 0 or
Len(C) = Len(T).
Remove LSBLen(T)(C) encrypted tag information when decrypting ciphertext C.
CCM final phase
In this last phase, the AES peripheral generates the GCM authentication tag and stores it in
the AES_DOUTR register. The sequence to execute is:
1.

Note:

Indicate the final phase, by setting to 11 the GCMPH[1:0] bitfield of the AES_CR
register.

2.

Wait until the end-of-computation flag CCF of the AES_SR register is set.

3.

Read four times the AES_DOUTR register: the output corresponds to the CCM
authentication tag.

4.

Clear the CCF flag of the AES_SR register by setting the CCF bit of the AES_ICR
register.

5.

Disable the AES peripheral, by clearing the EN bit of the AES_CR register.

6.

For authenticated decryption, compare the generated encrypted tag with the encrypted
tag padded in the ciphertext.

In this final phase, swapping is applied to tag data read from AES_DOUTR register.
When transiting from the header phase to the final phase, the AES peripheral must not be
disabled, otherwise the result is wrong.
Application must mask the authentication tag output with tag length to obtain a valid tag.

Suspend/resume operations in CCM mode
To suspend the processing of a message in header or payload phase, proceed as
follows:
1.

If DMA is used, stop the AES DMA transfers to the IN FIFO by clearing the DMAINEN
bit of the AES_CR register. If DMA is not used, make sure that the current computation
is completed, which is indicated by the CCF flag of the AES_SR register set to 1.

2.

In the payload phase, if DMA is not used, read four times the AES_DOUTR register to
save the last-processed block. If DMA is used, wait until the CCF flag is set in the
RM0456 Rev 6

<!-- pagebreak -->

1973

AES hardware accelerator (AES)

RM0456

AES_SR register then stop the DMA transfers from the OUT FIFO by clearing the
DMAOUTEN bit of the AES_CR register.
3.

Clear the CCF flag of the AES_SR register, by setting to 1 the CCF bit of the AES_ICR
register.

4.

Save the AES_SUSPxR registers (where x is from 0 to 7) in the memory.

5.

Save the AES_IVRx registers as, during the data processing, they changed from their
initial values.

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

49.4.13

1.

If DMA is used, configure the DMA controller in order to complete the rest of the FIFO
IN transfers. In the payload phase, the rest of the FIFO OUT transfers must also be
configured in the DMA controller.

2.

Disable the AES peripheral by clearing the EN bit of the AES_CR register.

3.

Write the suspend register values, previously saved in the memory, back into their
corresponding AES_SUSPxR registers (where x is from 0 to 7).

4.

Write the initialization vector register values, previously saved in the memory, back into
their corresponding AES_IVRx registers.

5.

Restore the initial setting values in the AES_CR and AES_KEYRx registers.

6.

Enable the AES peripheral by setting the EN bit of the AES_CR register.

7.

If DMA is used, enable AES DMA requests by setting to 1 the DMAINEN bit (and
DMAOUTEN bit if in payload phase) of the AES_CR register.

AES operation with shared keys
The AES peripheral can use the SAES peripheral as security co-processor. In this case,
secure application prepares the key in the robust SAES peripheral. When ready, the AES
application can load this prepared key through a dedicated hardware key bus.
Recommended sequences are described below and in SAES peripheral section
Section 50.4.10: SAES operation with shared keys.

<!-- pagebreak -->

1.

In SAES peripheral application encrypts the key to be shared, once, in Shared-key
mode (KMOD[1:0] = 10).

2.

Each time shared key is needed in AES peripheral application needs to decrypt it in
SAES peripheral, selecting the same Shared-key mode (KMOD[1:0] = 10).

3.

Once the shared key is decrypted and loaded in SAES_KEYRx registers it can be
shared with the AES peripheral. Indeed, when shared key must be loaded in AES
peripheral, application sets correct KEYSIZE and write KMOD[1:0] = 10 in AES_CR
register. When KEYVALID is cleared, the key information is automatically transfered by
hardware into AES_KEYRx, with BUSY set in AES_SR.

4.

Once transfer is completed BUSY bit is cleared and KEYVALID bit is set in AES_SR
register. If KEYVALID is not set when BUSY bit is cleared, or if a key error flag (KEIF) is
set it means that an unexpected event occurred during the transfer (for example

RM0456 Rev 6

RM0456

AES hardware accelerator (AES)
hardware fault) or the KEYVALID bit in SAES_SR was cleared before the end of the
transfer. When such errors occur, the whole SAES key sharing process must be
restarted through the IPRST bits of control registers in both SAES and AES peripherals
(see Section 49.4.18 for details).
At that point AES is initialized with a valid, shared key. Application can proceed with the
processing of data, setting KMOD[1:0] to 00.

Note:

While KMOD[1:0] = 10 and BUSY = 1, if KEYSIZE in AES peripheral is different from the
KEYSIZE in SAES peripheral the key sharing fails and KEIF is set in both peripherals.

49.4.14

AES data registers and data swapping
Data input and output
A 128-bit data block is entered into the AES peripheral with four successive 32-bit word
writes into the AES_DINR register (bitfield DIN[31:0]), the most significant word (bits
[127:96]) first, the least significant word (bits [31:0]) last.
A 128-bit data block is retrieved from the AES peripheral with four successive 32-bit word
reads from the AES_DOUTR register (bitfield DOUT[31:0]), the most significant word (bits
[127:96]) first, the least significant word (bits [31:0]) last.
The 32-bit data word for AES_DINR register or from AES_DOUTR register is organized in
big endian order, that is:
•
the most significant byte of a word to write into AES_DINR must be put on the lowest
address out of the four adjacent memory locations keeping the word to write, or
•

the most significant byte of a word read from AES_DOUTR goes to the lowest address
out of the four adjacent memory locations receiving the word

For using DMA for input data block write into AES, the four words of the input block must be
stored in the memory consecutively and in big-endian order, that is, the most significant
word on the lowest address. See Section 49.4.17: AES DMA interface.

Data swapping
The AES peripheral can be configured to perform a bit-, a byte-, a half-word-, or no
swapping on the input data word in the AES_DINR register, before loading it to the AES
processing core, and on the data output from the AES processing core, before sending it to
the AES_DOUTR register. The choice depends on the type of data. For example, a byte
swapping is used for an ASCII text stream.
The data swap type is selected through the DATATYPE[1:0] bitfield of the AES_CR register.
The selection applies both to the input and the output of the AES core.
For different data swap types, Figure 482 shows the construction of AES processing core
input buffer data P127 to P0, from the input data entered through the AES_DINR register, or
the construction of the output data available through the AES_DOUTR register, from the
AES processing core output buffer data P127 to P0.

RM0456 Rev 6

<!-- pagebreak -->

1973

AES hardware accelerator (AES)

RM0456

Figure 482. 128-bit block construction with respect to data swap
increasing memory address
byte 3
D63

DATATYPE[1:0] = 00: no swapping

byte 2

D56 D55

byte 1

D48 D47

byte 0

D40 D39

D32

MSB

LSB

Word 3

Word 2

D127

D96

Word 1

D95

1

D63

D64

2

D127

D96

Word 0
D32

D31

3

D95
P95

4

D63

D64

D0

D32

D31

D0
LSB

MSB

DATATYPE[1:0] = 01: 16-bit (half-word) swapping
MSB

LSB

Word 3
D127

D112

Word 2

D111

D96

D95

1

D80 D79

D64

D63

2

D111

D96

D127

D112

Word 0

Word 1
D48 D47

D32

D31

3

D79

D64 D95

D80

D16 D15

D0

D0 D31

D16
LSB

4

D47

D32 D63

D48

D15

MSB

DATATYPE[1:0] = 10: 8-bit (byte) swapping
MSB

LSB

Word 3

Word 2

D127..120 D119..112 D111.104

D103..96

D95..88

1
D103..96

D87..80

Word 1

D79..72

D71...64

D63...56

2
D111.104

D119..112 D127..120

D55...48

Word 0

D47...40

D39...32

D31...24

3

D71...64

D79..72

D87..80

D95..88

D39...32

D23...16

D15...8

D7...0

D15...8

D23...16

D31...24
LSB

4
D47...40

D55...48

D63...56

D7...0

MSB

DATATYPE[1:0] = 11: bit swapping
LSB

MSB

Word 2

Word 3

D98 D97 D96 D95 D94 D93

D127 D126 D125

1

Word 1
D66 D65 D64 D63 D62 D61

2

D96 D97 D98

D125 D126 D127 D64 D65 D66

Word 0
D34 D33 D32 D31 D30 D29

3
D93 D94 D95 D32 D33 D34

D2

D1

D61 D62 D63

D0

D1

D2

D29 D30 D31
LSB

MSB

Legend:

AES input/output data block in memory

MSB

most significant bit (127) of memory data block / AEC core buffer

AES core input/output buffer data

LSB

least significant bit (0) of memory data block / AEC core buffer

Zero padding (example)

1

Order of write to AES_DINR / read from AES_DOUTR

Data swap

Note:

D0

4

4
Dx

input/output data bit ‘x’

MSv42153V2

The data in AES key registers (AES_KEYRx) and initialization registers (AES_IVRx) are not
sensitive to the swap mode selection.

Data padding
Figure 482 also gives an example of memory data block padding with zeros such that the
zeroed bits after the data swap form a contiguous zone at the MSB end of the AES core
input buffer. The example shows the padding of an input data block containing:

<!-- pagebreak -->

