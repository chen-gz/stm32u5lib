•

Writing the RCC nonsecure bits is possible only with privileged access.

•

The RCC nonsecure bits can be read only with privileged access except
RCC_PRIVCFGR that can be read by privileged or unprivileged access.

•

An unprivileged access to a privileged RCC bit or register is discarded: the bits are
read as zero and the write to these bits is ignored (RAZ/WI).

RM0456 Rev 6

RM0456

11.6

Reset and clock control (RCC)

RCC low-power modes
•

AHB and APB peripheral clocks, including DMA clock, can be disabled by software.

•

Sleep mode stops the CPU clock. The memory interface clocks (flash memory, caches,
and all SRAM interfaces) can be stopped by software during Sleep mode. The AHB to
APB bridge clocks are disabled by hardware during Sleep mode when all the clocks of
the peripherals connected to them are disabled.

•

Stop modes (Stop 0, Stop 1, Stop 2, Stop 3) stop all the clocks in the core domain and
disable the PLLs, HSI16, HSI48, SHSI, MSI, and HSE oscillators. However, HSI16 or
MSI can be switched ON if the peripheral requests it for autonomous mode purpose, or
to generate a wake-up interrupt (see Section 11.4.24 for more details). LSI and LSE
remain active in Stop modes.

•

Standby and Shutdown modes stop all the clocks in the core domain and disable the
PLLs, HSI16, HSI48, SHSI, MSI, and HSE oscillators.

The CPU deep-sleep mode can be overridden for debugging by setting the DBG_STOP or
DBG_STANDBY bit in the DBGMCU_CR register.
When exiting Stop modes (Stop 0, Stop 1, Stop 2, or Stop 3), the system clock is either
MSIS or HSI16, depending on STOPWUCK in RCC_CFGR1. The frequency (range and
user trim) of MSIS and MSIK oscillators is the one configured before entering Stop mode,
except if above 24 MHz. In this case, the MSIS or MSIK range is the 24 MHz range.
The user trim of HSI16 is kept. If MSI is in PLL-mode before entering Stop mode with
MSIPLLFAST = 0, the PLL-mode stabilization time must be waited for after wake-up even
if the LSE was kept on during Stop mode. The PLL-mode accuracy is kept after wake-up
from Stop 0, Stop 1, or Stop 2 mode without stabilization time if MSIPLLFAST = 1.
MSIPLLFAST bit has no effect when exiting Stop 3 mode.
The other internal oscillator can be automatically woken up in addition to the one used by
the system clock, in order to avoid waiting for the other oscillator wake-up time when the
device is back in Run mode. This is done thanks to STOPKERWUCK in RCC_CFGR1.
When leaving the Standby and Shutdown modes, the system clock is MSIS. The MSIS and
MSIK frequency at wake-up from Standby mode is configured with MSISSRANGE and
MSIKSRANGE in RCC_CSR, from 1 to 4 MHz. The MSI frequency at wake-up from
Shutdown mode is 4 MHz. The user trim is lost.
If a flash memory programming operation is ongoing, a Stop, Standby, or Shutdown mode
entry is delayed until the flash memory interface access is finished. If an access to the APB
domain is ongoing, a Stop, Standby, or Shutdown mode entry is delayed until the APB
access is finished. If an autonomous peripheral generates a system clock request, a Stop,
Standby, or Shutdown mode entry is delayed until the system clock request is released.

RM0456 Rev 6

<!-- pagebreak -->

