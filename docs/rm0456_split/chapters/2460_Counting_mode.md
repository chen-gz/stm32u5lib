IWDG

RM0456 Rev 6

RM0456

Independent watchdog (IWDG)

3. Wake-up from Stop with interrupt is supported only in Stop 0, Stop 1, and Stop 2 modes for
STM32U575/585, and in Stop 0, Stop 1, Stop 2, and Stop 3 modes for STM32U535/545/59x/5Ax/5Fx/5Gx.
4. Controlled via DBG_IWDG_STOP in DBG section.
5. Controlled via the option byte IWDG_STOP in FLASH section.
6. Controlled via the option byte IWDG_STDBY in FLASH section.
7. Controlled via the option byte IWDG_SW in FLASH section.

61.4

IWDG functional description

61.4.1

IWDG block diagram
Figure 766 shows the functional blocks of the independent watchdog module.
Figure 766. Independent watchdog block diagram

IWDG
APB

VCORE voltage domain
Register interface

IRQ
interface

iwdg_it

iwdg_pclk
sync

iwdg_ker_req

Shadow registers and Control

12-bit reload value
iwdg_ker_ck

Prescaler
presc_ck

IWDCNT
(12 bits)

iwdg_in_rst

Comparator logic

iwdg_wkup
iwdg_out_rst

VDD voltage domain
MS49973V4

The register and IRQ interfaces are located into the VCORE voltage domain. The watchdog
function itself is located into the VDD voltage domain to remain functional in low power
modes. See Section 61.3 for IWDG capabilities.
The register and IRQ interfaces are mainly clocked by the APB clock (iwdg_pclk), while the
watchdog function is clocked by a dedicated kernel clock (iwdg_ker_ck). A synchronization
mechanism makes the data exchange between the two domains possible. Note that most of
the registers located in the register interface are shadowed into the VDD voltage domain.
The IWDG down-counter (IWDCNT) is clocked by the prescaled clock (presc_ck). The
prescaled clock is generated from the kernel clock iwdg_ker_ck divided by the prescaler,
according to PR[3:0] bitfield.
The table below gives the timing delays according to the actions performed on the IWDG:
changing the prescaler, the timeout, the window or the early wake-up comparator values or
doing a refresh.

RM0456 Rev 6

<!-- pagebreak -->

2577

Independent watchdog (IWDG)

RM0456

Table 623. IWDG delays versus actions (1)
TDRVU, TDPVU

TDWVU

TDEWU

TDRefresh

TDRefAuto

Min

Max

Min

Max

Min

Max

Min

Max

Min

Max

5 Tk

6 Tk

-

6 Tk

-

6 Tk

2 Tk

2 Tk + Tp

-

Tp

1. Tk represents a period of the kernel clock input, Tp represents a period of presc_ck.

61.4.2

IWDG internal signals
The list of IWDG internal signals is detailed in Table 624.
Table 624. IWDG internal input/output signals

61.4.3

Signal name

Signal type

Description

iwdg_ker_ck

Input

IWDG kernel clock

iwdg_ker_req

Input

IWDG kernel clock request

iwdg_pclk

Input

IWDG APB clock

iwdg_out_rst

Output

IWDG reset output

iwdg_in_rst

Input

IWDG reset input

iwdg_wkup

Output

IWDG wake-up event

iwdg_it

Output

IWDG early wake-up interrupt

Software and hardware watchdog modes
The watchdog modes allow the application to select the way the IWDG is enabled, either by
software commands (Software watchdog mode), or automatically (Hardware watchdog
mode). All other functions work similarly for both Software and Hardware modes.
The Software watchdog mode is the default working mode. The independent watchdog is
started by writing the value 0x0000 CCCC into the IWDG key register (IWDG_KR), and the
IWDCNT starts counting down from the reset value (0xFFF).
In the hardware watchdog mode the independent watchdog is started automatically at
power-on, or every time it is reset (via iwdg_in_rst). The IWDCNT down-counter starts
counting down from the reset value 0xFFF. The hardware watchdog mode feature is
enabled through the device option bits, see Section 61.3 for details.
When the IWDCNT reaches 0x000, a reset signal is generated (iwdg_out_rst asserted).
Whenever the key value 0x0000 AAAA is written in the IWDG key register (IWDG_KR), the
IWDG_RLR value is reloaded into the IWDCNT, and the watchdog reset is prevented.
Due to re-synchronization delays, the IWDG must be refreshed before the IWDCNT
down-counter reaches 1.
Once started, the IWDG can be stopped only when it is reset (iwdg_in_rst asserted).
As shown in Figure 767, when the refresh command is executed, one period of presc_ck
later, the IWDCNT is reloaded with the content of RL[11:0].

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Independent watchdog (IWDG)
Figure 767. Reset timing due to timeout

IWDCNT value

Refresh not allowed(1)

Refresh allowed

0xFFF
RL[11:0]
WIN[11:0]+1
WIN[11:0]

Tpresc_ck

Time
2
1
0

Tpresc_ck

Tpresc_ck

Writing 0xAAAA
into IWDG_KR
iwdg_out_rst
iwdg_in_rst
MS49974V2

1. If window option activated.

If the IWDG is not refreshed before the IWDCNT reaches 1, the IWDG generates a reset
(iwdg_out_rst is asserted). In return, the RCC resets the IWDG (assertion of iwdg_in_rst) to
clear the reset source.

61.4.4

Window option
The IWDG can also work as a window watchdog, by setting the appropriate window in the
IWDG window register (IWDG_WINR).
If the reload operation is performed while the counter is greater than WIN[11:0] + 1, a reset
is generated. WIN[11:0] is located in the IWDG window register (IWDG_WINR). As shown in
Figure 768, the reset is generated one period of presc_ck after the unexpected refresh
command.
The default value of the IWDG window register (IWDG_WINR) is 0x0000 0FFF, so, if not
updated, the window option is disabled.
As soon as the window value changes, the down-counter (IWDCNT) is reloaded with the
RL[11:0] value, to ease the estimation for where the next refresh must take place.

RM0456 Rev 6

<!-- pagebreak -->

2577

Independent watchdog (IWDG)

RM0456

Figure 768. Reset timing due to refresh in the not allowed area

IWDCNT value
Refresh not allowed

0xFFF
RL[11:0]
WIN[11:0] + 1

Time
Tpresc_ck

Tpresc_ck

Writing 0xAAAA
into IWDG_KR
iwdg_out_rst
iwdg_in_rst
MS49975V3

Configuring the IWDG when the window option is enabled

Note:

<!-- pagebreak -->

1.

Enable the IWDG by writing 0x0000 CCCC in the IWDG key register (IWDG_KR).

2.

Enable register access by writing 0x0000 5555 in the IWDG key register (IWDG_KR).

3.

Write the IWDG prescaler by programming IWDG prescaler register (IWDG_PR).

4.

Write the IWDG reload register (IWDG_RLR).

5.

If needed, enable the early wake-up interrupt, and program the early wake-up
comparator, by writing the proper values into the IWDG early wake-up interrupt register
(IWDG_EWCR).

6.

Write to the IWDG window register (IWDG_WINR). This automatically reloads the
IWDCNT down-counter with the RL[11:0] value.

7.

Wait for the registers to be updated (IWDG_SR = 0x0000 0000).

8.

Write 0x0000 0000 into IWDG key register (IWDG_KR) to write-protect registers.

Step 7 can be skipped if the application does not intend to disable the APB clock after the
completion of this sequence.

RM0456 Rev 6

RM0456

Independent watchdog (IWDG)

Configuring the IWDG when the window option is disabled
When the window option it is not used, the IWDG can be configured as follows:
1.

Enable the IWDG by writing 0x0000 CCCC in the IWDG key register (IWDG_KR).

2.

Enable register access by writing 0x0000 5555 in the IWDG key register (IWDG_KR).

3.

Write the prescaler by programming the IWDG prescaler register (IWDG_PR).

4.

Write the IWDG reload register (IWDG_RLR).

5.

If needed, enable the early wake-up interrupt, and program the early wake-up
comparator, by writing the proper values into the IWDG early wake-up interrupt register
(IWDG_EWCR).

6.

Wait for the registers to be updated (IWDG_SR = 0x0000 0000).

7.

Refresh the counter with RL[11:0] value, and write-protect registers by writing
0x0000 AAAA into IWDG key register (IWDG_KR).

The figure below shows a sequence example changing the prescaler, the reload value, and
then performing a refresh.
Figure 769. Changing PR, RL, and performing a refresh(1)
iwdg_ker_ck
TDPVU
WDGCNT
counter

TDRVU

N

...

N-k

N-k-1

...

÷4

÷4

÷4

÷8

...

N-k-t

TDRefresh
N-k-t-1

...

...

M

...

Refresh ignored region

PVU
RVU
Prescaler

÷4

RL[11:0]

÷8
M

P

CPU activity
Write 1 into IWDG_PR

Write into M into RL[11:0]

Write 0xAAAA into IWDG_KR
Refresh + Registers lock

Write 0x5555 into IWDG_KR
Registers unlock

MSv75426V1

1. Refer to Table 623: IWDG delays versus actions for details on timing values.

Note:

When the new prescaler value is accepted by the IWDG (falling edge of PVU), the new
division ratio is effective when the current division sequence is completed (N-k in the
drawing).

Note:

The timeout delay between the refresh (write 0xAAAA into IWDG_KR) and the watchdog
reset is increased by TDRefresh.
If a refresh command is sent while the previous refresh command is not yet completed, this
last refresh command is ignored.

RM0456 Rev 6

<!-- pagebreak -->

2577

Independent watchdog (IWDG)

RM0456

Updating the window comparator
It is possible to update the window comparator when the IWDG is already running. The
IWDCNT is reloaded as well. The following sequence can be performed to update the
window comparator:
1.

Enable register access by writing 0x0000 5555 in the IWDG key register (IWDG_KR).

2.

Write to the IWDG window register (IWDG_WINR). This automatically reloads the
IWDCNT down-counter with RL[11:0] value.

3.

Wait for WVU = 0

4.

Lock registers by writing IWDG_KR to 0x0000 0000

Step 3 can be skipped if the application does not intend to disable the APB clock after the
completion of this sequence.

61.4.5

Debug
When the processor enters into Debug mode (core halted), the IWDCNT down-counter
either continues to work normally or stops, depending on debug capability of the product.
Refer to Section 61.3 for details on the capabilities of this product.

61.4.6

Register access protection
Write accesses to IWDG prescaler register (IWDG_PR), IWDG reload register
(IWDG_RLR), IWDG early wake-up interrupt register (IWDG_EWCR) and IWDG window
register (IWDG_WINR) are protected. To modify them, first write 0x0000 5555 in the IWDG
key register (IWDG_KR). A write access to this register with a different value breaks the
sequence and register access is protected again. This is the case of the reload operation
(writing 0x0000 AAAA).
A status register is available to indicate that an update of the prescaler or the down-counter
reload value or the window value is ongoing.

61.5

IWDG low power modes
Depending on option bytes configuration, the IWDG can continue counting or not during the
low power modes. Refer to Section 61.3 for details.
Table 625. Effect of low power modes on IWDG
Mode

<!-- pagebreak -->

Description

Sleep

No effect. IWDG interrupts cause the device to exit from the mode.

Stop

The IWDG remains active or not, depending on option bytes configuration.
Refer to Section 61.3 for details.
IWDG interrupts cause the device exit the Stop mode.

Standby

The IWDG remains active or not, depending on option bytes configuration.
Refer to Section 61.3 for details.
IWDG interrupts do not make the device to exit from Standby mode.

Shutdown

The IWDG is not working.

RM0456 Rev 6

RM0456

61.6

Independent watchdog (IWDG)

IWDG interrupts
The IWDG offers the possibility to generate an early interrupt depending on the value of the
down-counter. The early interrupt is enabled by setting the EWIE bit of the IWDG early
wake-up interrupt register (IWDG_EWCR) to 1.
A comparator value (EWIT[11:0]) allows the application to define the position where the
early interrupt must be generated.
When the IWDCNT down-counter reaches the value of EWIT[11:0] - 1, the iwdg_wkup is
activated, making it possible for the system to exit from low power modes, if needed.
When the APB clock is available, the iwdg_it is activated as well.
In addition, the flag EWIF of the IWDG status register (IWDG_SR) is set to 1.
The EWI interrupt is acknowledged by writing 1 to the EWIC bit in the IWDG early wake-up
interrupt register (IWDG_EWCR).
Writing into the IWDG_EWCR register also triggers a refresh of the down-counter
(IWDCNT) with the reload value RL[11:0].
Figure 770. Independent watchdog interrupt timing diagram

IWDCNT value
IWDCNT = EWIT - 1
EWIT[11:0]

Writing
EWIC to ‘1’

Time

APB write accesses
iwdg_wkup_it
pclk active

pclk
iwdg_it

MS49976V1

The early wake-up interrupt (EWI) can be used if specific safety operations or data logging
must be performed before the watchdog reset is generated.

Changing the early wake-up comparator value
It is possible to change the early wake-up comparator value or to enable/disable the
interrupt generation at any time, by performing the following sequence:
1.

Enable register access by writing 0x0000 5555 in the IWDG key register (IWDG_KR).

RM0456 Rev 6

<!-- pagebreak -->

