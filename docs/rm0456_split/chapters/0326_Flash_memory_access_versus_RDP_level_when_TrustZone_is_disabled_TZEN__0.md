363

Embedded flash memory (FLASH)

RM0456

1. RDP level 1 no intrusion = when booting from user flash memory and no debug access.
2. RDP level 1 with intrusion = when debug access detected.
3. Others refers to the other flash memory secure configurations than the one described for HDP protections.
Example: Flash memory secure and HDP area enabled but ACCDIS = 0.

Table 69. Flash memory access versus RDP level when TrustZone is disabled (TZEN = 0)
Access
type

RDP level 0, level 1 no intrusion(1), or level 2

Fetch

OK

Read
Write
Erase

RDP level 1 with intrusion(2)

Bus error

No WRP: OK
WRP pages: WI and nonsecure WRPERR flag set

WI and nonsecure WRPERR flag set

1. RDP Level 1 no intrusion = when booting from user flash memory and no debug access.
2. RDP Level 1 with intrusion = when booting from RAM or system memory or debug access detected.

Table 70. Flash memory mass erase versus RDP level when TrustZone is active (TZEN = 1)
RDP level 0, level 0.5, level 1 no intrusion(1), or level 2
Secure flash memory

Secure

Nonsecure
flash memory

Bank
or
mass
erase

WI, secure
WRPERR flag
set, flash
memory illegal
access event

Nonsecure

Access
type

RDP level 1 with
intrusion(2)

Bank
or
mass
erase

No WRP: OK
WRP pages: WI
and nonsecure
WRPERR flag
set

HDP area
(HDPxEN = 1 and
ACCDIS = 1)

Others(3)

WI, secure
WRPERR flag set

No WRP: OK
WRP pages: WI
and secure
WRPERR flag
set

Mix nonsecure
and secure flash
memory

Nonsecure or
secure flash
memory

WI, secure
WRPERR flag set,
WI, secure WRPERR
flash memory
flag set
illegal access
event

WI, nonsecure WRPERR flag set,
flash memory illegal access event

WI, nonsecure
WRPERR flag set

1. RDP Level 1 no intrusion = when booting from user flash memory and no debug access.
2. RDP Level 1 with intrusion = when debug access detected.
3. Others refers to the other flash memory secure configurations than the one described for HDP protections.
Example: Flash memory secure and HDP area enabled but ACCDIS = 0.

<!-- pagebreak -->

