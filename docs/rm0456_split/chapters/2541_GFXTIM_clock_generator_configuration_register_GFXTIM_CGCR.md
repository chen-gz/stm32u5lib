2687

Tamper and backup registers (TAMP)

RM0456

When TrustZone is disabled, the APB access to the TAMP registers are nonsecure.

64.4.3

GPIOs controlled by the RTC and TAMP
Refer to Section 63.3.3: GPIOs controlled by the RTC and TAMP.

64.4.4

TAMP register write protection
After system reset, the TAMP registers (including backup registers) are protected against
parasitic write access by the DBP bit in the power control peripheral (refer to the PWR
power control section). DBP bit must be set in order to enable TAMP registers write access.

64.4.5

TAMP secure protection modes
By default after a backup domain power-on reset, all TAMP registers can be read or written
in both secure and nonsecure modes, except for the TAMP secure configuration register
(TAMP_SECCFGR) which can be written in secure mode only. The TAMP protection
configuration is not affected by a system reset.
•

•

When the TAMPSEC bit is set in the TAMP_SECCFGR register:
–

Writing the TAMP registers is possible only in secure mode, except for the backup
registers which have their own protection setting.

–

Reading TAMP_SECCFGR, TAMP_PRIVCFGR and TAMP_MISR is always
possible in secure and nonsecure modes. All the other TAMP registers can be
read only in secure mode, except for the backup registers and monotonic counters
which have their own protection setting.

When the CNT1SEC bit is set in the TAMP_SECCFGR register: the TAMP_COUNT1R
can be read and written only in secure mode.

A nonsecure access to a secure-protected register is denied:
•

There is no bus error generated.

•

A notification is generated through a flag/interrupt in the TZIC (TrustZone illegal access
controller).

•

When write protected, the bits are not written.

•

When read protected they are read as 0.

As soon as at least one function is configured to be secured, the TAMP reset and clock
control is also secured in the RCC.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

64.4.6

Tamper and backup registers (TAMP)

Backup registers protection zones
The backup registers protection is configured thanks to BKPRWSEC[7:0] and
BKPWSEC[7:0] (refer to the figure below):
Figure 775. Backup registers protection zones
TAMP_BKPI(1)R
Protection Zone 3

Read nonsecure
Write nonsecure
TAMP_BKPtR (t = BKPWSEC)
TAMP_BKPzR (z = BKPWSEC-1)

Protection Zone 2

Read nonsecure
Write secure
TAMP_BKPyR (y = BKPRWSEC)
TAMP_BKPxR (x = BKPRWSEC-1)

Protection Zone 1

Read secure
Write secure
TAMP_BKP0R
MSv62397V2

1. l = last backup register index

In case TZEN =1, the bits BKPWPRIV and BKPRWPRIV in the TAMP_PRIVCFGR can be
written only in secure mode.

64.4.7

TAMP privilege protection modes
By default after a backup domain power-on reset, all TAMP registers can be read or written
in both privileged and unprivileged modes, except for the TAMP privilege configuration
register (TAMP_PRIVCFGR) which can be written in privilege mode only. The TAMP
protection configuration is not affected by a system reset.
When the TAMPPRIV bit is set in the TAMP_PRIVCFGR register:
•

Writing the TAMP registers is possible only in privilege mode, except for the backup
registers and the monotonic counter which have their own protection setting.

•

When the CNT1PRIV bit is set in the TAMP_PRIVCFGR register: the
TAMP_COUNT1R can be read and written only in privilege mode.

•

Reading TAMP_SECCFGR, TAMP_PRIVCFGR is always possible in privilege and
unprivilege modes. All the other TAMP registers can be read only in privileged mode,
except for the backup registers and the monotonic counter which have their own
protection setting.

The backup registers protection is configured thanks to BKPRWSEC[7:0] and BKPRWPRIV
for the protection zone 1, and thanks to BKPRWSEC[7:0], BKPWSEC[7:0] and BKPWPRIV
for the protection zone 2 (refer to Figure 775). The BHKLOCK bit can be written only in
privileged mode when the BKPRWPRIV bit is set.
A unprivileged access to a privileged-protected register is denied:
•

There is no bus error generated.

•

When write protected, the bits are not written.

•

When read protected they are read as 0.

RM0456 Rev 6

<!-- pagebreak -->

