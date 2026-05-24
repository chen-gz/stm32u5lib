191

System security

RM0456

The DBGCMU is a TrustZone-aware peripheral, managing accesses to its control registers
as described in the table below.
Table 15. TrustZone-aware DBGMCU access management
Debug profile

Peripheral status(1)

DBG_xx_STOP control bits
Write access

Nonsecure invasive
(SPIDEN = 0)

NS

Yes (S(2) or NS)

S

None (S or NS)

Secure invasive
(SPIDEN = 1)

NS

Yes (S or NS)

S

Yes (S only)

Read access

Yes (S or NS)

1. As reported by the GTZC, the TrustZone-aware peripheral or the DMA channel.
2. Secure access from the debugger is converted to nonsecure access in the device.

Refer to Section 75.12: Microcontroller debug unit (DBGMCU) for more details.

3.5.6

Activating TrustZone security
The TrustZone is deactivated by default in all STM32U5 series devices. It can be activated
by setting the TZEN user option bit in FLASH_OPTR when in RDP level 0. Once TZEN has
changed from 0 to 1, the default security state, after reset, is always the following:
•

•

•

CPU subsystem
–

Cortex-M33 exits reset in secure state, hence the boot address must point toward
a secure memory area.

–

All interrupt sources are secure (in NVIC).

–

The memory mapped viewed by the CPU through IDAU/SAU is fully secure.

Embedded flash memory
–

Flash memory nonvolatile secure areas (with their HDP zone), are defined with
nonvolatile registers FLASH_SECWMxR (x = 1, 2). Default secure option bytes
setup is all user flash secure, without HDP area defined.

–

Volatile block-based security attributions of the flash memory are nonsecure. The
secure boot code can change this setup, making the blocks secure.

Embedded SRAM memories
–

•

External memories
–

<!-- pagebreak -->

