1443

Analog-to-digital converter (ADC4)

RM0456

Table 325. ADC main features(1) (continued)
ADC modes/features

STM32U535/545/ STM32U59x/5Ax
575/585
/5Fx/5Gx
ADC1

ADC1

ADC2

STM32U535/545/575/585
/59x/5Ax/5Fx/5Gx
ADC4

Gain compensation

X

-

Number of analog watchdogs

3

3

Wake-up from Stop mode

-

X(2)

1. Note: ‘X’ = supported, ‘-’= not supported.
2. Wake-up supported from Stop 0, Stop 1, and Stop 2 modes.

Table 326. Memory location of the temperature sensor calibration values
Name

Description

Memory address

TS_CAL1

Temperature sensor 14-bit raw data
acquired by ADC1 at 30 °C (± 5 °C),
VDDA = VREF+ = 3.0 V (±10 mV)

0x0BFA 0710 - 0x0BFA 0711

TS_CAL2

Temperature sensor 14-bit raw data
acquired by ADC1 at 130 °C (± 5 °C),
VDDA = VREF+ = 3.0 V (±10 mV)

0x0BFA 0742 - 0x0BFA 0743

Table 327. Memory location of the internal reference voltage sensor
calibration value
Name
VREFINT_CAL

<!-- pagebreak -->

