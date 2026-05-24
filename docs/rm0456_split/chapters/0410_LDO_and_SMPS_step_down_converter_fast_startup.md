482

Power control (PWR)

RM0456
3. Wait for 8 µs.
4. Increase the system frequency to the required frequency.

10.5.3

LDO and SMPS step down converter fast startup
After BOR reset, the LDO and SMPS regulators starts in slow-startup mode. This -startup
feature is selected to limit the inrush current after power-on reset. This increases
the wake-up time when exiting Stop or Standby mode.
However, it is possible to configure a faster startup on-the-fly, and it is applied for next
startup either after a system reset or after a wake-up from low-power mode except
Shutdown and VBAT modes. The fast startup is selected by setting FSTEN in PWR_CR3.

10.5.4

Dynamic voltage scaling management
The dynamic voltage scaling is a power management technique that consists in increasing
or decreasing the voltage used for the digital peripherals (VCORE), according to
the application performance and power consumption needs.
Dynamic voltage scaling to increase VCORE is known as overvolting. It allows the device
to improve its performance.
Dynamic voltage scaling to decrease VCORE is known as undervolting. It is performed to
save power, particularly in laptop and other mobile devices where the energy comes from a
battery and is thus limited.
The regulator operates in the following ranges:
• Range 1: high performance
It provides a typical output voltage at 1.2 V. It is used when the system clock frequency
is up to 160 MHz.
• Range 2: medium-high performance
It provides a typical output voltage at 1.1 V. It is used when the system clock frequency
is up to 110 MHz.
• Range 3: medium-low power range
It provides a typical output voltage at 1.0 V. The system clock frequency can be up to
55 MHz.
• Range 4: low-power range
It provides a typical output voltage at 0.9 V. The system clock frequency can be
up to 25 MHz.
Voltage scaling is selected through VOS[1:0] in PWR_VOSR. The EPOD (embedded power
distribution) booster must be enabled and ready before increasing the system clock
frequency above 55 MHz in range 1 and range 2.
The sequence to switch the voltage scaling from range L (lower power) to range P (higher
performance) with L > P, is the following:
1.

<!-- pagebreak -->

If target SYSCLK > 55 MHz:
a)

Configure PLL1MBOOST[3:0] in RCC_PLL1CFGR to generate a booster clock
frequency between 4 and 16 MHz.

b)

Switch on the PLL1 oscillator clock source.

RM0456 Rev 6

RM0456

Power control (PWR)
c)

Select the PLL1 clock source (PLL1SRC[1:0] in RCC_PLL1CFGR).

2.

Program VOS[1:0] to range P in PWR_VOSR.

3.

Wait until the VOSRDY flag is set in PWR_VOSR.

4.

If target SYSCLK > 55 MHz:
a)

Set BOOSTEN in PWR_VOSR. This step can be done together with
VOS programming.

b)

Wait until the BOOSTRDY flag is set in PWR_VOSR.

5.

Adjust number of wait states according new frequency target in range P (LATENCY bits
in FLASH_ACR, and WSC bits in RAMCFG_MxCR).

6.

Configure and enable the PLL if needed.

7.

Configure and switch to new system frequency.

The sequence to switch the voltage scaling from range P (higher performance) to range L
(lower power) with L > P, is the following:
1.

Reduce the system frequency to a value lower than range L maximum frequency.

2.

Adjust number of wait states according new frequency target (LATENCY bits
in FLASH_ACR and WSC bits in the RAMCFG_MxCR).

3.

If new SYSCLK ≤ 55 MHz, clear BOOSTEN in PWR_VOSR if it was set.

4.

Program VOS bits to range L in PWR_VOSR. This step can be done together with
BOOSTEN clearing.

System frequency steps on STM32U59x/5Ax/5Fx/5Gx devices
On STM32U59x/5Ax/5Fx/5Gx devices only, the maximum system frequency increase or
decrease in the VOS range 1 is 80 MHz.
The sequence to increase the frequency in the VOS range 1 above 80 MHz is the following:
1.

Divide the system clock by two, using the AHB prescaler (HPRE = 0b1000
in RCC_CFGR2).

2.

Configure and enable the PLL1 if needed.

3.

Select PLL1 as system clock source (SW = 0b11 in RCC_CFGR1).

4.

Wait for 5 µs.

5.

Set the AHB prescaler to 1 (HPRE = 0b0000 in RCC_CFGR2).

When running at higher frequencies than 80 MHz in the VOS range 1, the sequence
to decrease the frequency is the following:
1.

Divide the system clock by two using the AHB prescaler (HPRE = 0b1000
in RCC_CFGR2).

2.

Wait for 5 µs.

3.

Define the lower speed clock as system clock source.

4.

Set the AHB prescaler back to 1 (HPRE = 0b0000 in RCC_CFGR2).

In other VOS ranges, there is no limitation during system frequency increase or decrease.

RM0456 Rev 6

<!-- pagebreak -->

