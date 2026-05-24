1973

AES hardware accelerator (AES)

RM0456

Bits 4:3 MODE[1:0]: AES operating mode
This bitfield selects the AES operating mode:
00: Mode 1: encryption
01: Mode 2: key derivation (or key preparation for ECB/CBC decryption)
10: Mode 3: decryption
11: Reserved
Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as
when the EN bit of the AES_CR register is set before the write access and it is not cleared by that
write access.
Bits 2:1 DATATYPE[1:0]: Data type selection
This bitfield defines the format of data written in the AES_DINR register or read from the
AES_DOUTR register, through selecting the mode of data swapping:
00: None
01: Half-word (16-bit)
10: Byte (8-bit)
11: Bit
For more details, refer to Section 49.4.14: AES data registers and data swapping.
Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as
when the EN bit of the AES_CR register is set before the write access and it is not cleared by that
write access.
Bit 0 EN: AES enable
This bit enables/disables the AES peripheral:
0: Disable
1: Enable
At any moment, clearing then setting the bit re-initializes the AES peripheral.
This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2)
and upon the completion of GCM/GMAC/CCM initial phase.
The bit cannot be set as long as KEYVALID = 0.
Note: With KMOD[1:0] other than 00, use the IPRST bit rather than the bit EN.

49.7.2

AES status register (AES_SR)
Address offset: 0x04

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

KEYVALID

Reset value: 0x0000 0000

Res.

Res.

Res.

BUSY

r

Bits 31:8 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

r

WRERR RDERR

r

r

CCF

r

RM0456

AES hardware accelerator (AES)

Bit 7 KEYVALID: Key Valid flag
This bit is set by hardware when the amount of key information defined by KEYSIZE in AES_CR has
been loaded in AES_KEYx key registers.
0: No valid key information is available in key registers. EN bit in AES_CR cannot be set.
1: Valid key information, defined by KEYSIZE in AES_CR, is loaded in key registers.
In normal mode when KEYSEL equals to zero, the application must write the key registers in the
correct sequence, otherwise the KEIF flag of the AES_ISR register is set and KEYVALID stays at
zero.
When KEYSEL is different from zero the BUSY flag is automatically set by AES. When key is loaded
successfully, the BUSY flag is cleared and KEYVALID set. Upon an error, the KEIF flag of the
AES_ISR register is set, the BUSY flag cleared and KEYVALID kept at zero.
When the KEIF flag is set, the application must clear it through the AES_ICR register, otherwise
KEYVALID cannot be set. See the KEIF bit description for more details.
For more information on key loading, refer to Section 49.4.15: AES key registers.
Bits 6:4 Reserved, must be kept at reset value.
Bit 3 BUSY: Busy
This flag indicates whether AES is idle or busy during GCM payload encryption phase:
0: Idle
1: Busy
When the flag indicates “idle”, the current GCM encryption processing may be suspended to
process a higher-priority message. In other chaining modes, or in GCM phases other than payload
encryption, the flag must be ignored for the suspend process.
The flag is set when transferring a shared key from SAES peripheral.
Bit 2 WRERR: Write error
This flag indicates the detection of an unexpected write operation to the AES_DINR register (during
computation or data output phase):
0: Not detected
1: Detected
The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the AES_ICR
register.
Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the AES_ICR
register.
The flag setting has no impact on the AES operation. Unexpected write is ignored.
Bit 1 RDERR: Read error flag
This flag indicates the detection of an unexpected read operation from the AES_DOUTR register
(during computation or data input phase):
0: Not detected
1: Detected
The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the AES_ICR
register.
Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the AES_ICR
register.
The flag setting has no impact on the AES operation. Unexpected read returns zero.
Bit 0 CCF: Computation completed flag
This bit mirrors the CCF bit of the AES_ISR register.

RM0456 Rev 6

<!-- pagebreak -->

