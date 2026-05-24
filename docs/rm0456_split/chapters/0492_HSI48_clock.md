609

Reset and clock control (RCC)

RM0456

Software calibration
The MSIRCx (x = 0 to 3) oscillators frequency may vary from one chip to another due to
manufacturing process variations, this is why each device is factory calibrated by ST for 1 %
accuracy at an ambient temperature, TA = 25 °C. After reset, the factory calibration value is
loaded in MSICALx[4:0] (x = 0 to 3) in RCC_ICSCR1. If the application is subject to voltage
or temperature variations, this may affect the RC oscillator speed. The MSIRCx frequency
can be trimmed in the application by using MSITRIMx[4:0] (x = 0 to 3) in RCC_ICSCR.
Note:

The final accuracy after applying the calibration value is reached after a stabilization time.
This stabilization time is needed after reset of exiting Standby or Shutdown mode. It is also
needed when switching from PLL-mode to normal mode.
The hardware auto calibration with LSE must not be used in conjunction with software
calibration.
For more details on how to measure the MSI frequency variation, refer to Section 11.4.23.

11.4.4

HSI48 clock
The HSI48 clock signal is generated from an internal 48 MHz RC oscillator and can be used
directly for USB/OTG_FS, and for the RNG, as well as the SDMMC.
The internal 48 MHz RC oscillator is mainly dedicated to provide a high-precision clock
to the OTG_FS and the USB by means of a special clock recovery system (CRS) circuitry.
The CRS can use the USB SOF signal (only on STM32U535/545/575/585), the LSE, or an
external signal to automatically and quickly adjust the oscillator frequency on-the-fly. It is
disabled as soon as the system enters Stop or Standby mode. When the CRS is not used,
the HSI48 RC oscillator runs on its default frequency that is subject to manufacturing
process variations.
For more details on how to configure and use the CRS peripheral, refer to Section 12: Clock
recovery system (CRS).
The HSI48RDY flag in the RCC_CR register indicates whether the HSI48 RC oscillator is
stable or not. At startup, the HSI48 RC oscillator output clock is not released until this bit is
set by hardware.
The HSI48 can be switched on and off using the HSI48ON bit in the RCC_CR register.

11.4.5

SHSI clock
The SHSI is an internal securable RC oscillator dedicated to clock the SAES. SHSIRDY flag
in RCC_CR indicates if the SHSI RC is stable or not. At startup, the SHSI RC output clock is
not released until this bit is set by hardware.
The SHSI RC can be switched on and off using SHSION in RCC_CR.

<!-- pagebreak -->

