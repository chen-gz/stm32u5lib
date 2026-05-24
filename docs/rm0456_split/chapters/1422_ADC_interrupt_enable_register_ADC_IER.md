1443

Analog-to-digital converter (ADC4)

RM0456

Bit 4 OVR: ADC overrun
This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete
while the EOC flag was already set. It is cleared by software writing 1 to it.
0: No overrun occurred (or the flag event was already acknowledged and cleared by software)
1: Overrun has occurred
Bit 3 EOS: End of sequence flag
This bit is set by hardware at the end of the conversion of a sequence of channels selected by the
CHSEL bits. It is cleared by software writing 1 to it.
0: Conversion sequence not complete (or the flag event was already acknowledged and cleared by
software)
1: Conversion sequence complete
Bit 2 EOC: End of conversion flag
This bit is set by hardware at the end of each conversion of a channel when a new data result is
available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR
register.
0: Channel conversion not complete (or the flag event was already acknowledged and cleared by
software)
1: Channel conversion complete
Bit 1 EOSMP: End of sampling flag
This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by
software by writing 1 to it.
0: Not at the end of the sampling phase (or the flag event was already acknowledged and cleared by
software)
1: End of sampling phase reached
Bit 0 ADRDY: ADC ready
This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches
a state where it is ready to accept conversion requests.
It is cleared by software writing 1 to it.
0: ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared
by software)
1: ADC is ready to start conversion

34.7.2

ADC interrupt enable register (ADC_IER)
Address offset: 0x04
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

AWD3
IE

AWD2
IE

AWD1
IE

rw

rw

rw

Res.

Res.

Res.

LDORD EOCAL
YIE
IE
rw

<!-- pagebreak -->

rw

RM0456 Rev 6

Res.

Res.

OVRIE

EOSIE

rw

rw

EOSMP ADRDY
EOCIE
IE
IE
rw

rw

rw

RM0456

Analog-to-digital converter (ADC4)

Bits 31:13 Reserved, must be kept at reset value.
Bit 12 LDORDYIE: LDO ready interrupt enable
This bit is set and cleared by software. It is used to enable/disable the LDORDY interrupt.
0: LDO ready interrupt disabled
1: LDO ready interrupt enabled. An interrupt is generated when the LDO output is ready.
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensure that no conversion is ongoing).
Bit 11 EOCALIE: End of calibration interrupt enable
This bit is set and cleared by software to enable/disable the end of calibration interrupt.
0: End of calibration interrupt disabled
1: End of calibration interrupt enabled
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bit 10 Reserved, must be kept at reset value.
Bit 9 AWD3IE: Analog watchdog 3 interrupt enable
This bit is set and cleared by software to enable/disable the analog watchdog interrupt.
0: Analog watchdog interrupt disabled
1: Analog watchdog interrupt enabled
Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bit 8 AWD2IE: Analog watchdog 2 interrupt enable
This bit is set and cleared by software to enable/disable the analog watchdog interrupt.
0: Analog watchdog interrupt disabled
1: Analog watchdog interrupt enabled
Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bit 7 AWD1IE: Analog watchdog 1 interrupt enable
This bit is set and cleared by software to enable/disable the analog watchdog interrupt.
0: Analog watchdog interrupt disabled
1: Analog watchdog interrupt enabled
Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bits 6:5 Reserved, must be kept at reset value.
Bit 4 OVRIE: Overrun interrupt enable
This bit is set and cleared by software to enable/disable the overrun interrupt.
0: Overrun interrupt disabled
1: Overrun interrupt enabled. An interrupt is generated when the OVR bit is set.
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bit 3 EOSIE: End of conversion sequence interrupt enable
This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt.
0: EOS interrupt disabled
1: EOS interrupt enabled. An interrupt is generated when the EOS bit is set.
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

Bit 2 EOCIE: End of conversion interrupt enable
This bit is set and cleared by software to enable/disable the end of conversion interrupt.
0: EOC interrupt disabled
1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set.
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bit 1 EOSMPIE: End of sampling flag interrupt enable
This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt.
0: EOSMP interrupt disabled.
1: EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set.
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bit 0 ADRDYIE: ADC ready interrupt enable
This bit is set and cleared by software to enable/disable the ADC Ready interrupt.
0: ADRDY interrupt disabled.
1: ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set.
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).

<!-- pagebreak -->

