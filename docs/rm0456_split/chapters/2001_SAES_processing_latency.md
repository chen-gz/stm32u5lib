RM0456 Rev 6

RM0456

Secure AES coprocessor (SAES)

50.6

SAES processing latency
The tables below summarize the latency to process a 128-bit block for each mode of
operation.
Table 483. Processing latency for ECB, CBC
Algorithm

Clock
cycles(1)

ECB, CBC

528

-

200

Mode 3: Decryption

ECB, CBC

528

Mode 1: Encryption

ECB, CBC

743

-

324

ECB, CBC

743

Key size

Mode of operation
Mode 1: Encryption

128-bit

256-bit

Mode 2: Key derivation

Mode 2: Key derivation
Mode 3: Decryption

1. SHSI clock from RCC (48 MHz with +/-15% jitter)

50.7

SAES registers

50.7.1

SAES control register (SAES_CR)
Address offset: 0x00

28

26

KSHAREID[1:0]

25

24

23

22

KMOD[1:0]

21

20

Res.

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

Res.

Res.

Res.

DMAINEN

KEYSEL[2:0]

27

DMAOUTEN

IPRST

29

Res.

Res.

Res.

Res.

rw

rw

6

5

4

19

18

17

16

Res.

KEYSIZE

30

KEYPROT

31

CHMOD[2]

Reset value: 0x0000 0000

rw

rw

3

2

CHMOD[1:0]

MODE[1:0]

rw

rw

rw

rw

rw
1

DATATYPE[1:0]

rw

rw

0

EN

rw

Bit 31 IPRST: SAES peripheral software reset
Setting the bit resets the SAES peripheral, putting all registers to their default values, except the
IPRST bit itself and the SAES_DPACFG register. Hence, any key-relative data is lost. For this
reason, it is recommended to set the bit before handing over the SAES to a less secure application.
The bit must be low while writing any configuration registers.

RM0456 Rev 6

<!-- pagebreak -->

2015

Secure AES coprocessor (SAES)

RM0456

Bits 30:28 KEYSEL[2:0]: Key selection
The bitfield defines the source of the key information to use in the AES cryptographic core.
000: Software key, loaded in key registers SAES_KEYx
001: Derived hardware unique key (DHUK)
010: Boot hardware key (BHK)
100: XOR of DHUK and BHK
111: Test mode key (256-bit hardware constant 0xA5A5...A5A5)
Others: Reserved (if used, unfreeze SAES with IPRST)
When KEYSEL is different from zero, selected key value is available in key registers when BUSY bit
is cleared and KEYVALID is set in the SAES_SR register. Otherwise, the key error flag KEIF is set.
Repeated writing of KEYSEL[2:0] with the same non-zero value only triggers the loading of DHUK or
BHK if KEYVALID = 0.
When the application software changes the key selection by writing the KEYSEL[2:0] bitfield, the
key registers are immediately erased and the KEYVALID flag cleared.
At the end of the decryption process, if KMOD[1:0] is other than zero, KEYSEL[2:0] is cleared.
With the bitfield value other than zero and KEYVALID set, the application cannot transfer the
ownership of SAES with a loaded key to an application running in another security context (such as
secure, nonsecure). More specifically, when security of an access to any register does not match
the information recorded by SAES, the KEIF flag is set.
Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as
when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that
write access.
Bits 27:26 KSHAREID[1:0]: Key share identification
This bitfield defines, at the end of a decryption process with KMOD[1:0] = 10 (shared key), which
target can read the SAES key registers using a dedicated hardware bus.
00: AES peripheral
Others: Reserved
Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as
when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that
write access.
Bits 25:24 KMOD[1:0]: Key mode selection
The bitfield defines how the SAES key can be used by the application:
00: Normal key
01: Wrapped key
10: Shared key
Others: Reserved
With normal key selection, the key registers are freely usable, no specific usage or protection
applies to SAES_DIN and SAES_DOUT registers.
With wrapped key selection, the key loaded in key registers can only be used to encrypt or decrypt
AES keys. Hence, when a decryption is selected in Wrapped-key mode read-as-zero SAES_DOUT
register is automatically loaded into SAES key registers after a successful decryption process.
With shared key selection, after a successful decryption process, SAES key registers are shared
with the peripheral described in KSHAREID(1:0] bitfield. This sharing is valid only while
KMOD[1:0] = 10 and KEYVALID = 1. When a decryption is selected, read-as-zero SAES_DOUT
register is automatically loaded into SAES key registers after a successful decryption process.
With KMOD[1:0] other than zero, any attempt to configure the SAES peripheral for use by an
application belonging to a different security domain (secure or nonsecure) results in automatic key
erasure and setting of the KEIF flag.
Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as
when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that
write access.
Bits 23:20 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure AES coprocessor (SAES)

Bit 19 KEYPROT: Key protection
When set, hardware-based key protection is enabled.
0: When KEYVALID is set and KEYSEL = 0 application can transfer the ownership of the SAES,
with its loaded key, to an application running in another security context (such as nonsecure,
secure).
1: When KEYVALID is set, key error flag (KEIF) is set when an access to any registers is detected,
this access having a security context (for example, secure, nonsecure) that does not match the one
of the application that loaded the key.
Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as
when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that
write access.
Bit 18 KEYSIZE: Key size selection
This bitfield defines the length of the key used in the SAES cryptographic core, in bits:
0: 128
1: 256
When KMOD[1:0] = 01 or 10 KEYSIZE also defines the length of the key to encrypt or decrypt.
Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as
when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that
write access.
Bit 17 Reserved, must be kept at reset value.
Bit 16 Reserved, must be kept at reset value.
Bits 15:13 Reserved, must be kept at reset value.
Bit 12 DMAOUTEN: DMA output enable
This bit enables/disables data transferring with DMA, in the output phase:
0: Disable
1: Enable
When the bit is set, DMA requests are automatically generated by SAES during the output data
phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE[1:0]
bitfield. It is not effective for Mode 2 (key derivation).
Bit 11 DMAINEN: DMA input enable
This bit enables/disables data transferring with DMA, in the input phase:
0: Disable
1: Enable
When the bit is set, DMA requests are automatically generated by SAES during the input data
phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE[1:0]
bitfield. It is not effective for Mode 2 (key derivation).
Bits 10:7 Reserved, must be kept at reset value.
Bit 6 Reserved, must be kept at reset value.
Bits 16, 6:5 CHMOD[2:0]: Chaining mode selection
This bitfield selects the AES chaining mode:
000: Electronic codebook (ECB)
001: Cipher-block chaining (CBC)
others: Reserved
Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as
when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that
write access.

RM0456 Rev 6

<!-- pagebreak -->

