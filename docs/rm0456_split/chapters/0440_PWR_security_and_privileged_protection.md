482

Power control (PWR)

1.

RM0456

CSLEEP, CDSTOP, and SRDSTOP are generated in core domain, consequently they are not driven in Stop 3, Standby,
and Shutdown modes.

10.8

PWR security and privileged protection

10.8.1

PWR security protection
When the TrustZone security is activated by TZEN in FLASH_OPTR, some PWR register
fields can be secured against nonsecure access.
The PWR TrustZone security allows the following features to be secured
through PWR_SECCFGR:
•

low-power mode

•

wake-up (WKUP) pins

•

voltage detection and monitoring

•

VBAT mode

•

I/O pull-up/pull-down configuration

Other PWR configuration bits are secure when:
•

The system clock selection is secure in RCC: the voltage scaling (VOS) configuration
and the regulator booster (BOOSTEN) are secure.

•

A GPIO is configured as secure: its corresponding bit for pull-up/pull-down
configuration in Standby mode is secure.

•

The UCPD1 is secure in the GTZC: the PWR_UCPDR register is secure.

Table 109 gives a summary of the PWR secured bits following the security configuration bit
in PWR_SECCFGR. As soon as at least one function is configured to be secure, the PWR
clock control is also secure in the RCC.
A nonsecure access to a secure-protected register bit is denied:
•

The secured bits are not written (WI) with a nonsecure write access.

•

The secured bits are read as 0 (RAZ) with a nonsecure read access.

A nonsecure write access to PWR_SECCFGR is WI and generates an illegal access event
and an interrupt if enabled in the GTZC. It can be read with a nonsecure read access.
When the TrustZone security is disabled (TZEN = 0), PWR_SECCFGR is RAZ/WI, and
all other registers are nonsecure.
.

Table 109. PWR Security configuration summary

Secure
configuration
register

Security
configuration bit

Register name

Secured bits

Nonsecure
access on
secure bits

PWR_SECCFGR

Not applicable(1)

PWR_SECCFGR

All bits

Read OK.
WI and illegal
access event

PWR_SECCFGR

At least one bit is set

PWR_PRIVCFGR

SPRIV

Read OK.
WI

<!-- pagebreak -->

