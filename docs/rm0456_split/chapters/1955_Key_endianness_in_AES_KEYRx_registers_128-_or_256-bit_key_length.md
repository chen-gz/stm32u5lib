•

48 message bits, with DATATYPE[1:0] = 01

•

56 message bits, with DATATYPE[1:0] = 10

•

34 message bits, with DATATYPE[1:0] = 11

RM0456 Rev 6

RM0456

AES hardware accelerator (AES)

49.4.15

AES key registers
The AES_KEYRx write-only registers store the encryption or decryption key bitfield
KEY[127:0] or KEY[255:0]. The data to write to each register is organized in the memory in
little-endian order, that is, with most significant byte on the highest address (reads are not
allowed for security reason).
The key is spread over eight registers as shown in Table 474.
Table 474. Key endianness in AES_KEYRx registers (128- or 256-bit key length)

AES_KEYR7
[31:0]

AES_KEYR6
[31:0]

AES_KEYR5
[31:0]

AES_KEYR4
[31:0]

AES_KEYR3
[31:0]

AES_KEYR2
[31:0]

AES_KEYR1
[31:0]

AES_KEYR0
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

The key for encryption or decryption may be written into these registers when the AES
peripheral is disabled, by clearing the EN bit of the AES_CR register.
The key registers are not affected by the data swapping controlled by DATATYPE[1:0]
bitfield of the AES_CR register.
The entire key must be written before starting an AES computation. In normal key mode
(KMOD[1:0] = 00), the AES_KEYRx (x = 0 to 3 for KEYSIZE = 0, x = 0 to 7 for
KEYSIZE = 1) registers must always be written in either ascending or descending order.
Note:

Initiating the key-loading sequence sets the BUSY flag and clears the KEYVALID flag.
Once the amount of bits defined by KEYSIZE is transfered to the AES_KEYRx registers,
BUSY is cleared, KEYVALID set and the EN bit becomes writable. If an error occurs,
BUSY and KEYVALID are cleared and KEIF set (see Section 49.4.18: AES error
management for details).
For additional information on key modes, refer to Section 49.4.13.

49.4.16

AES initialization vector registers
The four AES_IVRx registers keep the initialization vector input bitfield IVI[127:0]. The data
to write to or to read from each register is organized in the memory in little-endian order, that
is, with most significant byte on the highest address. The registers are also ordered from
lowest address (AES_IVR0) to highest address (AES_IVR3).
The signification of data in the bitfield depends on the chaining mode selected. When used,
the bitfield is updated upon each computation cycle of the AES core.
Write operations to the AES_IVRx registers when the AES peripheral is enabled have no
effect to the register contents. For modifying the contents of the AES_IVRx registers, the EN
bit of the AES_CR register must first be cleared.
Reading the AES_IVRx registers returns the latest counter value (useful for managing
suspend mode).
The AES_IVRx registers are not affected by the data swapping feature controlled by the
DATATYPE[1:0] bitfield of the AES_CR register.

RM0456 Rev 6

<!-- pagebreak -->

1973

AES hardware accelerator (AES)

49.4.17

RM0456

AES DMA interface
The AES peripheral provides an interface to connect to the DMA (direct memory access)
controller. The DMA operation is controlled through the AES_CR register.

Data input using DMA
Setting the DMAINEN bit of the AES_CR register enables DMA writing into AES. The AES
peripheral then initiates a DMA request during the input phase each time it requires to write
a 128-bit block (quadruple word) to the AES_DINR register, as shown in Figure 483.
Note:

According to the algorithm and the mode selected, special padding / ciphertext stealing
might be required. For example, in case of AES GCM encryption or AES CCM decryption, a
DMA transfer must not include the last block. For details, refer to Section 49.4.4: AES
procedure to perform a cipher operation.
Figure 483. DMA transfer of a 128-bit data block during input phase

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
Setting the DMAOUTEN bit of the AES_CR register enables DMA reading from AES. The
AES peripheral then initiates a DMA request during the Output phase each time it requires
to read a 128-bit block (quadruple word) to the AES_DINR register, as shown in Figure 484.
Note:

<!-- pagebreak -->

According to the message size, extra bytes might need to be discarded by application in the
last block.

RM0456 Rev 6

RM0456

AES hardware accelerator (AES)
Figure 484. DMA transfer of a 128-bit data block during output phase

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

System

Chronological order
Increasing address

D0
LSB

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
via the MODE[1:0] bitfield of the register AES_CR. As in Mode 2 (key derivation) the
AES_KEYRx registers must be written by software, enabling the DMA transfer through the
DMAINEN and DMAOUTEN bits of the AES_CR register have no effect in that mode.
DMA single requests are generated by AES until it is disabled. So, after the data output
phase at the end of processing of a 128-bit data block, AES switches automatically to a new
data input phase for the next data block, if any.
When the data transferring between AES and memory is managed by DMA, the CCF flag
has no use because the reading of the AES_DOUTR register is managed by DMA
automatically at the end of the computation phase. The CCF flag must only be cleared when
transiting back to data transferring managed by software. See Section 49.4.4: AES
procedure to perform a cipher operation, subsection Data append, for details.

49.4.18

AES error management
AES configuration can be changed at any moment by clearing the EN bit of the AES_CR
register.

Read error flag (RDERR)
Unexpected read attempt of the AES_DOUTR register sets the RDERR flag of the AES_SR
register and the RWEIF flag of the AES_ISR register, and returns zero.
RDERR is triggered during the computation phase or during the input phase.
Note:

AES is not disabled upon a RDERR error detection and continues processing.
An interrupt is generated if the RWEIE bit of the AES_IER register is set. For more details,
refer to Section 49.5: AES interrupts.
The RDERR and RWEIF flag is cleared by setting the RWEIE bit of the AES_ICR register.

Write error flag (WDERR)
Unexpected write attempt of the AES_DINR register sets the WRERR flag of the AES_SR
register and the RWEIF flag of the AES_ISR register, and has no effect on the AES_DINR
register. The WRERR is triggered during the computation phase or during the output phase.
RM0456 Rev 6

<!-- pagebreak -->

