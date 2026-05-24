2015

Secure AES coprocessor (SAES)

RM0456

5.

Save initialization vector registers (only required in CBC mode as SAES_IVRx registers
are altered during the data processing).

6.

Disable the SAES peripheral by clearing the bit EN of the SAES_CR register.

7.

Save the SAES_CR register and clear the key registers if they are not needed, to
process the higher priority message.

8.

If DMA is used, save the DMA controller status (pointers for IN and OUT data transfers,
number of remaining bytes, and so on).

To resume the processing of a message, proceed as follows:
1.

50.4.9

If DMA is used, configure the DMA controller so as to complete the rest of the FIFO IN
and FIFO OUT transfers.

2.

Disable the SAES peripheral by clearing the EN bit of the SAES_CR register.

3.

Restore SAES_CR register (with correct KEYSIZE) then restore SAES_KEYRx
registers. Alternatively, select a key source different from key registers, using
KEYSEL[2:0]. Refer to Section 50.4.12: SAES key registers for details.

4.

Prepare the decryption key as described in Section 50.4.5: SAES decryption round key
preparation (only required for ECB or CBC decryption).

5.

Restore SAES_IVRx registers using the saved configuration (only required in CBC
mode).

6.

Enable the SAES peripheral by setting the EN bit of the SAES_CR register.

7.

If DMA is used, enable SAES DMA transfers by setting the DMAINEN and
DMAOUTEN bits of the SAES_CR register.

SAES operation with wrapped keys
SAES peripheral can wrap (encrypt) and unwrap (decrypt) application keys using
hardware-secret key DHUK, XOR-ed or not with application key BHK. With this feature,
AES keys can be made usable by application software without being exposed in clear-text
(unencrypted).
Wrapped key sequences are too small to be suspended/resumed. SAES cannot unwrap a
key using an unwrapped key.
Figure 495 summarizes the operation with wrapped keys. To protect the wrapped key,
select DHUK by setting KEYSEL[2:0] to 001 or 100. Alternatively, select BHK by setting
KEYSEL to 010 if the corresponding registers are read/write-locked in the TAMP peripheral.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure AES coprocessor (SAES)
Figure 495. Operation with wrapped keys
Wrapped-key mode (KMOD = 01)
Step 2: load

DOUT
wrapped (encrypted) key

KEYSEL = 001 or 100
MODE = 00 (encryption)

dec

DIN
unwrapped key

SAES

Step 3: use

IV (if applicable)

enc

BHK

SAES

DHUK

DIN
wrapped key

DHUK

DIN
clear-text key

IV (if applicable)

DHUK
BHK

DHUK

Step 1: provision

Normal-key mode (KMOD = 00)

DOUT

unwrapped (decrypted) key

KEYSEL = 001 or 100
MODE =10 (decryption)

SAES
enc/dec

KEYSEL = 00 (to keep the wrapped key)
MODE = any value

Pink: hardware secret (not readable by application)

Note:

DHUK value depends on privilege, KMOD[1:0], KEYSEL, CHMOD[2:0], and on whether
SAES peripheral is secure or nonsecure.
Encryption in Wrapped-key mode
Recommended sequence to wrap (that is, encrypt) a key is described below:
1.

Verify that BUSY = 0 in SAES_SR (no RNG random number fetch in progress).

2.
3.

Disable the SAES peripheral by clearing the EN bit of the SAES_CR register.
In SAES_CR register, select the Mode 1 (encryption) by setting to 00 the MODE[1:0]
bitfield, and select ECB or CBC chaining mode by setting CHMOD[2:0] bitfield to 000 or
001, respectively. Data type can be defined as 32-bit, with DATATYPE[1:0] bitfield set
to 00. Key size must be properly configured using KEYSIZE bit, and KMOD[1:0] bitfield
must be set as 01 (wrapped key). The KEYSIZE information is used both for the
encryption key and for the key to be encrypted.

4.

Write the SAES_IVRx registers with the initialization vector if CBC mode has been
selected in previous step.

5.

Select the DHUK by setting the KEYSEL[2:0] bitfield of the SAES_CR register to 001 or
100. Upon successful key loading BUSY bit is cleared and KEYVALID bit is set in
SAES_SR register. Refer to Section 50.4.12: SAES key registers for detail on
KEYSEL = 100 usage.

6.

Enable SAES by setting the EN bit of the SAES_CR register.

7.

Write the SAES_DINR register four times to input the key to encrypt (MSB first, see
Table 481 on page 1995).

8.

Wait until the CCF flag is set in the SAES_SR register.

9.

Get the encrypted key (MSB first) by reading the SAES_DOUTR register four times.
Then clear the CCF flag by setting the CCF bit in SAES_ICR register.

10. Repeat steps 6 to 8 if KEYSIZE is 256 bits.
Note:

Encryption in Wrapped-key mode is only supported when ECB or CBC is selected through
CHMOD[2:0].

RM0456 Rev 6

<!-- pagebreak -->

2015

Secure AES coprocessor (SAES)

RM0456

Decryption in Wrapped-key mode
Recommended sequence to unwrap (i.e. decrypt) a wrapped key is described below:
1.

Verify that BUSY = 0 in SAES_SR (no RNG random number fetch in progress).

2.
3.

Disable the SAES peripheral by clearing the EN bit of the SAES_CR register.
SAES_CR register, select the Mode 2 by setting to 01 the MODE[1:0] bitfield, and
select ECB or CBC chaining mode by setting CHMOD[2:0] to 000 or 001, respectively.
Key size must be properly configured using KEYSIZE bit, and KMOD[1:0] bitfield must
be set as 01 (wrapped key). Data type selection with DATATYPE[1:0] bitfield must be
the same as the one used during encryption (that is, 0x0). The KEYSIZE information is
used both for the decryption key and for the key to be decrypted.

4.

If the decrypted key is not to share with a different security context (different security
attribute), the KEYPROT bit of the SAES_CR register must also be set.
Select the DHUK by setting the KEYSEL[2:0] bitfield of the SAES_CR register to 001 or
100. Upon successful key loading, the SAES_SR register BUSY bit is cleared and
KEYVALID bit set. Refer to Section 50.4.12: SAES key registers for detail on
KEYSEL = 100 usage.

5.

Enable SAES by setting the EN bit of the SAES_CR register.

6.

Wait until the CCF flag is set in the SAES_SR register.

7.

Clear the CCF flag. Decryption key is available in AES core, and SAES is disabled
automatically.

8.

In SAES_CR register select the Mode 3 by setting to 10 the MODE[1:0] bitfield.

9.

Write the SAES_IVRx registers with the initialization vector if CBC mode has been
selected in previous step.

10. Enable SAES by setting the EN bit of the SAES_CR register.
11. Write the SAES_DINR register four times to input the encrypted random key (MSB first,
see Table 481 on page 1995).
12. Wait until the CCF flag is set in the SAES_SR register.
13. Clear the CCF flag, then repeat steps 10 and 11 if KEYSIZE is 256 bits.
When the decrypted key is loaded in key registers, KEYSEL[2:0] of the SAES_CR register is
automatically cleared. Hence, after this sequence, the decrypted wrapped key is available in
SAES_KEYRx registers, immediately usable by the application software for any AES
operation (normal key mode, that is, with KMOD[1:0] = 00).
Decrypted wrapped key can be shared with an application running in a different security
context (different security attribute) if KEYPROT bit was cleared during step 2.
Note:

When KMOD[1:0] = 01 (wrapped key) and MODE[1:0] = 10 (decryption) a read access to
SAES_DOUTR register triggers a read error (RDERR).
When KEYSEL[2:0] = 001 (DHUK) or 100 (DHUK XOR BHK), the application software
must use the same privilege, security, KMOD[1:0], CHMOD[2:0] and KEYSIZE context for
encryption and decryption. Otherwise, the result is incorrect.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

50.4.10

Secure AES coprocessor (SAES)

SAES operation with shared keys
SAES peripheral can share application keys wrapped with hardware-secret key DHUK,
XOR-ed or not with application key BHK . With this feature, the application software can
make the AES keys available to the AES peripheral, without exposing them in clear-text
(unencrypted).
Shared key sequences are too small to be suspended/resumed. SAES cannot unwrap a
shared key using an unwrapped key.

Note:

When a key stored in SAES is shared with AES, the protection given by KEYPROT bit is
lost. The protection is detailed in Section 50.4.12: SAES key registers.
Figure 496 summarizes how to use Shared-key mode. To protect the shared key, DHUK
must be selected, by setting KEYSEL[2:0] to 001 or 100. Alternatively, select BHK by
setting KEYSEL to 010 if the corresponding registers are read/write-locked in the TAMP
peripheral.
Figure 496. Usage of Shared-key mode
Shared-key mode (KMOD = 10)
Step 2: load

DOUT
wrapped (encrypted)
shared key

dec

DIN
DIN

shared key

unwrapped (decrypted)
shared key

AES

shared key

SAES

Step 3: use

IV (if applicable)

enc

BHK

SAES

DHUK

DIN
wrapped shared key

DHUK

DIN
clear-text shared key

IV (if applicable)

DHUK
BHK

DHUK

Step 1: provision

Normal-key mode (KMOD = 00)

SAES
enc/dec

enc/dec
DOUT
DOUT

KEYSEL = 001 or 100
MODE = 00 (encryption)

KEYSEL = 001 or 100
MODE =10 (decryption)

KEYSEL = 00 (to keep the wrapped key)
MODE = any value

Pink: hardware secret (not readable by application)

Note:

DHUK value depends on privilege, KMOD[1:0], KSHAREID, KEYSEL, CHMOD[2:0], and on
whether SAES peripheral is secure or nonsecure.
In the step 3, AES represents the AES peripheral.

Encryption in Shared-key mode
Before SAES can share a key with the AES peripheral, the key must be encrypted once.
The encryption sequence of a shared key is the same as for a wrapped key, with KMOD[1:0]
set to 10 (shared key) and KSHAREID[1:0] kept at 00 in the step 3 in Figure 496. See
Encryption in Wrapped-key mode for details.
Note:

Encryption in Shared-key mode is only supported when ECB or CBC is selected through
CHMOD[2:0].

RM0456 Rev 6

<!-- pagebreak -->

2015

Secure AES coprocessor (SAES)

RM0456

Decryption in Shared-key mode
Each time SAES needs to share a key with the AES peripheral, shared encrypted key must
be decrypted in SAES, then loaded by the AES. The overall sequence is described next.
Sequence in the SAES peripheral
The decryption sequence of a shared key is the same as for a wrapped key, with KMOD[1:0]
set to 10 (shared key) and KSHAREID[1:0] kept at 00 in the step 3 in Figure 496. See
Decryption in Wrapped-key mode for details.
Note:

Instead of being shared, a decrypted shared key can be used directly in SAES as the
KEYSEL[2:0] bitfield is automatically cleared. In this case, KMOD[1:0] must be set to 00
(normal key mode).
Sequence in the AES peripheral
Once the shared key is decrypted in SAES key registers, it can be shared with the AES
peripheral, while SAES peripheral remains in key sharing state, that is, with KMOD[1:0] = 10
and KEYVALID = 1 in the SAES_SR register. The sequence in the AES key share target
peripheral is as follows:
1.

Initialize the AES processor to process some data

2.

When the key must be loaded, set the same KEYSIZE as for the SAES peripheral and
write the KMOD[1:0] bitfield of the AES_CR register to 10 (shared key). If the previous
sequence in the SAES peripheral completed successfully, with KMOD[1:0] kept at 10
and KSHAREID[1:0] kept at 00, the SAES_KEYRx registers are automatically copied
into the AES_KEYRx registers, with BUSY set in AES_SR.

3.

Once the transfer is completed, the BUSY flag is cleared and KEYVALID set in the
AES_SR register. If KEYVALID is not set when BUSY bit is cleared, or if a key error flag
(KEIF) is set an unexpected event occurred during the transfer, such as a DPA error, a
tamper event or the KEYVALID SAES flag was cleared before the end of the transfer.
When such errors occur, the whole key sharing process starting from the SAES
peripheral must be restarted, through the IPRST bits of both control registers.

4.

As KEYVALID is set, the key share target peripheral is initialized with a valid, shared
key. The application can proceed with the data processing of data, setting KMOD[1:0]
to 00.

This sequence can be run multiple times (for example, to manage a suspend/resume
situation), as long as SAES is unused and duly remains in key sharing state.
Note:

in SAES peripheral, when KMOD[1:0] = 10 (shared key) and MODE[1:0] = 10 (decryption),
a read access to the SAES_DOUTR register triggers a read error (RDERR).
When KEYSEL[2:0] = 001 (DHUK) or 100 (DHUK XOR BHK), the application software
must use the same privilege, security, KMOD[1:0] / KSHAREID[1:0], CHMOD[2:0], and
KEYSIZE context for encryption and decryption. Otherwise, the result is incorrect.
When KMOD[1:0] = 10 and BUSY = 1 in the AES peripheral and KEYSIZE value of AES
and SAES differs, the key sharing fails and the KEIF flag is raised in both peripherals.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

50.4.11

Secure AES coprocessor (SAES)

SAES data registers and data swapping
Data input and output
A 128-bit data block is entered into the SAES peripheral with four successive 32-bit word
writes into the SAES_DINR register (bitfield DIN[31:0]), the most significant word (bits
[127:96]) first, the least significant word (bits [31:0]) last.
A 128-bit data block is retrieved from the SAES peripheral with four successive 32-bit word
reads from the SAES_DOUTR register (bitfield DOUT[31:0]), the most significant word (bits
[127:96]) first, the least significant word (bits [31:0]) last.
The 32-bit data word for SAES_DINR register or from SAES_DOUTR register is organized
in big endian order, that is:
•
the most significant byte of a word to write into SAES_DINR must be put on the lowest
address out of the four adjacent memory locations keeping the word to write, or
•

the most significant byte of a word read from SAES_DOUTR goes to the lowest
address out of the four adjacent memory locations receiving the word

For using DMA for input data block write into SAES, the four words of the input block must
be stored in the memory consecutively and in big-endian order, that is, the most significant
word on the lowest address. See Section 50.4.14: SAES DMA interface.

Data swapping
The SAES peripheral can be configured to perform a bit-, a byte-, a half-word-, or no
swapping on the input data word in the SAES_DINR register, before loading it to the AES
processing core, and on the data output from the AES processing core, before sending it to
the SAES_DOUTR register. The choice depends on the type of data. For example, a byte
swapping is used for an ASCII text stream.
The data swap type is selected through the DATATYPE[1:0] bitfield of the SAES_CR
register. The selection applies both to the input and the output of the AES core.
For different data swap types, Figure 497 shows the construction of AES processing core
input buffer data P127 to P0, from the input data entered through the SAES_DINR register,
or the construction of the output data available through the SAES_DOUTR register, from the
AES processing core output buffer data P127 to P0.

RM0456 Rev 6

<!-- pagebreak -->

2015

Secure AES coprocessor (SAES)

RM0456

Figure 497. 128-bit block construction with respect to data swap
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

The data in SAES key registers (SAES_KEYRx) and initialization registers (SAES_IVRx)
are not sensitive to the swap mode selection.

Data padding
Figure 497 also gives an example of memory data block padding with zeros such that the
zeroed bits after the data swap form a contiguous zone at the MSB end of the AES core
input buffer. The example shows the padding of an input data block containing:

<!-- pagebreak -->

