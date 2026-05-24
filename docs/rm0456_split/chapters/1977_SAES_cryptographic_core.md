Signal name

Signal type

Description

saes_hclk

Input

AHB bus clock

saes_it

Output

SAES interrupt request

saes_in_dma

Input/Output

Input DMA single request/acknowledge

saes_out_dma

Input/Output

Output DMA single request/acknowledge

RM0456 Rev 6

RM0456

Secure AES coprocessor (SAES)
Table 480. SAES internal input/output signals (continued)
Signal name

Signal type

Description

saes_itamp_out

Output

Tamper event signal to TAMP (XOR-ed), triggered when an
unexpected hardware fault occurs. When this signal is
triggered, SAES automatically clears key registers. A reset
is required for SAES to be usable again.

rcc_shsi_ck

Input

Dedicated SHSI clock from RCC

RHUK

Input

256-bit root hardware unique key (non-volatile, unique per
device and secret to software), used to internally compute
the derived hardware unique key (DHUK)

Input

256-bit boot hardware key (BHK) stored in tamper-resistant
secure backup registers and written by a secure code
during boot. Once written, this key cannot be read nor
written by any application until the next product reset.

(1)

BHK

1. Connected to a set of backup registers in TAMP peripheral that are written, then read/write locked, by the
application software (see Section 50.4.12 for details).

50.4.3

SAES cryptographic core
Overview
The SAES cryptographic core consists of the following components:
•

AES core algorithm (AEA)

•

key input

•

initialization vector (IV) input

•

chaining algorithm logic (XOR)

The SAES core works on 128-bit data blocks (four words) with 128-bit or 256-bit key length.
Depending on the chaining mode, the SAES requires zero or one 128-bit initialization vector
IV.
The SAES features the following modes of operation:
•

Mode 1:
Plaintext encryption using a key stored in the SAES_KEYRx registers or read using a
dedicated hardware bus

•

Mode 2:
ECB or CBC decryption key preparation. It must be used prior to selecting Mode 3 with
ECB or CBC chaining modes. The key prepared for decryption is stored automatically
in the SAES_KEYRx registers. Now the SAES peripheral is ready to switch to Mode 3
for executing data decryption.

•

Mode 3:
Ciphertext decryption using a key stored in the SAES_KEYRx registers. When ECB
and CBC chaining modes are selected, the key must be prepared beforehand, through
Mode 2.

Note:

Mode 2 is only used when performing ECB and CBC decryption.
The operating mode is selected by programming the MODE[1:0] bitfield of the SAES_CR
register. It may be done only when the SAES peripheral is disabled.

RM0456 Rev 6

<!-- pagebreak -->

2015

Secure AES coprocessor (SAES)

RM0456

Special key operation is selected using the KMOD[1:0] bitfield of the SAES_CR register.
See Section 50.4.10 and Section 50.4.9 for details.

Typical data processing
Typical usage of the SAES is described in Section 50.4.4: SAES procedure to perform a
cipher operation on page 1979.
Note:

The outputs of the intermediate AEA stages are never revealed outside the cryptographic
boundary, with the exclusion of the IVI bitfield.

Chaining modes
The following chaining modes are supported by SAES, selected through the CHMOD[2:0]
bitfield of the SAES_CR register:

Note:

•

Electronic code book (ECB)

•

Cipher block chaining (CBC)

The chaining mode may be changed only when SAES is disabled (bit EN of the SAES_CR
register cleared).
Principle of each SAES chaining mode is provided in the following subsections.
Detailed information is in dedicated sections, starting from Section 50.4.8: SAES basic
chaining modes (ECB, CBC).

Electronic codebook (ECB) mode
Figure 486. ECB encryption and decryption principle
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

ECB is the simplest mode of operation. There are no chaining operations, and no special
initialization stage. The message is divided into blocks and each block is encrypted or
decrypted separately.

<!-- pagebreak -->

