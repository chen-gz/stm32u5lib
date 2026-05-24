RM0456 Rev 6

RM0456

AES hardware accelerator (AES)
Table 475. AES interrupt requests

Interrupt
acronym

AES

AES interrupt event

Event flag

Enable bit

Interrupt clear
method

computation completed flag

CCF

CCFIE

set CCF(1)

read error flag

RDERR(2)

write error flag

WRERR(2)

RWEIE

set RWEIF(1)

key error flag

KEIF

KEIE

set KEIF(1)

1. Bit of the AES_ICR register.
2. Flag of the AES_SR register, mirrored by the flag RWEIF of the AES_ISR register.

49.6

AES processing latency
The tables below summarize the latency to process a 128-bit block for each mode of
operation.
Table 476. Processing latency for ECB, CBC and CTR

Key size

Algorithm

Clock
cycles

ECB, CBC, CTR

51

-

59

Mode 3: Decryption

ECB, CBC, CTR

51

Mode 1: Encryption

ECB, CBC, CTR

75

-

82

ECB, CBC, CTR

75

Mode of operation
Mode 1: Encryption

128-bit

256-bit

Mode 2: Key derivation

Mode 2: Key derivation
Mode 3: Decryption

Table 477. Processing latency for GCM and CCM (in clock cycles)
Algorithm

Init Phase

Header
phase(1)

Payload
phase(1)

Tag phase(1)

Mode 1: Encryption/
Mode 3: Decryption

GCM

64

35

51

59

CCM

63

55

114

58

Mode 1: Encryption/
Mode 3: Decryption

GCM

88

35

75

75

CCM

87

79

162

82

Key size

Mode of operation

128-bit

256-bit

1. Data insertion can include wait states forced by AES on the AHB bus (maximum 3 cycles, typical 1 cycle).

RM0456 Rev 6

<!-- pagebreak -->

