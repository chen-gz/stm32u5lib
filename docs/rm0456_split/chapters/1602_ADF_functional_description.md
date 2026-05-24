1674

Audio digital filter (ADF)

RM0456

40.4

ADF functional description

40.4.1

ADF block diagram
Figure 361. ADF block diagram
From the ADC

ADF

ADCITF 1/2

(2)

(ADC interface)

adf_sad_det

Sound activity (1)
detector (SAD)

adf_hclk

REGIF

adf_flt0_it
adf_flt0_dma

RX
FIFO0

AHB

Digital filter
processing (DFLT0)

bs_flt0

BSMX

bs0_r

SITF0

bs0_f

(serial
interface)

(3)

ADF_SDI0

TRIG0

adf_trgi0
adf_trgi1
...

adf_trgi

adf_trgi7

TRIG_CK
adf_trgo

ADF_CCK0

adf_bus_ckreq

CKGEN and control

adf_ker_ck

ADF_CCK1

adf_ker_ckreq

AHB clock domain

adf_ker_ck clock domain
MSv63650V3

1. Refer to Section 40.3: ADF implementation to check if the SAD is available.
2. Refer to Section 40.3: ADF implementation to check if the ADCITF is available, and which ADCs are
connected.
3. The number of trigger inputs depends on the product. Refer to Table 392 for details.

40.4.2

ADF pins and internal signals
Table 390. ADF external pins

Pin name
ADF_SDI0
ADF_CCKy
(y = 0,1)

Pin type
Input

Remarks
Data signal from external sensors

Input/output Clock outputs for external sensor, or common clock input from external sensors

Table 391. ADF internal signals
Signal name

Signal type

Remarks

adf_trgi

Input

Trigger inputs to control the acquisition (see Table 392: ADF trigger
connections for details)

adf_trgo

Output

Trigger output for synchronizing with other MDF instances

<!-- pagebreak -->

