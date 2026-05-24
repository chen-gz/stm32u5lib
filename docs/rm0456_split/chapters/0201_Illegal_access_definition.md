RM0456 Rev 6

ILA= illegal access (security only)
MSv63638V2

RM0456

5.4.2

Global TrustZone controller (GTZC)

Illegal access definition
Three different types of illegal access exist:
•

Illegal nonsecure access
Any nonsecure transaction trying to write a secure resource is considered as illegal and
thus the addressed resource generates an illegal access interrupt for illegal write
access and a bus error for illegal fetch access. However some exceptions exist on
secure and privileged configuration registers: these later ones authorize nonsecure
read access to secure registers (see GTZC_TZSC_SECCFGRx and
GTZC_TZSC_PRIVCFGRx).

•

Illegal secure access
Any secure transaction trying to access nonsecure block in internal block-based SRAM
or watermarked memory is considered as illegal.
Correct TZIC settings allows the capture of the associated event and then generates
the GTZC_IRQn interrupt to the NVIC. This applies for read, write and execute access.
Concerning the MPCBB controller, there is an option to ignore secure data read/write
access on nonsecure SRAM blocks, by setting the SRWILADIS bit in the
GTZC_MPCBBz_CR register. Secure read and write data transactions are then
allowed on nonsecure SRAM blocks, while secure execution access remains not
allowed.
Any secure execute transaction trying to access a nonsecure peripheral register is
considered as illegal and generate a bus error.

•

Illegal unprivileged access
Any unprivileged transaction trying to access a privileged resource is considered as
illegal. There is no illegal access event generated for illegal read and write access. The
addressed resource follows a silent-fail behavior, returning all zero data for read and
ignoring any write. No bus error is generated. A bus error is generated when any
unprivileged execute transaction tries to access a privileged memory.

5.4.3

TrustZone security controller (TZSC)
The TZSC is composed of a configurable set of registers, providing the following features:
•

Control of secure and privileged state for all peripherals, done through:
–

GTZC_TZSC_SECCFGRx registers to control AHB/APB firewall stubs for the

–

GTZC_TZSC_PRIVCFGRx registers to control AHB/APB firewall stubs for the

securable peripherals
privileged peripherals
•

For watermark memory protection controller (external memories and backup SRAM),
two independent regions can be defined and the following fields are used to program:
–

the start of the first protected sub-region on external memory/backup SRAM:
SUBA_START[10:0]

–

the length of the first protected sub-region on external memory/backup SRAM:
SUBA_LENGTH[11:0]

–

the start of the second protected sub-region on external memory/backup SRAM:
SUBB_START[10:0]

–

the length of the second protected sub-region on external memory/backup SRAM:
SUBB_LENGTH[11:0]

RM0456 Rev 6

<!-- pagebreak -->

