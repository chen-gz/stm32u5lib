609

Reset and clock control (RCC)

RM0456

Table 117 summarizes the RCC secured bits following the security configuration bit
in RCC_SECCFGR register.
When one security configuration bit is set, some configuration and status bits are secured.
The RCC registers may contain secure and nonsecure bits:
•

Secured bits: read and write operations are only allowed by a secure access.
Nonsecure read returns 0 and write accesses are ignored. No illegal access event is
generated.

•

Nonsecure bits: no restriction. Read and write operations are allowed by both secure
and nonsecure accesses.

•

A nonsecure write access to RCC_SECCFGR is ignored and generates an illegal
access event. An illegal access interrupt is generated if the RCC illegal access interrupt
is enabled in the GTZC TZIC registers. RCC_SECCFGR can be read by secure or
nonsecure access.

When the TrustZone security is disabled (TZEN = 0 in FLASH_OPTR), all registers are
nonsecure. RCC_SECCFGR write accesses are ignored.
Table 117. RCC security configuration summary
Configuration bit in
RCC_SECCFGR

HSISEC

HSESEC

MSISEC

LSISEC

<!-- pagebreak -->

Secured bits

Corresponding
register

HSION, HSIKERON, HSIRDY

RCC_CR

HSICAL[7:0], HSITRIM[6:0]

RCC_ICSCR3

HSIRDYIE

RCC_CIER

HSIRDYIF

RCC_CIFR

HSIRDYC

RCC_CICR

HSEON, HSERDY, HSEBYP, CSSON, HSEEXT

RCC_CR

HSERDYIE

RCC_CIER

HSERDYIF, CSSF

RCC_CIFR

HSERDYC, CSSC

RCC_CICR

MSISON, MSIKERON, MSISRDY, MSIPLLEN, MSIKON,
MSIKRDY, MSIPLLSEL, MSIPLLFAST

RCC_CR

MISIRANGE[3:0], MISIKRANGE[3:0], MSIRGSEL, MSIBIAS,
MSICAL0[4:0], MSICAL1[4:0], MSICAL2[4:0], MSICAL3[4:0]

RCC_ICSCR1

MSITRIM0[4:0], MSITRIM1[4:0], MSITRIM2[4:0], MSITRIM3[4:0]

RCC_ICSCR2

MSISRDYIE, MSIKRDYIE

RCC_CIER

MSISRDYIF, MSIKRDYIF

RCC_CIFR

MSISRDYIC, MSIKRDYIC

RCC_CICR

MSISSRANGE[3:0], MSIKSRANGE[3:0]

RCC_CSR

LSION, LSIRDY, LSIPREDIV, LSCOSEL, LSCOEN

RCC_BDCR

LSIRDYIE

RCC_CIER

LSIRDYIF

RCC_CIFR

LSIRDYC

RCC_CICR

RM0456 Rev 6

RM0456

Reset and clock control (RCC)
Table 117. RCC security configuration summary (continued)

Configuration bit in
RCC_SECCFGR

LSESEC

SYSCLKSEC

PRESCSEC

PLL1SEC

PLL2SEC

PLL3SEC

Secured bits

Corresponding
register

LSECSSON, LSECSSD, LSEDRV[1:0], LSEBYP, LSERDY,
LSEON, LSEGFON, LSESYSRDY, LSESYSEN, LSCOSEL,
LSCOEN

RCC_BDCR

LSERDYIE

RCC_CIER

LSERDYF

RCC_CIFR

LSERDYC

RCC_CICR

SW[1:0], SWS[1:0], STOPWUCK, STOPKERWUCK,
MCOSEL[3:0], MCOPRE[2:0]

RCC_CFGR1

SYSTICKSEL[1:0]

RCC_CCIPR1

VOS[1:0]

PWR_VOSR

HPRE[3:0], PPRE1[2:0], PPRE2[2:0]

RCC_CFGR2

PPRE3[2:0]

RCC_CFGR3

PLL1SRC[1:0], PLL1RGE[1:0], PLL1FRACEN, PLL1M[3:0],
PLL1MBOOST[3:0], PLL1PEN, PLL1QEN, PLL1REN

RCC_PLL1CFGR

PLL1N[8:0], PLL1P[6:0], PLL1Q[6:0], PLL1R[6:0]

RCC_PLL1DIVR

PLL1FRACN[12:0]

RCC_PLL1FRACR

PLL1RDY, PLL1ON

RCC_CR

PLL1RDYIE

RCC_CIER

PLL1RDYF

RCC_CIFR

PLL1RDYC

RCC_CICR

PLL2SRC[1:0], PLL2RGE[1:0], PLL2FRACEN, PLL2M[3:0],
PLL2PEN, PLL2QEN, PLL2REN

RCC_PLL2CFGR

PLL2N[8:0], PLL2P[6:0], PLL2Q[6:0], PLL2R[6:0]

RCC_PLL2DIVR

PLL2FRACN[12:0]

RCC_PLL2FRACR

PLL2RDY, PLL2ON

RCC_CR

PLL2RDYIE

RCC_CIER

PLL2RDYF

RCC_CIFR

PLL2RDYC

RCC_CICR

PLL3SRC[1:0], PLL3RGE[1:0], PLL3FRACEN, PLL3M[3:0],
PLL3PEN, PLL3QEN, PLL3REN

RCC_PLL3CFGR

PLL3N[8:0], PLL3P[6:0], PLL3Q[6:0], PLL3R[6:0]

RCC_PLL3DIVR

PLL3FRACN[12:0]

RCC_PLL3FRACR

PLL3RDY, PLL3ON

RCC_CR

PLL3RDYIE

RCC_CIER

PLL3RDYF

RCC_CIFR

PLL3RDYC

RCC_CICR

RM0456 Rev 6

<!-- pagebreak -->

