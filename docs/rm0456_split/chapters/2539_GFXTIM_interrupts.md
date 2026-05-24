2687

Tamper and backup registers (TAMP)

RM0456

Table 643. TAMP internal input/output signals (continued)
Internal signal name

Signal type

Description
Confirmed tamper detection signal generated
only when ERCFGz = 1.
This signal is active when:
– a tamper event detection flag (internal or
external tamper), is generated in confirmed
mode.
– or a software request is done by writing
BKERASE to 1

tamp_confirmed_ercfgz
(z = signal index)

Output

tamp_it

Output

TAMP interrupt (refer to Section 64.6: TAMP
interrupts for details)

tamp_trg[x]
(x = signal index)

Output

Tamper detection trigger

tamp_bhk

Output

Tamper boot hardware key bus

1. Refer to Table 644: TAMP interconnection.

The TAMP kernel clock is usually the LSE at 32.768 kHz although it is possible to select
other clock sources in the RCC (refer to RCC for more details). The TAMP kernel clock is
required only for external tamper inputs level with filtering, and for external active tamper
detection modes. Internal tampers detection and external tampers inputs edge detection are
functional without requiring any kernel clock.
Read and write access to backup registers and all other TAMP registers are also functional
without any kernel clock (only APB clock is needed).
Some detections modes are not available in some low-power modes or VBAT depending on
the selected clock (refer to Section 64.5: TAMP low-power modes for more details.
Table 644. TAMP interconnection
Signal name

<!-- pagebreak -->

