make TAMP_COUNT1R register read and write privileged-only by setting the
CNTPRIV bit in TAMP_PRIVCFGR

RM0456 Rev 6

RM0456

System security
–

increase security for two of the three protection zones in backup registers, using
BKPRWPRIV and BKPWPRIV bits in TAMP_PRIVCFGR:
– Make protection zone 1 read privileged, write privileged.
– Make protection zone 2 read privileged or unprivileged, write privileged.
– Protection zone 3 is always read and write privileged or unprivileged.

•

General-purpose I/Os (GPIO)
All GPIO registers can be read and written by privileged and unprivileged accesses,
whatever the security state (secure or nonsecure).

•

Extended interrupts and event controller (EXTI)
The EXTI peripheral is able to protect event register bits from being modified by
unprivileged accesses. The protection is individually activated per input event via the
register bits in the privileged-only EXTI_PRIVCFGR1 register. When an input event is
configured as privileged, only a privileged application can change the configuration
(including security if applicable), change the masking or clear the status of this input
event.
The security configuration in EXTI_PRIVCFGR1 can be globally locked after reset in
EXTI_LOCKR.
See Section 23: Extended interrupts and event controller (EXTI) for more details.

•

System configuration controller (SYSCFG)
All SYSCFG registers can be read and written in both privileged and unprivileged
modes, except:
–

FPUSEC bit in SYSCFG_SECCFGR registers (privileged only)

–

SYSCFG registers for CPU configuration: SYSCFG_CSLCKR,
SYSCFG_FPUIMR and SYSCFG_CNSLCKR

See Section 15: System configuration controller (SYSCFG) for more details.

3.7

Secure execution
Through a mix of special software and hardware features, the devices ensure the correct
operation of their functions against abnormal situations caused by programmer errors,
software attacks through network access or local attempt for tampering code execution.
This section describes the hardware features specifically designed for secure execution.

3.7.1

Memory protection unit (MPU)
The Cortex-M33 includes a memory protection unit (MPU) that can restrict the read and
write accesses to memory regions (including regions mapped to peripherals), based on one
or more of the following parameters:
•

Cortex-M33 operating mode (privileged, unprivileged)

•

data/instruction fetch

The memory map and the programming of the nonsecure and secure MPUs split memory
into regions (up to eight per MPU). Secure MPU is only available when TrustZone
is activated.

RM0456 Rev 6

<!-- pagebreak -->

