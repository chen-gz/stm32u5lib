•

the Armv8-M mainline security extension of Cortex-M33, enabling a new processor
secure state, with its associated secure interrupts

•

the dynamic allocation of memory and peripherals to TrustZone using eight security
attribution unit (SAU) regions of Cortex-M33

•

a global TrustZone framework (GTZC), extending the TrustZone protection against
transactions coming from other masters in the system than the Cortex-M33

•

TrustZone-aware embedded flash memory and peripherals

The TrustZone security is activated by the TZEN option bit in the FLASH_OPTR register.

RM0456 Rev 6

RM0456

3.5.2

System security

Armv8-M security extension of Cortex-M33
The Arm security extension of the Cortex-M33 is an evolution, not a revolution. It uses the
programmer model from earlier Cortex-M subfamilies like Cortex-M4. Indeed, Armv8-M is
architecturally similar to Armv7-M, using the same 32-bit architecture, the same memory
mapped resources protected with an MPU. Armv8-M also uses the nested vectored
interrupt controller (NVIC).
The Armv8-M TrustZone implementation in STM32U5 series devices is composed of the
following features:
•

a new processor state, with almost no additional code/cycle overhead, as opposed to
Armv8-A TrustZone that uses a dedicated exception routine for triggering a
secure/nonsecure world change

•

two memory map views of a shared 4-Gbyte address space

•

a low interrupt latency for both secure and nonsecure domains, and a new interrupt
configuration for security grouping and priority setting

•

separated exception vector tables for the secure and nonsecure exceptions

•

micro-coded context preservation

•

banking of specific registers across secure/nonsecure states, including stack pointers
with stack-limit checkers

•

banking of the following Cortex-M33 programmable components (two separate units for
secure and nonsecure):
–

SysTick timer

–

MPU configuration registers (eight MPU regions in secure, eight in nonsecure)

–

some of the system control block (SCB) registers

•

new system exception (SecureFault) for handling of security violations

•

configurable debug support, as defined in Section 3.11

For more information, refer to STM32 Cortex-M33 MCUs programming manual (PM0264).

3.5.3

Memory and peripheral allocation using IDAU/SAU
Security attributes
As illustrated on Figure 8, the Armv8-M nonsecure memory view is similar to Armv7-M
(that can be found in Cortex-M4), with the difference that the secure memory is hidden. The
secure memory view shows the flash memory, SRAM, and peripherals that are only
accessible while the Cortex processor executes in Secure state.

RM0456 Rev 6

<!-- pagebreak -->

191

System security

RM0456

The figure below shows the 32-bit address space viewed after the SAU configuration
by the secure code.
Figure 8. Sharing memory map between CPU in secure and nonsecure state

0xFFFF FFFF
0xF000 0000

Nonsecure Secure
memory view memory view

Nonsecure Secure
memory view memory view

System region

MPU-NS*
SCB-NS*
SysTick-NS*
DEBUG
hidden
SAU
MPU-NS
MPU-S
SCB-NS
SCB-S
NVIC
SysTick-NS
SysTick-S
ITM / DWT / FBP
hidden

System control and debug
0xE000 0000
External peripherals
0xA000 0000
External memories
0x6000 0000
0x4000 0000
0x2000 0000
0x0000 0000

Periph-NS
hidden
Periph-S
SRAM-NS
hidden
SRAM-S
Flash-NS
hidden
Flash-S

(*) Aliased addresses
MSv64440V1

The Cortex processor state (and associated rights) depends on the security attribute
assigned to the memory region where it is executed:
•

A processor in a nonsecure state only executes from nonsecure (NS) program
memory, while a processor in a secure state only executes from secure (S) program
memory.

•

While running in the secure state, the processor can access data from both S and NS
memories. Running in the nonsecure state, the CPU is limited to nonsecure memories.

In order to manage transitions to the secure world, developers must create nonsecure
callable (NSC) regions that contain valid entry points to the secure libraries. The first
instruction in these entry points must be the new secure gate (SG) instruction, used
by the nonsecure code to call a secure function (see the figure below).
Figure 9. Secure world transition and memory partitioning
Nonsecure memory

Secure memory

Nonsecure
callable
call

Nonsecure
application

return

Secure
entry point

call / branch

Secure
library
return

MSv64441V1

<!-- pagebreak -->

