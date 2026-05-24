When the ECCC flag is set, a further two-errors detection is not able to generate the NMI or
break signal to timers. It is therefore recommended to clear the ECCC flag as soon as a
correction is operated, to preserve the ECC error detection capability. In case of a double

RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)
ECC error detection (ECCD flag set and NMI triggered), the software must clean the cache
in the NMI handler. Refer to STM32U5 Series safety manual (UM2875) for the full
description of the implications on safety standards compliance.

Note:

For an erased flash line, one error is detected and corrected but two errors detection is not
supported. When an ECC error is reported, a new read at the failing address may not
generate an ECC error if the data is still present in the current buffer, even if ECCC and
ECCD are cleared.
The following addresses in the system flash memory are used to store words including ECC
errors to allow run-time tests by software on ECC correction detection capability:
•

0x0BFA1F00 (embeds a word with 1-bit error)

•

0x0BFA1F80 (embeds a word with 2-bit error)

In case the second address is read, for instance by the debugger memory viewer,
an NMI is generated.

7.3.3

Read access latency
To correctly read data from flash memory, the number of wait states (latency) must be
correctly programmed in FLASH_ACR according to the frequency of the CPU clock (HCLK),
and the internal voltage range of the device VCORE. Refer to Section 10.5.4: Dynamic
voltage scaling management.
The table below shows the correspondence between wait states and CPU clock frequency.
Table 54. Number of wait states according to CPU clock (HCLK) frequency (LPM = 0)
HCLK (MHz)

Wait states (WS) (latency)
VCORE range 1

VCORE range 2

VCORE range 3

VCORE range 4

0 WS (1 CPU cycle)

≤ 32

≤ 30

≤ 24

≤ 12

1 WS (2 CPU cycles)

≤ 64

≤ 60

≤ 48

≤ 25

2 WS (3 CPU cycles)

≤ 96

≤ 90

≤ 55

-

3 WS (4 CPU cycles)

≤ 128

≤ 110

-

-

4 WS (5 CPU cycles)

≤ 160

-

-

-

RM0456 Rev 6

<!-- pagebreak -->

