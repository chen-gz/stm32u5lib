RM0456 Rev 6

RM0456

Reset and clock control (RCC)

External source (HSE bypass)
In this mode, an external clock source must be provided. It can have a frequency of up to
50 MHz. This mode is selected by setting HSEBYP and HSEON in RCC_CR. The external
clock signal (square, sinus, or triangle) with ~40-60 % duty cycle depending on
the frequency (refer to the datasheet) must drive the OSC_IN pin while the OSC_OUT pin
can be used a GPIO (see Figure 39).

11.4.2

HSI16 clock
The HSI16 clock signal is generated from an internal 16 MHz RC oscillator.
The HSI16 RC oscillator has the advantage of providing a clock source at low cost (no
external components). It also has a faster startup time than the HSE crystal oscillator.
However, even with calibration, the frequency is less accurate than an external crystal
oscillator or ceramic resonator.
The HSI16 clock can be selected as system clock after wake-up from Stop modes.
Refer to Section 11.8.6. It can also be used as a backup clock source (auxiliary clock) if the
HSE crystal oscillator fails. Refer to Section 11.4.11.

Calibration
The RC oscillator frequencies may vary from one chip to another due to manufacturing
process variations, this is why each device is factory calibrated by ST for 1 % accuracy
at TA = 25°C.
After reset, the factory calibration value is loaded in HSICAL[7:0] of RCC_ICSCR3.
If the application is subject to voltage or temperature variations, this may affect the RC
oscillator speed. The HSI16 frequency can be trimmed in the application using
HSITRIM[6:0] in RCC_ICSCR3.
For more details on how to measure the HSI16 frequency variation, refer to Section 11.4.23.
HSIRDY in RCC_CR indicates if the HSI16 RC is stable or not. At startup, the HSI16 RC
output clock is not released until this bit is set by hardware.
The HSI16 RC can be switched on and off using HSION in RCC_CR.
The HSI16 signal can also be used as a backup source (auxiliary clock) if the HSE crystal
oscillator fails. Refer to Section 11.4.11.

11.4.3

MSI (MSIS and MSIK) clocks
The MSI is made of four internal RC oscillators: MSIRC0 at 48 MHz, MSIRC1 at 4 MHz,
MSIRC2 at 3.072 MHz, and MSIRC3 at 400 kHz. Each oscillator feeds a prescaler providing
a division by 1, 2, 3, or 4. Two output clocks are generated from these divided oscillators:
MSIS that can be selected as system clock, and MSIK that can be selected by some
peripherals as kernel clock.

RM0456 Rev 6

<!-- pagebreak -->

