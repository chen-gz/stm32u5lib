RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)

7.8

FLASH interrupts
Table 78. Flash interrupt requests

Interrupt
vector

Exit Stop
and
Standby
modes

Event flag

Event flag/interrupt
clearing method

Secure end
of operation

Secure EOP(1)

Write secure EOP = 1

Secure
EOPIE

Yes

No

Secure
operation
error

Secure OPERR(2)

Write secure OPERR = 1

Secure
ERRIE

Yes

No

Nonsecure
end of
operation

Nonsecure
EOP(1)

Write nonsecure EOP = 1

Nonsecure
EOPIE

Yes

No

Nonsecure
operation
error

Nonsecure
OPERR(2)

Write nonsecure
OPERR = 1

Nonsecure
ERRIE

Yes

No

ECC
correction

ECCC

Write ECCC=1

ECCCIE

Yes

No

FLASH_S

FLASH

Interrupt
Exit Sleep
enable
mode
control bit

Interrupt
event

1. Secure EOP (resp. nonsecure EOP) is set only if secure EOPIE (resp. nonsecure EOPIE) is set.
2. Secure OPERR (resp. nonsecure OPERR) is set only if secure ERRIE (resp. nonsecure ERRIE) is set.

7.9

FLASH registers

7.9.1

FLASH access control register (FLASH_ACR)
Address offset: 0x00
Reset value: 0x0000 0000
Access: no wait state when no flash memory read is ongoing; word, half-word, and byte
access
This register is nonsecure. It can be read and written by both secure and nonsecure access.
This register can be protected against unprivileged access when NSPRIV = 1
in FLASH_PRIVCFGR register.

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

PRFTE
N

Res.

Res.

Res.

Res.

15

14

13

12

Res.

SLEEP
_PD

PDRE
Q2

PDRE
Q1

LPM

rw

rs

rs

rw

Res.

rw

LATENCY[3:0]
rw

rw

rw

rw

--

Bits 31:15 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

363

Embedded flash memory (FLASH)

RM0456

Bit 14 SLEEP_PD: Flash memory power-down mode during Sleep mode
This bit determines whether the flash memory is in power-down mode or Idle mode when the
device is in Sleep mode.
0: Flash memory in Idle mode during Sleep mode
1: Flash memory in power-down mode during Sleep mode
Caution: The flash memory must not be put in power-down while a program or an erase
operation is ongoing.
Bit 13 PDREQ2: Bank 2 power-down mode request
This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter powerdown mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the
PDKEY2R is locked.
0: No request for bank 2 to enter power-down mode
1: Bank 2 requested to enter power-down mode
Bit 12 PDREQ1: Bank 1 power-down mode request
This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter powerdown mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the
PDKEY1R is locked.
0: No request for bank 1 to enter power-down mode
1: Bank 1 requested to enter power-down mode
Bit 11 LPM: Low-power read mode
This bit puts the flash memory in low-power read mode.
0: Flash memory not in low-power read mode
1: Flash memory in low-power read mode
Bits 10:9 Reserved, must be kept at reset value.
Bit 8 PRFTEN: Prefetch enable
This bit enables the prefetch buffer in the embedded flash memory.
0: Prefetch disabled
1: Prefetch enabled
Bits 7:4 Reserved, must be kept at reset value.
Bits 3:0 LATENCY[3:0]: Latency
These bits represent the ratio between the HCLK (AHB clock) period and the flash memory
access time.
0000: Zero wait state
0001: One wait state
0010: Two wait states
...
1111: Fifteen wait states

<!-- pagebreak -->

