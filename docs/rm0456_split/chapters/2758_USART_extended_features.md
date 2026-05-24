RM0456 Rev 6

RM0456

67.4.9

Low-power universal asynchronous receiver transmitter (LPUART)

Tolerance of the LPUART receiver to clock deviation
The asynchronous receiver of the LPUART works correctly only if the total clock system
deviation is less than the tolerance of the LPUART receiver. The causes that contribute to
the total deviation are:
•

DTRA: deviation due to the transmitter error (which also includes the deviation of the
transmitter’s local oscillator)

•

DQUANT: error due to the baud rate quantization of the receiver

•

DREC: deviation of the receiver local oscillator

•

DTCL: deviation due to the transmission line (generally due to the transceivers, which
can introduce an asymmetry between the low-to-high transition timing and the high-tolow transition timing)
DTRA + DQUANT + DREC + DTCL + DWU < LPUART receiver tolerance

where
DWU is the error due to sampling point deviation when the wake-up from lowpower mode is used.
when M[1:0] = 01:
t WULPUART
DWU = --------------------------11 × Tbit

when M[1:0] = 00:
t WULPUART
DWU = --------------------------10 × Tbit

when M[1:0] = 10:
t WULPUART
DWU = --------------------------9 × Tbit

tWULPUART is the time between the detection of the start bit falling edge and the
instant when the clock (requested by the peripheral) is ready and reaching the
peripheral, and the regulator is ready.
The LPUART receiver can receive data correctly at up to the maximum tolerated deviation
specified in Table 691:
•

Number of stop bits defined through STOP[1:0] bits in the LPUART_CR2 register

•

LPUART_BRR register value.
Table 691. Tolerance of the LPUART receiver
1024 < BRR < 2048 2048 < BRR < 4096

4096 ≤ BRR

M bits

768 < BRR < 1024

8 bits (M = 00), 1 stop bit

1.82%

2.56%

3.90%

4.42%

9 bits (M = 01), 1 stop bit

1.69%

2.33%

2.53%

4.14%

7 bits (M = 10), 1 stop bit

2.08%

2.86%

4.35%

4.42%

8 bits (M = 00), 2 stop bits

2.08%

2.86%

4.35%

4.42%

9 bits (M = 01), 2 stop bits

1.82%

2.56%

3.90%

4.42%

7 bits (M = 10), 2 stop bits

2.34%

3.23%

4.92%

4.42%

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Note:

The data specified in Table 691 may slightly differ in the special case when the received
frames contain some idle frames of exactly 10-bit times when M bits = 00 (11-bit times when
M = 01 or 9- bit times when M = 10).

67.4.10

LPUART multiprocessor communication
It is possible to perform LPUART multiprocessor communications (with several LPUARTs
connected in a network). For instance one of the LPUARTs can be the master, with its TX
output connected to the RX inputs of the other LPUARTs. The others are slaves, with their
respective TX outputs are logically ANDed together and connected to the RX input of the
master.
In multiprocessor configurations it is often desirable that only the intended message
recipient actively receives the full message contents, thus reducing redundant LPUART
service overhead for all nonaddressed receivers.
The nonaddressed devices can be placed in mute mode by means of the muting function.
To use the mute mode feature, the MME bit must be set in the LPUART_CR1 register.

Note:

When FIFO management is enabled and MME is already set, MME bit must not be cleared
and then set again quickly (within two lpuart_ker_ck cycles), otherwise mute mode might
remain active.
When the mute mode is enabled:
•

none of the reception status bits can be set;

•

all the receive interrupts are inhibited;

•

the RWU bit in the LPUART_ISR register is set to 1. RWU can be controlled
automatically by hardware or by software, through the MMRQ bit in the LPUART_RQR
register, under certain conditions.

The LPUART can enter or exit from mute mode using one of two methods, depending on
the WAKE bit in the LPUART_CR1 register:
•

Idle Line detection if the WAKE bit is reset,

•

Address mark detection if the WAKE bit is set.

Idle line detection (WAKE = 0)
The LPUART enters mute mode when the MMRQ bit is written to 1 and the RWU is
automatically set.
The LPUART wakes up when an Idle frame is detected. The RWU bit is then cleared by
hardware but the IDLE bit is not set in the LPUART_ISR register. An example of mute mode
behavior using Idle line detection is given in Figure 839.

<!-- pagebreak -->

