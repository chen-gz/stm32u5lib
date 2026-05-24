RM0456 Rev 6

RM0456

44.8

DSI Host (DSI)

Functional description: timeout counters
The DSI Host includes counters to manage timeout during the various communication
phases. The duration of each timeout can be configured by the six timeout counter
configuration registers (DSI_TCCR0...5).
There are two types of counters:

44.8.1

•

contention error detection timeout counters (Section 44.8.1)

•

peripheral response timeout counters (Section 44.8.2).

Contention error detection timeout counters
The DSI Host implements a set of counters and conditions to notify the errors. It features a
set of registers to control the timers used to determine if a timeout has occurred, and also
contains a set of interruption status registers that are cleared upon a read operation
(detailed in Table 439). Optionally, these registers also trigger an interrupt signal that can be
used by the system to be activated when an error occurs within the DSI connection.
Table 439. Contention detection timeout counters configuration
Timeout counter

Value register

Value field

Flag register

Flag field

High-speed transmission

DSI_TCCR0

TOHSTX

DSI_ISR1

TOHSTX

Low-power reception

DSI_TCCR0

TOLPRX

DSI_ISR1

TOLPRX

Time units for these 16-bit counters are configured in cycles defined in the timeout clock
division (TOCKDIV) field in the DSI Host clock control register (DSI_CCR).
The value written to the timeout clock division (TOCKDIV) field in the DSI Host clock control
register (DSI_CCR) defines the time unit for the timeout limits using the lane byte clock as
input.
This mechanism increases the range to define these limits.

High-speed transmission contention detection
The timeout duration is configured in the high-speed transmission timeout count
(HSTX_TOCNT) field of the DSI Host timeout counter configuration register 1
(DSI_TCCR0). A 16-bit counter measures the time during which the high-speed mode is
active.
If that counter reaches the value defined by the high-speed transmission timeout count
(HSTX_TOCNT) field of the DSI Host timeout counter configuration register 1
(DSI_TCCR0), the timeout high-speed transmission (TOHSTX) bit in the DSI Host interrupt
and status register 1 (DSI_ISR1) is asserted and an internal soft reset is generated to the
DSI Host.
If the timeout high-speed transmission interrupt enable (TOHSTXIE) bit of the DSI Host
interrupt enable register 1 (DSI_IER1) is set, an interrupt is generated.

Low-power reception contention detection
The timeout is configured in the low-power reception timeout counter (LPRX_TOCNT) field
of the DSI Host timeout counter configuration register 1 (DSI_TCCR1). A 16-bit counter
measures the time during which the low-power reception is active. If that counter reaches

RM0456 Rev 6

<!-- pagebreak -->

