•

48 message bits, with DATATYPE[1:0] = 01

•

56 message bits, with DATATYPE[1:0] = 10

•

34 message bits, with DATATYPE[1:0] = 11

RM0456 Rev 6

RM0456

Secure AES coprocessor (SAES)

50.4.12

SAES key registers
The SAES_KEYRx write-only registers store the encryption or decryption key bitfield
KEY[127:0] or KEY[255:0]. The data to write to each register is organized in the memory in
little-endian order, that is, with most significant byte on the highest address (reads are not
allowed for security reason).
The key is spread over eight registers as shown in Table 481.

Table 481. Key endianness in SAES_KEYRx registers (128- or 256-bit key length)
SAES_KEYR SAES_KEYR SAES_KEYR SAES_KEYR SAES_KEYR SAES_KEYR SAES_KEYR SAES_KEYR
7
6
5
4
3
2
1
0
[31:0]
[31:0]
[31:0]
[31:0]
[31:0]
[31:0]
[31:0]
[31:0]
-

-

-

-

KEY[127:96]

KEY[95:64]

KEY[63:32]

KEY[31:0]

KEY[255:224]

KEY[223:192]

KEY[191:160]

KEY[159:128]

KEY[127:96]

KEY[95:64]

KEY[63:32]

KEY[31:0]

TAMP_BKP7R
[31:0]

TAMP_BKP6R
[31:0]

TAMP_BKP5R
[31:0]

TAMP_BKP4R
[31:0]

TAMP_BKP3R
[31:0]

TAMP_BKP2R
[31:0]

TAMP_BKP1R
[31:0]

TAMP_BKP0R
[31:0]

The key for encryption or decryption may be written into these registers when the SAES
peripheral is disabled, by clearing the EN bit and the KEYSEL[2:0] bitfield of the SAES_CR
register.
The key registers are not affected by the data swapping controlled by DATATYPE[1:0]
bitfield of the SAES_CR register.
The entire key must be written before starting an AES computation. In normal key mode
(KMOD[1:0] = 00), with KEYSEL[2:0] = 000, the SAES_KEYRx (x = 0 to 3 for KEYSIZE = 0,
x = 0 to 7 for KEYSIZE = 1) registers must always be written in either ascending or
descending order.
With KEYSEL[2:0] set to 001, a derived hardware unique key (DHUK), computed inside the
SAES engine from a non-volatile OTP-based root hardware unique key, is loaded directly
into key registers, based on KEYSIZE information.
With KEYSEL[2:0] set to 010, the boot hardware key (BHK), stored in tamper-resistant
secure backup registers, is entirely transferred into key registers upon a secure application
performing a single read of all TAMP_BKPxR registers (x = 0 to 3 for KEYSIZE = 0,
x = 0 to 7 for KEYSIZE = 1) in either ascending or descending order. Refer to Table 481.
With KEYSEL[2:0] set to 100, the XOR combination of DHUK and BHK is entirely
transferred into key registers upon a secure application performing a single read of all
TAMP_BKPxR registers (x = 0 to 3 for KEYSIZE = 0, x = 0 to 7 for KEYSIZE = 1) in either
ascending or descending order. Refer to Table 481.
Repeated writing of KEYSEL[2:0] with the same non-zero value only triggers the loading of
DHUK or BHK if KEYVALID = 0. The recommended method to clear KEYVALID is to set
the IPRST bit in the SAES_CR register. Such method is required for example when
switching from ECB decryption to ECB encryption, selecting the same BHK
(KEYSEL[2:0] = 010).

RM0456 Rev 6

<!-- pagebreak -->

2015

Secure AES coprocessor (SAES)

RM0456

The KEYPROT bit of SAES_CR register must be set if the key to load in key registers must
not be shared with an application executing in a different security context (that is, different
security attribute). Setting KEYPROT and KEYVALID makes KEIF flag an error upon SAES
access attempts by an application executing in a different security context than the one that
loaded the key, as shown in Figure 498.
Note:

KEYSEL[2:0] values different from zero (normal key) automatically protect the key
registers.
DHUK, BHK and their XOR combination are not readable by any software (even secure).
Secure SAES uses secure DHUK. Nonsecure SAES uses nonsecure DHUK.
Figure 498. Key protection mechanisms
Security context B
(for example non-secure)

SAES

Security context A
(for example secure)
KEYPROT = 0
and KEYSEL[2:0] = 000

SAES
KEYPROT = 1
or KEYSEL 
KEYVALID = 1

KEIF

SAES

MSv63151V1

Note:

Initiating the key-loading sequence sets the BUSY flag and clears the KEYVALID flag.
Once the amount of bits defined by KEYSIZE is transfered to the SAES_KEYRx registers,
BUSY is cleared, KEYVALID set and the EN bit becomes writable. If an error occurs,
BUSY and KEYVALID are cleared and KEIF set (see Section 50.4.15: SAES error
management for details).This holds for all KEYSEL values.
For additional information on key modes, refer to Section 50.4.10 and Section 50.4.9.

50.4.13

SAES initialization vector registers
The four SAES_IVRx registers keep the initialization vector input bitfield IVI[127:0]. The data
to write to or to read from each register is organized in the memory in little-endian order, that
is, with most significant byte on the highest address. The registers are also ordered from
lowest address (SAES_IVR0) to highest address (SAES_IVR3).
The signification of data in the bitfield depends on the chaining mode selected. When used,
the bitfield is updated upon each computation cycle of the AES core.
Write operations to the SAES_IVRx registers when the SAES peripheral is enabled have no
effect to the register contents. For modifying the contents of the SAES_IVRx registers, the
EN bit of the SAES_CR register must first be cleared.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure AES coprocessor (SAES)
Reading the SAES_IVRx registers returns the latest counter value (useful for managing
suspend mode).
The SAES_IVRx registers are not affected by the data swapping feature controlled by the
DATATYPE[1:0] bitfield of the SAES_CR register.

50.4.14

SAES DMA interface
The SAES peripheral provides an interface to connect to the DMA (direct memory access)
controller. The DMA operation is controlled through the SAES_CR register.

Data input using DMA
Setting the DMAINEN bit of the SAES_CR register enables DMA writing into SAES. The
SAES peripheral then initiates a DMA request during the input phase each time it requires to
write a 128-bit block (quadruple word) to the SAES_DINR register, as shown in Figure 499.
Note:

According to the algorithm and the mode selected, special padding / ciphertext stealing
might be required. For details, refer to Section 50.4.6: SAES ciphertext stealing and data
padding.
Figure 499. DMA transfer of a 128-bit data block during input phase

D127
MSB

Word3
DIN[127:96]

DMA
single write

DMA req N

D96

D95

DMA req N+1

1

Memory accessed through DMA
Word2
Word1
DIN[95:64]
DIN[63:32]
D64 D63

D32

DMA
single write

DMA req N+3

DMA req N+2

2

DMA
single write

D31

3

Word0
DIN[31:0]

D0
LSB

System

Chronological order
Increasing address

DMA
single write

4

(No swapping)

1

2

3

4

AES core input buffer
I127

I96

I95

I64

I63

I32

I31

I0

AES
peripheral

AES_DINR

LSB

MSB

1

4

Order of write to AES_DINR

MSv42160V1

Data output using DMA
Setting the DMAOUTEN bit of the SAES_CR register enables DMA reading from SAES.
The SAES peripheral then initiates a DMA request during the Output phase each time it
requires to read a 128-bit block (quadruple word) to the SAES_DINR register, as shown in
Figure 500.
Note:

According to the message size, extra bytes might need to be discarded by application in the
last block.

RM0456 Rev 6

<!-- pagebreak -->

2015

Secure AES coprocessor (SAES)

RM0456

Figure 500. DMA transfer of a 128-bit data block during output phase

D127
MSB

Word3
DOUT[127:96]

DMA
single read

DMA req N

D96

D95

DMA req N+1

Memory accessed through DMA
Word2
Word1
DOUT[95:64]
DOUT[63:32]
D64 D63

DMA
single read

1

2

1

2

DMA req N+2

DMA
single read

D32

D31

DMA req N+3

Word0
DOUT[31:0]

D0
LSB

System

Chronological order
Increasing address

DMA
single read

3

4

3

4

AES core output buffer
O127

O96

O95

O64

O63

O32

O31

O0

AES
peripheral

AES_DOUTR
(No swapping)

LSB

MSB

1

4

Order of read from AES_DOUTR

MSv42161V1

DMA operation in different operating modes
DMA operations are usable when Mode 1 (encryption) or Mode 3 (decryption) are selected
via the MODE[1:0] bitfield of the register SAES_CR. As in Mode 2 (key derivation) the
SAES_KEYRx registers must be written by software, enabling the DMA transfer through the
DMAINEN and DMAOUTEN bits of the SAES_CR register have no effect in that mode.
DMA single requests are generated by SAES until it is disabled. So, after the data output
phase at the end of processing of a 128-bit data block, SAES switches automatically to a
new data input phase for the next data block, if any.
When the data transferring between SAES and memory is managed by DMA, the CCF flag
has no use because the reading of the SAES_DOUTR register is managed by DMA
automatically at the end of the computation phase. The CCF flag must only be cleared when
transiting back to data transferring managed by software. See Section 50.4.4: SAES
procedure to perform a cipher operation, subsection Data append, for details.

50.4.15

SAES error management
Unless indicated otherwise, SAES configuration can be changed at any moment by clearing
the EN bit of the SAES_CR register.

Read error flag (RDERR)
Unexpected read attempt of the SAES_DOUTR register sets the RDERR flag of the
SAES_SR register and the RWEIF flag of the SAES_ISR register, and returns zero.
RDERR is triggered during the computation phase or during the input phase.
Note:

Unless indicated otherwise, SAES is not disabled upon a RDERR error detection and
continues processing.
An interrupt is generated if the RWEIE bit of the SAES_IER register is set. For more details,
refer to Section 50.5: SAES interrupts.
The RDERR and RWEIF flag is cleared by setting the RWEIE bit of the SAES_ISR register.

Write error flag (WDERR)
Unexpected write attempt of the SAES_DINR register sets the WRERR flag of the
SAES_SR register and the RWEIF flag of the SAES_ISR register, and has no effect on the

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure AES coprocessor (SAES)
SAES_DINR register. The WRERR is triggered during the computation phase or during the
output phase.

Note:

Unless indicated otherwise, SAES is not disabled after a WRERR error detection and
continues processing.
An interrupt is generated if the RWEIE bit of the SAES_IER register is set. For more details,
refer to Section 50.5: SAES interrupts.
The WRERR and RWEIF flag is cleared by setting the RWEIF bit of the SAES_ISR register.

Key error interrupt flag (KEIF)
Failure to load a key into key registers, or attempt to load a key while the key is protected,
sets the KEIF flag of the SAES_ISR register and clears the KEYVALID bit of the SAES_SR
register.
The KEIF flag is cleared with corresponding bit of the SAES_ISR register. An interrupt is
generated if the KEIE bit of the SAES_IER register is set. For more details, refer to
Section 50.5: SAES interrupts.
The possible sources of key errors are:
•

Key protection error: while KEYVALID is set, if KEYPROT = 1 or KEYSEL is different
from zero this error is triggered when an application executing in a security context
different from the one used to load the key is detected accessing the SAES (that is,
different security attribute).

•

Key writing sequence error: an incorrect sequence has been detected when writing key
registers. See Section 50.4.12: SAES key registers for details.

•

Key sharing size mismatch: error is triggered when KMOD[1:0] = 10 and application
sets a KEYSIZE information in AES peripheral that does not match the KEYSIZE
stored in SAES peripheral

•

Key sharing error: the copy of key registers from SAES peripheral to AES failed. See
Section 50.4.10: SAES operation with shared keys for details.

•

Hardware secret key loading error: the DHUK or BHK failed to load into SAES.
KEYSEL = 001 (DHUK), 010 (BHK) or 100 (DHUK XOR BHK) is not functional.

Upon a key sharing error, reset both AES and SAES peripherals through the IPRST bit of
their corresponding control register, then restart the key sharing sequence.
Upon a key selection error, clearing the KEIF flag automatically restarts the key selection
process. Persisting problems (for example, RHUK load failing) may require a power-on
reset.
Note:

For any key error, clear KEIF flag prior to re-configuring SAES.

RNG error interrupt flag (RNGEIF)
SAES fetches random numbers from the RNG peripheral automatically after an IP reset
triggered in the RCC. SAES cannot be used when RNGEIF is set.
An error detected while fetching a random number from RNG peripheral (due to, for
example, bad entropy) sets the RNGEIF flag of the SAES_ISR register. The flag is cleared
by setting the corresponding bit of the SAES_ICR register. An interrupt is generated if the

RM0456 Rev 6

<!-- pagebreak -->

