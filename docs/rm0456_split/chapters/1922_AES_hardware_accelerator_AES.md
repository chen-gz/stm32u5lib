1921

AES hardware accelerator (AES)

49

RM0456

AES hardware accelerator (AES)
This section only applies to STM32U545/585/5Ax/5Gx devices.

49.1

Introduction
The AES hardware accelerator (AES) encrypts or decrypts data, using an algorithm and
implementation fully compliant with the advanced encryption standard (AES) defined in
Federal information processing standards (FIPS) publication 197.
The peripheral supports CTR, GCM, GMAC, CCM, ECB, and CBC chaining modes for key
sizes of 128 or 256 bits.
AES is an AMBA AHB slave peripheral accessible through 32-bit single accesses only.
Other access types generate an AHB error, and other than 32-bit writes may corrupt the
register content.
The peripheral supports DMA single transfers for incoming and outgoing data (two DMA
channels required).

49.2

<!-- pagebreak -->

