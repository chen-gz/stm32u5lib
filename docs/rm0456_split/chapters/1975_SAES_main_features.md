RM0456 Rev 6

RM0456

50.2

Secure AES coprocessor (SAES)

SAES main features
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

•

528 or 743 clock cycle latency in ECB mode for processing one 128-bit block of data
with, respectively, 128-bit or 256-bit key

•

Integrated round key scheduler to compute the last round key for ECB/CBC decryption

•

AMBA AHB slave peripheral, accessible through 32-bit word single accesses only

•

256-bit write-only register for storing the cryptographic key (eight 32-bit registers)
–

50.3

Optional hardware loading of two hardware secret keys (BHK, DHUK) that can be
XOR-ed together

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

Possibility for software to suspend a message if SAES needs to process another
message with a higher priority, then resume the original message

•

Security context enforcement for keys

•

Hardware secret key encryption/ decryption (Wrapped-key mode)

•

Protection against side-channel attacks (SCA), incl. differential power analysis (DPA),
certified SESIP and PSA security assurance level 3

•

Hardware key sharing with faster AES peripheral (Shared-key mode), controlled by
SAES

SAES implementation
The devices have one SAES and one AES peripheral, implemented as shown in the
following table.
Table 479. AES/SAES features
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

RM0456 Rev 6

X

<!-- pagebreak -->

