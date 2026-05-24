RM0456 Rev 6

RM0456

Power control (PWR)
Table 109. PWR Security configuration summary (continued)

Secure
configuration
register

Security
configuration bit

PWR_SECCFGR

LPMSEC

PWR_SECCFGR

VDMSEC

PWR_SECCFGR

VBSEC

PWR_SECCFGR

APCSEC

WUPxSEC
(x = 1 to 8)

PWR_SECCFGR

Nonsecure
access on
secure bits

Register name

Secured bits

PWR_CR1

All bits

PWR_CR2

All bits

PWR_SR

CSSF

WI

PWR_CR3

All bits

RAZ/WI

PWR_SVMCR

All bits

RAZ/WI

PWR_BDCR1

All bits

RAZ/WI

PWR_BDCR2

All bits

RAZ/WI

PWR_DBPR

All bits

RAZ/WI

PWR_APCR

All bits

RAZ/WI

PWR_WUCR1

WUPENx

RAZ/WI

PWR_WUCR2

WUPPx

RAZ/WI

PWR_WUCR3

WUSELx

RAZ/WI

PWR_WUSCR

CWUFx

WI

RAZ/WI

GTZC_TZSC_
SECCFGR

UCPD1SEC

PWR_UCPDR

All bits

RAZ/WI

RCC_SECCFGR

SYSCLKSEC

PWR_VOSR

VOS[1:0], BOOSTEN,
USBPWREN,
USBBOOSTEN(2)

RAZ/WI

GPIOx_SECCFGR
(x=A,B..J)

SECy (y=0..15)

PWR_PUCRx (x = A to J)

PUy (y = 0 to 15)

RAZ/WI

PWR_PDCRx (x = A to J)

PDy (y = 0 to 15)

RAZ/WI

1. PWR_SECCFGR is always secure.
2. USBPWREN and USBBOOSTEN are available in STM32U59x/5Ax/5Fx/5Gx only.

10.8.2

WR privileged protection
By default, after a reset, all PWR registers can be read or written with both privileged and
unprivileged accesses, except PWR_PRIVCFGR that can be written with privileged access
only. PWR_PRIVCFGR can be read by secure and nonsecure, privileged and unprivileged
accesses.
SPRIV in PWR_PRIVCFGR can be written with secure privileged access only. This bit
configures the privileged access of all PWR secure functions (defined by PWR_SECCFGR,
GTZC, RCC or GPIO as shown in Table 109).
When SPRIV is set in PWR_PRIVCFGR:
•

The PWR secure bits can be written only with privileged access, including
PWR_SECCFGR.

•

The PWR secure bits can be read only with privileged access except PWR_SECCFGR
and PWR_PRIVCFGR that can be read by privileged or unprivileged access.

•

An unprivileged access to a privileged PWR bit or register is discarded: the bits are
read as zero and the write to these bits is ignored (RAZ/WI).
RM0456 Rev 6

<!-- pagebreak -->

