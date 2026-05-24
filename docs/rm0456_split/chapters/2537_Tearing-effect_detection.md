2687

Tamper and backup registers (TAMP)

RM0456

64.4

TAMP functional description

64.4.1

TAMP block diagram
Figure 774. TAMP block diagram
tamp_ker_ck clock domain
Internal tamper
detection

tamp_itamp1
...
tamp_itampy
Monotonic
counter
overflow

TAMP1F

ITAMP1
...

ITAMP1F
...

ITAMPy(1)

ITAMPyF

Tamper detection

ITAMP8

ITAMP8F

EDGE detection
(passive mode)

TAMP_IN1

TAMP_IN1

tamp_trg1

LEVEL detection
(passive mode)

TAMP_OUT1

TAMP_OUT1

COMPARATOR
(active mode)

TAMP_OUT2

tampoutsel1

...

TAMP_IN2

TAMP_OUTx

TAMP_OUT2

TAMP2F
Tamper detection

(1)

EDGE detection
(passive mode)

...
TAMP_IN2
TAMP_INx

TAMP_OUT1

TAMP_OUTx

TAMP_OUT2

tamp_trg2

LEVEL detection
(passive mode)
COMPARATOR
(active mode)

tampoutsel2

...

...

TAMP_OUTx(1)

TAMPxF

Tamper detection
EDGE detection
(passive mode)

...
TAMP_INx

LEVEL detection
(passive mode)
TAMP_OUT1
COMPARATOR
(active mode)

TAMP_OUT2

tampoutselx

...
TAMP_OUTx(1)
tamp_ker_ck
PRNG

TAMP1F when TAMP1NOER=1

tamp_evt

TAMPxF
ITAMP1F

TAMPxF when TAMPxNOER=1
ITAMP1F when ITAMP1NOER=1

ERCFGz

TAMPxF when TAMPxNOER=0
ITAMP1F when ITAMP1NOER=0
...

...

...
ITAMPyF
tamp_potential_ercfgz

TAMP1F when TAMP1NOER=0
...

...

...

TAMP1F

ITAMPyF when ITAMPyNOER=1
BKBLOCK

ITAMPyF when ITAMPyNOER=0
BKERASE

tamp_potential
tamp_confirmed_ercfgz

ERCFGz

tamp_confirmed

overflow
tamp_it
IRQ interface
Backup registers

Monotonic counter

tamp_tzen
Registers interface
tamp_pclk
tamp_pclk clock domain

tamp_bhk

1. The number of external and internal tampers depends on products.

<!-- pagebreak -->

