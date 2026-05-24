1377

Analog-to-digital converter (ADC12)

RM0456

Bits 31:0 CALFACT[31:0]: Linearity or offset calibration factor
These bits can be written either by hardware or by software.
They contain the 32-bit offset or linearity calibration factor.
When CAPTURE_COEF is set, the calibration factor of the analog block is read back and stored in
CALFACT[31:0], indexed by CALINDEX[3:0] bits.
When LATCH_COEF is set, the calibration factor of the analog block is updated with the value
programmed in CALFACT[31:0], indexed by CALINDEX[3:0] bits.
To read all calibration factors, perform nine accesses to the ADC_CALFACT2 register.
To write all calibration factors, perform eight accesses to the ADC_CALFACT2 register.
Note: The software is allowed to write these bits only when ADEN = 1, ADSTART = 0 and
JADSTART = 0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).

33.7

ADC common registers
These registers define the control and status registers common to master and slave ADCs.

33.7.1

ADC common status register (ADC12_CSR)
Address offset: 0x00
Reset value: 0x0000 0000
The address offset is relative to the master ADC base address + 0x300.
This register provides an image of the status bits of the different ADCs. Nevertheless, it is
read-only and does not clear the different status bits. Instead, each status bit must be
cleared by writing 1 to it in the corresponding ADC_ISR register.
ADC1 and ADC2 are controlled by the same interface.
This register is available only the devices that support dual mode (see Section 33.3: ADC
implementation).

31

30

29

Res. Res. Res.

28

27

26

LDORDY
_SLV

Res.

Res.

12

11

10

LDORDY
_MST

Res.

Res.

r
15

14

13

Res. Res. Res.

r

25

24

23

AWD3_ AWD2_
SLV
SLV

AWD1_
SLV

22

21

JEOS_ JEOC_
SLV
SLV

20

19

18

OVR_
SLV

EOS_
SLV

EOC_
SLV

17

EOSMP_ ADRDY_
SLV
SLV

r

r

r

r

r

r

r

r

r

r

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

OVR_
MST

EOS_
MST

EOC_
MST

r

r

r

AWD3_ AWD2_
MST
MST
r

r

AWD1_
MST
r

JEOS_ JEOC_
MST
MST
r

r

EOSMP_ ADRDY_
MST
MST
r

Bits 31:29 Reserved, must be kept at reset value.
Bit 28 LDORDY_SLV: ADC voltage regulator ready flag of the slave ADC
This bit is a copy of the LDORDY bit of the corresponding ADCx+1_ISR register.
Bits 27:26 Reserved, must be kept at reset value.
Bit 25 AWD3_SLV: Analog watchdog 3 flag of the slave ADC
This bit is a copy of the AWD3 bit in the corresponding ADCx+1_ISR register.
Bit 24 AWD2_SLV: Analog watchdog 2 flag of the slave ADC
This bit is a copy of the AWD2 bit in the corresponding ADCx+1_ISR register.

<!-- pagebreak -->

16

RM0456 Rev 6

r

RM0456

Analog-to-digital converter (ADC12)

Bit 23 AWD1_SLV: Analog watchdog 1 flag of the slave ADC
This bit is a copy of the AWD1 bit in the corresponding ADCx+1_ISR register.
Bit 22 JEOS_SLV: End of injected sequence flag of the slave ADC
This bit is a copy of the JEOS bit in the corresponding ADCx+1_ISR register.
Bit 21 JEOC_SLV: End of injected conversion flag of the slave ADC
This bit is a copy of the JEOC bit in the corresponding ADCx+1_ISR register.
Bit 20 OVR_SLV: Overrun flag of the slave ADC
This bit is a copy of the OVR bit in the corresponding ADCx+1_ISR register.
Bit 19 EOS_SLV: End of regular sequence flag of the slave ADC
This bit is a copy of the EOS bit in the corresponding ADCx+1_ISR register.
Bit 18 EOC_SLV: End of regular conversion of the slave ADC
This bit is a copy of the EOC bit in the corresponding ADCx+1_ISR register.
Bit 17 EOSMP_SLV: End of sampling phase flag of the slave ADC
This bit is a copy of the EOSMP2 bit in the corresponding ADCx+1_ISR register.
Bit 16 ADRDY_SLV: Slave ADC ready
This bit is a copy of the ADRDY bit in the corresponding ADCx+1_ISR register.
Bits 15:13 Reserved, must be kept at reset value.
Bit 12 LDORDY_MST: ADC voltage regulator ready flag of the master ADC
This bit is a copy of the LDORDY bit of the corresponding ADC_ISR register.
Bits 11:10 Reserved, must be kept at reset value.
Bit 9 AWD3_MST: Analog watchdog 3 flag of the master ADC
This bit is a copy of the AWD3 bit in the corresponding ADC_ISR register.
Bit 8 AWD2_MST: Analog watchdog 2 flag of the master ADC
This bit is a copy of the AWD2 bit in the corresponding ADC_ISR register.
Bit 7 AWD1_MST: Analog watchdog 1 flag of the master ADC
This bit is a copy of the AWD1 bit in the corresponding ADC_ISR register.
Bit 6 JEOS_MST: End of injected sequence flag of the master ADC
This bit is a copy of the JEOS bit in the corresponding ADC_ISR register.
Bit 5 JEOC_MST: End of injected conversion flag of the master ADC
This bit is a copy of the JEOC bit in the corresponding ADC_ISR register.
Bit 4 OVR_MST: Overrun flag of the master ADC
This bit is a copy of the OVR bit in the corresponding ADC_ISR register.
Bit 3 EOS_MST: End of regular sequence flag of the master ADC
This bit is a copy of the EOS bit in the corresponding ADC_ISR register.
Bit 2 EOC_MST: End of regular conversion of the master ADC
This bit is a copy of the EOC bit in the corresponding ADC_ISR register.
Bit 1 EOSMP_MST: End of Sampling phase flag of the master ADC
This bit is a copy of the EOSMP bit in the corresponding ADC_ISR register.
Bit 0 ADRDY_MST: Master ADC ready
This bit is a copy of the ADRDY bit in the corresponding ADC_ISR register.

RM0456 Rev 6

<!-- pagebreak -->

