1973

AES hardware accelerator (AES)

RM0456

To resume the processing of a message, proceed as follows:

49.4.9

1.

If DMA is used, configure the DMA controller so as to complete the rest of the FIFO IN
and FIFO OUT transfers.

2.

Disable the AES peripheral by clearing the EN bit of the AES_CR register.

3.

Restore AES_CR register (with correct KEYSIZE) then restore AES_KEYRx registers.

4.

Prepare the decryption key as described in Section 49.4.5: AES decryption round key
preparation (only required for ECB or CBC decryption).

5.

Restore AES_IVRx registers using the saved configuration (only required in CBC
mode).

6.

Enable the AES peripheral by setting the EN bit of the AES_CR register.

7.

If DMA is used, enable AES DMA transfers by setting the DMAINEN and DMAOUTEN
bits of the AES_CR register.

AES counter (CTR) mode
Overview
The counter mode (CTR) uses AES as a key-stream generator. The generated keys are
then XOR-ed with the plaintext to obtain the ciphertext.
CTR chaining is defined in NIST Special Publication 800-38A, Recommendation for Block
Cipher Modes of Operation. A typical message construction in CTR mode is given in
Figure 473.
Figure 473. Message construction in CTR mode
16-byte boundaries
Ciphertext (C)

0

Zero
padding

decrypt

ICB

4-byte boundaries

Plaintext (P)
Initialization vector (IV)

Counter

MSv42156V1

The structure of this message is:
•

•

<!-- pagebreak -->

