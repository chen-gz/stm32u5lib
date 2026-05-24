RM0456 Rev 6

RM0456

AES hardware accelerator (AES)
GMAC is similar to GCM, except that it is applied on a message composed only by plaintext
authenticated data (that is, only header, no payload).

Counter with CBC-MAC (CCM) principle
Figure 465. CCM encryption and authentication principle
B0

key

Init
(Encrypt)

key

Plaintext block 1
Initialization
vector

Legend
input

+1

Count 1

key

key

Encrypt

Plaintext block 2

Ciphertext block 1

Encrypt

+1

Count 2

Count 3

key

Encrypt

Encrypt

Plaintext block 3

Ciphertext block 2

Encrypt

Ciphertext block 3

Encrypt

output
XOR

Final

TAG
MSv42145V1

In Counter with cipher block chaining-message authentication code (CCM) mode, the
plaintext message is encrypted while a message authentication code (MAC) is computed in
parallel, thus generating the corresponding ciphertext and the corresponding MAC (also
known as tag). It is described by NIST in Special Publication 800-38C, Recommendation for
Block Cipher Modes of Operation - The CCM Mode for Authentication and Confidentiality.
CCM mode is based on AES in counter mode for confidentiality and it uses CBC for
computing the message authentication code. It requires an initial value.
Like GCM, the CCM chaining mode can be applied on a message composed only by
plaintext authenticated data (that is, only header, no payload). Note that this way of using
CCM is not called CMAC (it is not similar to GCM/GMAC), and its use is not recommended
by NIST.

49.4.4

AES procedure to perform a cipher operation
Introduction
A typical cipher operation is explained below. Detailed information is provided in sections
starting from Section 49.4.8: AES basic chaining modes (ECB, CBC).

RM0456 Rev 6

<!-- pagebreak -->

1973

AES hardware accelerator (AES)

RM0456

Initialization of AES
To initialize AES, first disable it by clearing the EN bit of the AES_CR register. Then perform
the following steps in any order:
•

Note:

Configure the AES mode, by programming the MODE[1:0] bitfield of the AES_CR
register.
–

For encryption, select Mode 1 (MODE[1:0] = 00).

–

For decryption, select Mode 3 (MODE[1:0] = 10), unless ECB or CBC chaining
modes are used. In this latter case, perform an initial key derivation of the
encryption key, as described in Section 49.4.5: AES decryption round key
preparation.

•

Select the chaining mode, by programming the CHMOD[2:0] bitfield of the AES_CR
register.

•

Configure the data type (1-, 8-, 16- or 32-bit), with the DATATYPE[1:0] bitfield in the
AES_CR register.

•

When it is required (for example in CBC or CTR chaining modes), write the initialization
vector into the AES_IVRx registers.

•

Configure the key size (128-bit or 256-bit), with the KEYSIZE bitfield of the AES_CR
register. This step must be done before writing into key registers.

•

Write a symmetric key into the AES_KEYRx registers (4 or 8 registers depending on
the key size).

AES sets KEYVALID in AES_SR when key information defined by KEYSIZE is loaded in
AES_KEYRx.

Data append
This section describes different ways of appending data for processing, where the size of
data to process is not a multiple of 128 bits when KMOD[1:0] = 00. For other KMOD[1:0]
values refer to Section 49.4.13.
For ECB or CBC mode, refer to Section 49.4.6: AES ciphertext stealing and data padding.
The last block management in these cases is more complex than in the sequence described
in this section.
Data append through polling
This method uses flag polling to control the data append through the following sequence:
1.

Enable the AES peripheral by setting the EN bit of the AES_CR register.

2.

Repeat the following sub-sequence until the payload is entirely processed:

3.

<!-- pagebreak -->

a)

Write four input data words into the AES_DINR register.

b)

Wait until the status flag CCF is set in the AES_SR, then read the four data words
from the AES_DOUTR register.

c)

Clear the CCF flag, by setting the CCF bit of the AES_ICR register.

d)

If the data block just processed is the second-last block of the message and the
significant data in the last block to process is inferior to 128 bits, pad the
remainder of the last block with zeros and, in case of GCM payload encryption or
CCM payload decryption, specify the number of non-valid bytes, using the NPBLB
bitfield of the AES_CR register, for AES to compute a correct tag;.

As it is the last block, discard the data that is not part of the data, then disable the AES
peripheral by clearing the EN bit of the AES_CR register.

RM0456 Rev 6

RM0456
Note:

AES hardware accelerator (AES)
Up to three wait cycles are automatically inserted between two consecutive writes to the
AES_DINR register, to allow sending the key to the AES processor.
NPBLB bits are not used in header phase of GCM, GMAC and CCM chaining modes.
Data append using interrupt
The method uses interrupt from the AES peripheral to control the data append, through the
following sequence:

Note:

1.

Enable interrupts from AES by setting the CCFIE bit of the AES_IER register.

2.

Enable the AES peripheral by setting the EN bit of the AES_CR register.

3.

Write first four input data words into the AES_DINR register.

4.

Handle the data in the AES interrupt service routine, upon interrupt:
a)

Read four output data words from the AES_DOUTR register.

b)

Clear the CCF flag and thus the pending interrupt, by setting the CCF bit of the
AES_ICR register.

c)

If the data block just processed is the second-last block of an message and the
significant data in the last block to process is inferior to 128 bits, pad the
remainder of the last block with zeros and, in case of GCM payload encryption or
CCM payload decryption, specify the number of non-valid bytes, using the NPBLB
bitfield of the AES_CR register, for AES to compute a correct tag;. Then proceed
with point 4e).

d)

If the data block just processed is the last block of the message, discard the data
that is not part of the data, then disable the AES peripheral by clearing the EN bit
of the AES_CR register and quit the interrupt service routine.

e)

Write next four input data words into the AES_DINR register and quit the interrupt
service routine.

AES is tolerant of delays between consecutive read or write operations, which allows, for
example, an interrupt from another peripheral to be served between two AES computations.
NPBLB bits are not used in header phase of GCM, GMAC and CCM chaining modes.
Data append using DMA
With this method, all the transfers and processing are managed by DMA and AES. To use
the method, proceed as follows:
1.

Prepare the last four-word data block (if the data to process does not fill it completely),
by padding the remainder of the block with zeros.

2.

Configure the DMA controller so as to transfer the data to process from the memory to
the AES peripheral input and the processed data from the AES peripheral output to the
memory, as described in Section 49.4.17: AES DMA interface. Configure the DMA
controller so as to generate an interrupt on transfer completion. In case of GCM
payload encryption or CCM payload decryption, DMA transfer must not include the
last four-word block if padded with zeros. The sequence described in Data append
through polling must be used instead for this last block, because NPBLB bits must be
setup before processing the block, for AES to compute a correct tag.

3.

Enable the AES peripheral by setting the EN bit of the AES_CR register

4.

Enable DMA requests by setting the DMAINEN and DMAOUTEN bits of the AES_CR
register.

5.

Upon DMA interrupt indicating the transfer completion, get the AES-processed data
from the memory.

RM0456 Rev 6

<!-- pagebreak -->

