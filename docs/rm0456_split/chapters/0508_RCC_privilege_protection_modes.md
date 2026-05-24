609

Reset and clock control (RCC)

RM0456

Table 117. RCC security configuration summary (continued)
Configuration bit in
RCC_SECCFGR

Secured bits

Corresponding
register

HSI48ON, HSI48RDY

RCC_CR

HSI48CAL[8:0]

RCC_CRRCR

HSI48RDYE

RCC_CIER

HSI48RDYF

RCC_CIFR

HSI48RDYC

RCC_CICR

ICLKSEL

ICLKSEL[1:0]

RCC_CCIPR1

RMVFSEC

RMVF

RCC_CSR

HSI48SEC(1)

1. TRIM field of the HSI48 is located in CRS peripheral. Be sure to secure it using CRSSEC bit in GTZC1 TZSC secure
configuration register 1.

11.5.2

RCC privilege protection modes
By default, after reset, all RCC registers can be read or written with both privileged and
unprivileged access, except RCC_PRIVCFGR that can be written with privileged access
only. RCC_PRIVCFGR can be read by secure and nonsecure, privileged and unprivileged
access.
SPRIV in RCC_PRIVCFGR can be written with secure privileged access only. This bit
configures the privileged access of all RCC secure functions (as defined by
RCC_SECCFGR), or by the GTZC for securable peripherals, or by the peripheral itself
in case of TrustZone-aware peripherals).
When SPRIV = 1 in RCC_PRIVCFGR:
•

Writing the RCC secure bits is possible only with privileged access, including
RCC_SECCFGR.

•

The RCC secure bits can be read only with privileged access, except RCC_SECCFGR
and RCC_PRIVCFGR that can be read by privileged or unprivileged access.

•

An unprivileged access to a privileged RCC bit or register is discarded: the bits are
read as zero and the write to these bits is ignored (RAZ/WI).

NSPRIV in RCC_PRIVCFGR can be written with privileged access only, secure or
nonsecure functions (as defined by RCC_SECCFGR, or by the GTZC for securable
peripherals, or by the peripheral itself in case of TrustZone-aware peripherals).
When NSPRIV = 1 in RCC_PRIVCFGR:

<!-- pagebreak -->

