RM0456 Rev 6

RM0456
Note:

Secure AES coprocessor (SAES)
For decryption, a special key scheduling is required before processing the first block.

Cipher block chaining (CBC) mode
Figure 487. CBC encryption and decryption principle
Encryption
Plaintext block 1

Plaintext block 2

Plaintext block 3

initialization
vector
key

Encrypt

key

Encrypt

Ciphertext block 1

Ciphertext block 2

Plaintext block 1

Plaintext block 2

key

Encrypt

Ciphertext block 3

Decryption
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

For decryption, a special key scheduling is required before processing the first block.

50.4.4

SAES procedure to perform a cipher operation
Introduction
A typical cipher operation is explained below. Detailed information is provided in sections
starting from Section 50.4.8: SAES basic chaining modes (ECB, CBC).

RM0456 Rev 6

<!-- pagebreak -->

2015

Secure AES coprocessor (SAES)

RM0456

Initialization of SAES
To initialize SAES, first disable it by clearing the EN bit of the SAES_CR register. Then
perform the following steps in any order (except KEYSIZE):

Note:

•

Verify that BUSY = 0 in SAES_SR (no RNG random number fetch in progress), and
RNGEIF = 0 in SAES_ISR (no random number fetching error flagged).

•

Configure the SAES mode, by programming the MODE[1:0] bitfield of the SAES_CR
register.
–

For encryption, select Mode 1 (MODE[1:0] = 00).

–

For decryption, select Mode 2 (MODE[1:0] = 01) then Mode 3 (MODE[1:0] = 10),
as described in Section 50.4.8: SAES basic chaining modes (ECB, CBC).

•

Select the chaining mode, by programming the CHMOD[2:0] bitfield of the SAES_CR
register. Select normal key mode by setting KMOD[1:0] to 00. For the other KMOD[1:0]
values, refer to Section 50.4.9 and Section 50.4.10.

•

Configure the data type (1-, 8-, 16- or 32-bit), with the DATATYPE[1:0] bitfield in the
SAES_CR register.

•

When it is required (for example in CBC chaining mode), write the initialization vector
into the SAES_IVRx registers.

•

Configure the key size (128-bit or 256-bit), with the KEYSIZE bitfield of the SAES_CR
register. This step must be done before writing into key registers or selecting a different
key source using KEYSEL. If the key must not be shared with a different security
context (different secure attribute), the KEYPROT bit of the SAES_CR register must
also be set.

•

Write a symmetric key into the SAES_KEYRx registers (4 or 8 registers depending on
the key size). Alternatively, select a key source different from the key registers by
setting KEYSEL[2:0] to a value different from 0x0. See Section 50.4.12 for details.

SAES sets KEYVALID in SAES_SR when key information defined by KEYSIZE is loaded in
SAES_KEYRx.

Data append
This section describes different ways of appending data for processing, where the size of
data to process is not a multiple of 128 bits when KMOD[1:0] = 00. For other KMOD[1:0]
values refer to Section 50.4.10 and Section 50.4.9.
Data append through polling
This method uses flag polling to control the data append through the following sequence:
1.
2.

3.

<!-- pagebreak -->

Enable the SAES peripheral by setting the EN bit of the SAES_CR register.
Repeat the following sub-sequence until the payload is entirely processed:
a)

Write four input data words into the SAES_DINR register.

b)

Wait until the status flag CCF is set in the SAES_SR, then read the four data
words from the SAES_DOUTR register.

c)

Clear the CCF flag, by setting the CCF bit of the SAES_ISR register.

d)

If the data block just processed is the second-last block of the message and the
significant data in the last block to process is inferior to 128 bits, refer to
Section 50.4.6: SAES ciphertext stealing and data padding.

As it is the last block, follow the instructions in Section 50.4.6: SAES ciphertext stealing
and data padding, then disable the SAES peripheral by clearing the EN bit of the
SAES_CR register.
RM0456 Rev 6

RM0456
Note:

Secure AES coprocessor (SAES)
Up to three wait cycles are automatically inserted between two consecutive writes to the
SAES_DINR register, to allow sending the key to the AES co-processor.
Data append using interrupt
The method uses interrupt from the SAES peripheral to control the data append, through the
following sequence:

Note:

1.

Enable interrupts from SAES by setting the CCFIE bit of the SAES_IER register.

2.

Enable the SAES peripheral by setting the EN bit of the SAES_CR register.

3.

Write first four input data words into the SAES_DINR register.

4.

Handle the data in the SAES interrupt service routine, upon interrupt:
a)

Read four output data words from the SAES_DOUTR register.

b)

Clear the CCF flag and thus the pending interrupt, by setting the CCF bit of the
SAES_ISR register.

c)

If the data block just processed is the second-last block of an message and the
significant data in the last block to process is inferior to 128 bits, refer to
Section 50.4.6: SAES ciphertext stealing and data padding. Then proceed with
point 4e).

d)

If the data block just processed is the last block of the message, follow if needed
Section 50.4.6: SAES ciphertext stealing and data padding, then disable the
SAES peripheral by clearing the EN bit of the SAES_CR register and quit the
interrupt service routine.

e)

Write next four input data words into the SAES_DINR register and quit the
interrupt service routine.

SAES is tolerant of delays between consecutive read or write operations, which allows, for
example, an interrupt from another peripheral to be served between two SAES
computations.
Data append using DMA
With this method, all the transfers and processing are managed by DMA and SAES. To use
the method, proceed as follows:

Note:

1.

If the last block to process is inferior to 128 bits, refer to Section 50.4.6: SAES
ciphertext stealing and data padding to prepare the last four-word data block.

2.

Configure the DMA controller so as to transfer the data to process from the memory to
the SAES peripheral input and the processed data from the SAES peripheral output to
the memory, as described in Section 50.4.14: SAES DMA interface. Configure the
DMA controller so as to generate an interrupt on transfer completion.

3.

Enable the SAES peripheral by setting the EN bit of the SAES_CR register

4.

Enable DMA requests by setting the DMAINEN and DMAOUTEN bits of the SAES_CR
register.

5.

Upon DMA interrupt indicating the transfer completion, get the SAES-processed data
from the memory.

The CCF flag has no use with this method, because the reading of the SAES_DOUTR
register is managed by DMA automatically, without any software action, at the end of the
computation phase.

RM0456 Rev 6

<!-- pagebreak -->

