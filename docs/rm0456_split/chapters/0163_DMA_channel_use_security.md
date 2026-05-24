RM0456 Rev 6

RM0456

System security
The table below summarizes these security options available in each DMA channel.
Table 11. DMA channel use (security)(1)
Secure DMA channel x (SECx = 1)

Nonsecure DMA channel y (SECy = 0)

Destination type
Secure source

Nonsecure source

Secure destination

OK

OK(2)

Nonsecure destination

OK(3)

(4)

OK

Secure source

Nonsecure source

Transfer blocked
Transfer blocked

OK

1. When a transfer is blocked, the transfer completes but the corresponding writes are ignored, and reads return zeros. Also
an illegal access event to TZIC is automatically triggered by the memory/peripheral used as source or destination.
2. If the source is a memory, the transfer is only possible if SSEC = 0, otherwise the transfer is blocked.
3. If the destination is a memory, the transfer is only possible if DSEC = 0, otherwise the transfer is blocked.
4. If the transfer is memory-to-memory, the transfer is only possible if SSEC = 0 and DSEC = 0, otherwise the transfer
is blocked.

When a channel is configured as secure:
•

Registers allocated to this channel (excluding LP/GPDMA_SECCFGR,
LP/GPDMA_PRIVCFGR, and LP/GPDMA_RCFGLOCKR) are read as zero. Write are
ignored for nonsecure accesses. A secure illegal access event may also be triggered
toward the TZIC peripheral.
–

Note:

Writes to LP/GPDMA_SECCFGR and LP/GPDMA_RCFGLOCKR must be
secure. For each bit in LP/GPDMA_PRIVCFGR, write must be secure if the
corresponding bit in LP/GPDMA_SECCFGR is set.

•

In linked-list mode, the loading of the next linked-list data structure from memory is
performed with secure transfers.

•

When switching to a nonsecure state, the secure application must abort the channel or
wait until the secure channel is completed before doing the switch.

DMA secure channels are not available when TrustZone is deactivated.
When a channel is configured as nonsecure, in linked-list mode, the loading of the next
linked-list data structure from memory is performed with nonsecure transfers.
See Section 18: Low-power direct memory access controller (LPDMA) and Section 17:
General purpose direct memory access controller (GPDMA) for more details.

Power control (PWR)
When the TrustZone security is activated (TZEN = 1), the selected PWR registers can be
secured through PWR_SECCFGR, protecting the following PWR features:
•

low-power mode setup

•

wake-up (WKUP) pins

•

voltage detection and monitoring

•

backup domain control

Other PWR configuration bits become secure:
•

when the system clock selection is secure in the RCC: the voltage scaling (VOS) and
the regulator booster (BOOSTEN) configurations become secure.

•

when a GPIO is configured as secure: its corresponding bit for pull-up/pull-down
configuration in Standby mode becomes secure.

RM0456 Rev 6

<!-- pagebreak -->

191

System security
•

RM0456
when the USB Type-C/USB power delivery interface (UCPD) is configured as secure in
TZSC: PWR_UCPDR register becomes secure.

See Section 10: Power control (PWR) for details.

Secure clock and reset (RCC)
When the TrustZone security is activated (TZEN = 1) and security is enabled in the RCC,
the bits controlling the peripheral clocks and resets become TrustZone-aware:

Note:

•

If the peripheral is securable and programmed as secure in the TZSC, the peripheral
clock and reset bits become secure.

•

If the peripheral is TrustZone-aware, the peripheral clock and reset bits become secure
as soon as at least one function is configured as secure inside the peripheral.

Refer to Section 3.5.4 for the list of securable and TrustZone-aware peripherals.
The SHSI configuration and status bits are secured when the SAES is configured as secure.
Additionally, the following configurations can be made secure-only using RCC_SECCFGR:
•

external clock (such as HSE or LSE), internal oscillator (such as HSI, MSI, or LSI)

•

main PLL and AHB prescaler

•

system clock source selection

•

reset flag clearing

•

automatic internal oscillator waking up configuration

See Section 11: Reset and clock control (RCC) for details.

Real time clock (RTC)
Like all TrustZone-aware peripherals, a nonsecure read/write access to a secured RTC
register is RAZ/WI. It also generates an illegal access event that triggers a secure illegal
access interrupt if the RTC illegal access event is enabled in the TZIC.
After a backup domain power-on reset, all RTC registers can be read or written in both
secure and nonsecure modes. The secure boot code can then change this security setup,
making registers alarm A, alarm B, wake-up timer, and timestamp secure or not, using
RTC_SECCFGR.
When the SEC bit is set in secure-only RTC_SECCFGR:
•

Writing the RTC registers is possible only in secure mode.

•

Reading RTC_SECCFGR, RTC_PRIVCFGR, RTC_MISR, RTC_TR, RTC_DR,
RTC_SSR, RTC_PRER, and RTC_CALR is always possible in secure and nonsecure
modes. All the other RTC registers can be read only in secure mode.

When the SEC is cleared in secure-only RTC_SECCFGR, it is still possible to restrict
access in secure mode to some RTC registers by setting dedicated control bits: INITSEC,
CALSEC, TSSEC, WUTSEC, ALRASEC, and ALRBSEC.
Note:

The RTC security configuration is not affected by a system reset.
See Section 63: Real-time clock (RTC) for more details.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

System security

Tamper and backup registers (TAMP)
Like all TrustZone-aware peripherals, a nonsecure read/write access to a secured TAMP
register is RAZ/WI. It also generates an illegal access event that triggers a secure illegal
access interrupt if the TAMP illegal access event is enabled in the TZIC.
After a backup domain power-on reset, all TAMP registers can be read or written in both
secure and nonsecure modes. The secure boot code can change this security setup,
making some registers secure or not as needed, using the TAMP_SECCFGR register.
When TAMPSEC is set in TAMP_SECCFGR:
•

Writing the TAMP registers is possible only in secure mode. Backup registers have
their own write protection (see below).

•

Reading the TAMP registers (except for TAMP_SECCFGR, TAMP_PRIVCFGR and
TAMP_MISR) returns zero if the access is nonsecure. Backup registers have their own
read protection (see below).

The application can also:

Note:

•

make TAMP_COUNTR register read and write secure-only by setting the CNT1SEC bit
in TAMP_SECCFGR secure register

•

in backup registers increase security for two of the three protection zones configured
using BKPRWSEC[7:0] and BKPWSEC[7:0] bitfields in TAMP_SECCFGR:
–

protection zone 1 is read nonsecure, write nonsecure

–

protection zone 2 is read nonsecure, write secure

–

protection zone 3 is read secure, write secure

The TAMP security configuration is not affected by a system reset.
See Section 64: Tamper and backup registers (TAMP) for more details.

General-purpose I/Os (GPIO)
When the TrustZone security is activated (TZEN = 1), each I/O pin of the GPIO port can be
individually configured as secure through the GPIOx_SECCFGR registers. Only a secure
application can write to GPIOx_SECCFGR registers. After boot, each I/O pin of GPIO is set
as secure.
When an I/O pin is configured as secure, its corresponding configuration bits for alternate
function (AF), mode selection (MODE), and I/O data are RAZ/WI in case of nonsecure
access.
When the digital alternate function is used (input/output mode), in order to protect the data
transiting from/to the I/O managed by a secure peripheral, the devices add a secure
alternate function gate on the path between the peripheral and its allocated I/Os:
•

If the peripheral is secure, the I/O pin must also be secure to allow input/output of data.

•

If the peripheral is not secure, the connection is allowed regardless of the I/O pin state.

RM0456 Rev 6

<!-- pagebreak -->

