1377

Analog-to-digital converter (ADC12)

RM0456

Bit 3 EOS: End of regular sequence flag
This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is
cleared by software writing 1 to it.
0: Regular conversions sequence not complete (or the flag event was already acknowledged and
cleared by software)
1: Regular conversions sequence complete
Bit 2 EOC: End of conversion flag
This bit is set by hardware at the end of each regular conversion of a channel when a new data is
available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR
register
0: Regular channel conversion not complete (or the flag event was already acknowledged and
cleared by software)
1: Regular channel conversion complete
Bit 1 EOSMP: End of sampling flag
This bit is set by hardware during the conversion of any channel (only for regular channels), at the
end of the sampling phase.
0: not at the end of the sampling phase (or the flag event was already acknowledged and cleared by
software)
1: End of sampling phase reached
Bit 0 ADRDY: ADC ready
This bit is set by hardware after the ADC has been enabled (bit ADEN = 1) and when the ADC
reaches a state where it is ready to accept conversion requests.
It is cleared by software writing 1 to it.
0: ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared
by software)
1: ADC is ready to start conversion

33.6.2

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

Res.

Res.

Res.

Res.

Res.

AWD3 AWD2 AWD1
JEOSIE JEOCIE OVRIE
IE
IE
IE
rw

rw

rw

rw

rw

rw

EOSIE
rw

EOSMP ADRDY
EOCIE
IE
IE
rw

rw

Bits 31:10 Reserved, must be kept at reset value.
Bit 9 AWD3IE: Analog watchdog 3 interrupt enable
This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt.
0: Analog watchdog 3 interrupt disabled
1: Analog watchdog 3 interrupt enabled
Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which
ensures that no conversion is ongoing).

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

Analog-to-digital converter (ADC12)

Bit 8 AWD2IE: Analog watchdog 2 interrupt enable
This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt.
0: Analog watchdog 2 interrupt disabled
1: Analog watchdog 2 interrupt enabled
Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which
ensures that no conversion is ongoing).
Bit 7 AWD1IE: Analog watchdog 1 interrupt enable
This bit is set and cleared by software to enable/disable the analog watchdog 1 interrupt.
0: Analog watchdog 1 interrupt disabled
1: Analog watchdog 1 interrupt enabled
Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which
ensures that no conversion is ongoing).
Bit 6 JEOSIE: End of injected sequence of conversions interrupt enable
This bit is set and cleared by software to enable/disable the end of injected sequence of conversions
interrupt.
0: JEOS interrupt disabled
1: JEOS interrupt enabled. An interrupt is generated when the JEOS bit is set.
Note: Software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected
conversion is ongoing).
Bit 5 JEOCIE: End of injected conversion interrupt enable
This bit is set and cleared by software to enable/disable the end of an injected conversion interrupt.
0: JEOC interrupt disabled.
1: JEOC interrupt enabled. An interrupt is generated when the JEOC bit is set.
Note: Software is allowed to write this bit only when JADSTART = 0 (which ensures that no regular
conversion is ongoing).
Bit 4 OVRIE: Overrun interrupt enable
This bit is set and cleared by software to enable/disable the Overrun interrupt of a regular
conversion.
0: Overrun interrupt disabled
1: Overrun interrupt enabled. An interrupt is generated when the OVR bit is set.
Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular
conversion is ongoing).
Bit 3 EOSIE: End of regular sequence of conversions interrupt enable
This bit is set and cleared by software to enable/disable the end of regular sequence of conversions
interrupt.
0: EOS interrupt disabled
1: EOS interrupt enabled. An interrupt is generated when the EOS bit is set.
Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular
conversion is ongoing).
Bit 2 EOCIE: End of regular conversion interrupt enable
This bit is set and cleared by software to enable/disable the end of a regular conversion interrupt.
0: EOC interrupt disabled.
1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set.
Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular
conversion is ongoing).

RM0456 Rev 6

<!-- pagebreak -->

