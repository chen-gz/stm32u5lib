609

Reset and clock control (RCC)

11.7

RM0456

RCC interrupts
The table below summarizes the interrupt sources and the way to control them.
Table 118. Interrupt sources and control

Interrupt
vector

Description

Enable control
bit

Interrupt clear
method

Exit
Sleep
mode

Exit Stop,
Standby,
Shutdown
modes

LSIRDYF

LSI ready

LSIRDYIE and
LSISEC = 0

Set LSIRDYC to 1

Yes

No

LSERDYF

LSE ready

LSERDYIE and
LSESEC = 0

Set LSERDYC to 1

Yes

No

HSIDRYF

HSI ready

HSIDRYIE and
HSISEC = 0

Set HSIRDYC to 1

Yes

No

HSERDYF

HSE ready

HSERDYIE and
HSESEC = 0

Set HSERDYC to 1

Yes

No

MSISRDYF

MSIS ready

MSISRDYIE
Set MSISRDYC to 1
and MSISEC = 0

Yes

No

MSIKRDYF

MSIK ready

MSIKRDYIE
Set MSIKRDYC to 1
and MSISEC = 0

Yes

No

SHSIRDYF

SHSI ready

SHSIRDYIE and
SAESSEC = 0
(in GTZC)

Set SHSIRDYC to 1

Yes

No

HSI48RDYF

HSI48 ready

HSI48RDYIE
and HSI48SEC
=0

Set HSI48RDYC to 1

Yes

No

PLL1RDYF

PLL1 ready

PLL1RDYIE and
PLL1SEC = 0

Set PLL1RDYC to 1

Yes

No

PLL2RDYF

PLL2 ready

PLL2RDYIE and
PLL2SEC = 0

Set PLL2RDYC to 1

Yes

No

PLL3RDYF

PLL3 ready

PLL3RDYIE and
PLL3SEC = 0

Set PLL3DYC to 1

Yes

No

LSIRDYF

LSI ready

LSIRDYIE and
LSISEC = 1

Set LSIRDYC to 1

Yes

No

LSERDYF

LSE ready

LSERDYIE and
LSESEC = 1

Set LSERDYC to 1

Yes

No

HSIDRYF

HSI ready

HSIDRYIE and
HSISEC = 1

Set HSIRDYC to 1

Yes

No

HSERDYF

HSE ready

HSERDYIE and
HSESEC = 1

Set HSERDYC to 1

Yes

No

MSISRDYF

MSIS ready

MSISRDYIE
Set MSISRDYC to 1
and MSISEC = 1

Yes

No

MSIKRDYF

MSIK ready

MSIKRDYIE
Set MSIKRDYC to 1
and MSISEC = 1

Yes

No

Interrupt
event flag

RCC

RCC_S

(1)

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)
Table 118. Interrupt sources and control (continued)
Exit
Sleep
mode

Exit Stop,
Standby,
Shutdown
modes

SHSIRDYIE and
Set SHSIRDYC to 1
SAESSEC(2) = 1

Yes

No

HSI48 ready

HSI48RDYIE
and HSI48SEC
=1

Set HSI48RDYC to 1

Yes

No

PLL1RDYF

PLL1 ready

PLL1RDYIE and
PLL1SEC = 1

Set PLL1RDYC to 1

Yes

No

PLL2RDYF

PLL2 ready

PLL2RDYIE and
PLL2SEC = 1

Set PLL2RDYC to 1

Yes

No

PLL3RDYF

PLL3 ready

PLL3RDYIE and
PLL3SEC = 1

Set PLL3RDYC to 1

Yes

No

LSECSSON and
LSE CSS failure ITAMP3E(3) and
ITAMP3IE(3)

Set CITAMP3F(3) to
1

Yes

Yes

-(4)

Set CSSC to 1

Yes

No

LSECSS(5) Through EXTI LSE CSS failure

Through EXTI

Through EXTI

Yes

Yes(6)/No

MSI_PLL_
Through EXTI
UNLOCK(5)

Through EXTI

Through EXTI

Yes

Yes(6)/No

Interrupt
vector

RCC_S(1)

1.

Interrupt
event flag

Description

Enable control
bit

SHSIRDYF

SHSI ready

HSI48RDYF

TAMP

ITAMP3F(3)

NMI

CSSF

HSE CSS
failure

MSI PLL-mode
unlock(7)

Interrupt clear
method

The RCC secure interrupt vector is used only when TrustZone is enabled.

2. The SAESSEC bit is in the GTZC peripheral.
3. The LSE CSS failure event (LSECSSD) is connected to TAMP internal tamper 3. In order to get the interrupt associated to
this event, the internal tamper 3 must be enabled, and the internal tamper 3 interrupt must be enabled. The ITAMP3F,
ITAMP3E, ITAMP3IE, and CITAMP3F bits are in the TAMP peripheral. Consequently, the LSE CSS tamper interrupt erases
or blocks the device secrets as described in Table 644: TAMP interconnection.
4. It is not possible to mask this interrupt when the security system feature is enabled (CSSON = 1).
5. Not available in STM32U575/585 rev. X devices. Available in all other STM32U575/585 revisions, and in the other
STM32U5 Series devices.
6. This interrupt can wake up from Stop 0, Stop 1, and Stop 2 modes only.
7. This interrupt indicates that the MSI has left the PLL_mode, due to LSE missing pulses. As a consequence, the MSI
frequency accuracy is degraded.

RM0456 Rev 6

<!-- pagebreak -->

