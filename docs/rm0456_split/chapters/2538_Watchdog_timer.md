RM0456 Rev 6

MSv71400V2

RM0456

64.4.2

Tamper and backup registers (TAMP)

TAMP pins and internal signals
Table 642. TAMP input/output pins
Pin name

Signal type

Description

TAMP_INx (x = pin index)

Input

Tamper input pin

TAMP_OUTx (x = pin index)

Output

Tamper output pin (active mode only)

Table 643. TAMP internal input/output signals
Internal signal name

Signal type

Description

tamp_ker_ck

Input

TAMP kernel clock, connected to rtc_ker_ck
and also named RTCCLK in this document

tamp_pclk

Input

TAMP APB clock, connected to rtc_pclk

tamp_itamp[y]
(y = signal index)

Inputs

Internal tamper event sources

tamp_tzen

Input

TAMP TrustZone enabled

tamp_evt

Output

Tamper event detection flag (internal or
external tamper), whatever confirmed or
potential mode configuration.

Output

Potential tamper detection signal, used for
device secrets(1) protection.
This signal is active when:
– a tamper event detection flag (internal or
external tamper), is generated in potential
mode.
– or a software request is done by writing
BKBLOCK to 1

tamp_potential

Confirmed tamper detection signal, used for

tamp_confirmed

tamp_potential_ercfgz
(z = signal index)

device secrets(1) protection.
This signal is active when:
– a tamper event detection flag (internal or
external tamper), is generated in confirmed
mode.
– or a software request is done by writing
BKERASE to 1

Output

Potential tamper detection signal generated
only when ERCFGz = 1.
This signal is active when:
– a tamper event detection flag (internal or
external tamper), is generated in potential
mode.
– or a software request is done by writing
BKBLOCK to 1

Output

RM0456 Rev 6

<!-- pagebreak -->

