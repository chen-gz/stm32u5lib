Error management (privileged-only)
–

Any unprivileged transaction trying to access a privileged resource is considered
as illegal. There is no illegal access event generated for illegal unprivileged read
and write accesses.

–

The addressed resource follows a silent-fail behavior, returning all zero data for
read and ignoring any write.

–

When an illegal unprivileged access occurs, no bus error is generated, except
when this access is an instruction fetch, accessing a privileged memory or a
peripheral register.

RM0456 Rev 6

RM0456

System security

Managing security in privileged-aware peripherals
TrustZone-aware peripherals also implement privileged-only access mode. The privileged
protection is valid even if TZEN = 0:

•

Embedded flash memory
By default all embedded flash registers can be read or programmed in both privileged
and unprivileged modes.
When secure privileged bit SPRIV is set in FLASH_PRIVCFGR, reading and writing
the flash secure registers are possible only in privileged mode. Write access to this bit
is ignored if TrustZone is deactivated (TZEN = 0).
When nonsecure privileged bit NSPRIV is set in FLASH_PRIVCFGR, reading and
writing the flash nonsecure registers are possible only in privileged mode.
Regarding privileged protection of the embedded flash memory, the devices offer the
following features:

Note:

–

The system flash memory can be accessed both in privileged and unprivileged
modes.

–

Each watermark-based secure area, including its secure HDP area, is accessible
in secure-privileged and secure-unprivileged mode, if applicable.

–

Each 8-Kbyte page of the embedded flash memory can be programmed on-the-fly
as privileged only, using the block-based privileged configuration registers
FLASH_PRIV1BBRx and FLASH_PRIV2BBRx. An unprivileged page is
accessible by a privileged or unprivileged access.

Switching a page from privileged to unprivileged does not erase the content of the page.
When applicable, an erase or program operation is always available to privileged code, and
is available to unprivileged code only for unprivileged pages or unprivileged memory.

•

On-the-fly encryption/decryption (OTFDEC)
When privileged bit PRIV is set in OTFDEC_PRIVCFGR, the OTFDEC can only be
initialized by a privileged application.

Note:

OTFDEC_PRIVCFGR can be read by both privileged and unprivileged code.

•

Direct memory access controllers (LPDMA and GPDMA)
When a DMA channel x is defined as privileged (PRIVx = 1 in
LP/GPDMA_PRIVCFGR), special rules apply when accessing privileged/unprivileged
source or destination. Those rules are summarized on the table below.
Table 16. DMA channel use (privilege)
Privileged DMA channel x (PRIVx = 1)

Unprivileged DMA channel y (PRIVy = 0)

Destination
Privileged source
Privileged
Unprivileged

Unprivileged source

Privileged source

Unprivileged source

Transfer blocked(1)

OK

Transfer blocked

OK

1. When a transfer is blocked, the transfer completes but the corresponding writes are ignored, and reads return zeros.

See Section 18: Low-power direct memory access controller (LPDMA) and Section 17:
General purpose direct memory access controller (GPDMA) for more details.

RM0456 Rev 6

<!-- pagebreak -->

191

System security

•

RM0456
Power control (PWR)
By default, after a power-on or a system reset, all PWR registers but
PWR_PRIVCFGR, can be read or written in both privileged and unprivileged modes.
When secure privileged bit SPRIV is set in PWR_PRIVCFGR, reading and writing the
PWR securable registers are possible only in privileged mode. Write access to this bit
is ignored if TrustZone is disabled (TZEN = 0).
When nonsecure privileged bit NSPRIV is set in PWR_PRIVCFGR, reading and writing
the PWR nonsecure registers are possible only in privileged mode.
See Section 10: Power control (PWR) for details.

•

Secure clock and reset (RCC)
By default, after a power-on or a system reset, all RCC registers but RCC_PRIVCFGR
can be read or written in both privileged and unprivileged modes.
When the secure privileged bit SPRIV is set in RCC_PRIVCFGR, reading and writing
the RCC securable bits are possible only in privileged mode. Write access to this bit is
ignored if TrustZone is disabled (TZEN = 0).
When nonsecure privileged bit NSPRIV is set in RCC_PRIVCFGR, reading and writing
the RCC nonsecure bits are possible only in privileged mode.
See Section 11: Reset and clock control (RCC) for details.

•

Real time clock (RTC)
By default after a backup domain reset, all RTC registers but RTC_PRIVCFGR, can be
read or written in both privileged and unprivileged modes.
When PRIV bit is set in privileged-only RTC_PRIVCFGR:
–

Writing the RTC registers is possible only in privileged mode.

–

Reading the RTC_SECCFGR, RTC_PRIVCFGR, RTC_TR, RTC_DR, RTC_SSR,
RTC_PRER and RTC_CALR is always possible in privileged and unprivileged
modes.

All the other RTC registers can be read only in privileged mode.
When PRIV bit is cleared in privileged-only RTC_PRIVCFGR register, it is still possible
to restrict access to privileged mode to some RTC registers by setting dedicated
control bits: INITPRIV, CALPRIV, TSPRIV, WUTPRIV, ALRAPRV or ALRBPRIV.
See Section 63: Real-time clock (RTC) chapter for details.

•

Tamper and backup registers (TAMP)
By default after any backup domain reset, all TAMP registers but TAMP_PRIVCFGR
can be read or written in both privileged and unprivileged modes.
When PRIV bit is set in privileged-only TAMP_PRIVCFGR:
–

Writing the TAMP registers is possible only in privileged mode, except for the
backup registers and the monotonic counters that have their own protection
setting.

–

Reading the TAMP_SECCFGR or TAMP_PRIVCFGR is always possible in
privilege and unprivilege modes. All the other TAMP registers can be read only in
privilege mode, except for the backup registers and the monotonic counters that
have their own protection setting.

The application can also:
–

<!-- pagebreak -->

