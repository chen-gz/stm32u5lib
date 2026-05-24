RM0456 Rev 6

RM0456

System security

CloseExitHDP
Bootloader ID:
CloseExitHDP1 function
Secure attribute:
Secure callable function.
Prototype:
uint32_t CloseExitHDP(uint32_t HdpArea, uint32_tVectorTableAddr)
Arguments:
•

HdpArea:
Input parameter, bitfield that identifies which HDP area to close. Values can be either:
RSSLIB_HDP_AREA1_Msk, RSSLIB_HDP_AREA2_Msk or
RSSLIB_HDP_AREA1_Msk |RSSLIB_HDP_AREA2_Msk.

•

VectorTableAddr:
Input parameter,address of the next vector table to apply.
The vector table format is the one used by the Cortex®-M33 core.

Description:
The user calls CloseExitHDP to close flash HDP secure memory area and jump to the
reset handler embedded within the vector table which address is passed as input
parameter.
CloseExitHDP sets the SP provided by the passed vector table, however it is up to the
caller to first set the new vector table. Then it clears all general-purpose. A Cortex®M33 registers (r0, r1, …) before jumping to new vector table reset handler.
On successful execution, the function does not return and does not push LR onto the
stack.
In case of failure (bad input parameter value), this function returns RSSLIB_ERROR.
Please refer to section Chapter 7.5.3: Secure hide protection (HDP) to get more details
on flash memory HDP protection.

3.6.3

Resource isolation using Cortex privileged mode
In parallel to the TrustZone isolation described in Section 3.5, the hardware and software
resources of STM32U5 series devices can be partitioned so that they are restricted to
software running in Cortex privileged mode.
Thanks to this hardware isolation technology, available even if TrustZone is deactivated
(TZEN = 0), critical code or data can be protected against intentional or unintentional
tampering from the more exposed unprivileged code.

Memory and peripheral privileged allocation using MPU
The Cortex-M33 MPU divides the unified memory into eight regions, each aligned to a
multiple of 32 bytes. Each memory regions can be programmed to generate faults when
accessed inappropriately by unprivileged software.

Memory and peripheral privileged allocation using GTZC
For the Cortex-M33 master, to complement the coarse isolation provided by the MPU, the
GTZC reinforces, in a flexible way, the isolation between privileged and unprivileged tasks,
for peripherals and selected memories.
RM0456 Rev 6

<!-- pagebreak -->

191

System security

RM0456

For masters other than the Cortex-M33, the GTZC can assign them as unprivileged
initiators, automatically protecting resources defined as privileged against this master.
•

Securing peripherals with TZSC (privileged-only)
In the devices, a peripheral is either securable privileged-only through GTZC, or is
natively privileged-aware:
–

A securable privileged-only peripheral or memory is protected by an AHB/APB
firewall gate that is controlled by the TZSC.

–

A privileged-aware peripheral or memory is connected directly to the AHB or APB
interconnect, implementing a specific behavior such as a subset of registers or a
memory area is privilege-only.
When such peripheral is made privileged-only with GTZC, if it is master on the
interconnect (SDMMC), it automatically issues privileged transactions. Privilegeaware masters like GPDMA or LPDMA, drive privileged signal in the AHB
interconnect according to their internal privileged mode, independently to the
GTZC.
The list of securable peripherals can be found in Section 5: Global TrustZone
controller (GTZC).

•

Securing memories with TZSC and MPCBB (privileged-only)
The TZSC logic in GTZC provides the capability to manage the privilege level for all
securable external memories, programming the MPCWM resources defined in
Section 3.5.4.
Similarly, the GTZC provides the capability to configure the privilege level of embedded
SRAM blocks, programming the MPCBB resources defined in Section 3.5.4.

•

<!-- pagebreak -->

