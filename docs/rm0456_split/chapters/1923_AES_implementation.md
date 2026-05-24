AES main features
•

Compliance with NIST “Advanced encryption standard (AES), FIPS publication 197”
from November 2001

•

128-bit data block processing

•

Support for cipher key lengths of 128-bit and 256-bit

•

Encryption and decryption with multiple chaining modes:
–

Electronic codebook (ECB) mode

–

Cipher block chaining (CBC) mode

–

Counter (CTR) mode

–

Galois counter mode (GCM)

–

Galois message authentication code (GMAC) mode

–

Counter with CBC-MAC (CCM) mode

•

51 or 75 clock cycle latency in ECB mode for processing one 128-bit block of data with,
respectively, 128-bit or 256-bit key

•

Integrated round key scheduler to compute the last round key for ECB/CBC decryption

•

AMBA AHB slave peripheral, accessible through 32-bit word single accesses only

•

256-bit write-only register for storing the cryptographic key (eight 32-bit registers)

•

128-bit register for storing initialization vector (four 32-bit registers)

•

32-bit buffer for data input and output

•

Automatic data flow control with support of single-transfer direct memory access (DMA)
using two channels (one for incoming data, one for processed data)

•

Data-swapping logic to support 1-, 8-, 16- or 32-bit data

•

Possibility for software to suspend a message if AES needs to process another
message with a higher priority, then resume the original message

•

Hardware key sharing with side-channel resistant SAES peripheral (Shared-key
mode), controlled by SAES

RM0456 Rev 6

RM0456

49.3

AES hardware accelerator (AES)

AES implementation
The devices have one AES and one SAES peripheral, implemented as shown in the
following table.
Table 468. AES/SAES features
AES/SAES modes/features(1)

AES

SAES

ECB, CBC chaining

X

X

CTR, CCM, GCM chaining

X

-

AES 128-bit ECB encryption in cycles

51

528

DHUK and BHK key selection

-

X

Side-channel attacks resistance

-

X

Shared key between SAES and AES

X

1. X = supported.

49.4

AES functional description

49.4.1

AES block diagram
Figure 459 shows the block diagram of AES.
Figure 459. AES block diagram

AES
32-bit
AHB bus

AHB
interface
aes_hclk

aes_in_dma
aes_out_dma
aes_it

DMA
interface

KEY

SAES

32-bit
access

Shared key
Banked
registers

key

AES_KEYRx

KEY

IV, counter

AES_IVRx

IVI

status

AES_SR

control

AES_CR

data in

AES_DINR

DIN

data out

AES_DOUTR

DOUT

AES_SUSPRx

Save / Restore

swap

AES
Core
(AEA)

Control Logic

IRQ
interface

aes_itamp_out
MSv47991V1

49.4.2

AES internal signals
Table 469 describes the user relevant internal signals interfacing the AES peripheral.

RM0456 Rev 6

<!-- pagebreak -->

