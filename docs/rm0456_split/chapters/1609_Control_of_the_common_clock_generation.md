RM0456 Rev 6

RM0456

Audio digital filter (ADF)
Figure 365. CKGEN overview

ADF
CKGEN

TRIG0
TRIG_CK

adf_proc_ck

CKGDEN

PROCDIV[6:0]

Digital
processing:
DFLT0, SAD
Interfaces:
ADCITF[2:1],
SITF0

CKGMOD

CCKDIV[3:0]

CCK0DIR
CCK0EN

÷ 1 to 128

adf_ker_ck

0

ADF_CCK0

÷ 1 to 16

1

cck_trg
(from TRIG_CK)

ADF_CCK1

CCK1EN
CCK1DIR
MSv63653V1

The trigger logic for CKGEN is handled by the block TRG_CK. As shown in Figure 370, the
CCKDIV divider can be triggered on the rising or falling edge of an external trigger source.
When the proper trigger condition occurs, the cck_trg signal goes to high, allowing the
CCKDIV divider to start. The TRG_CK logic is reset when CKGDEN is set to 0.
This feature can be helpful to synchronize the ADF_CCKy (y = 0,1) clock of several ADF
instances, or to synchronize the clock generation to a timer event.
The application can control the activation of the ADF_CCK0 or ADF_CCK1 pin thanks to
CCK0EN/CCK1EN and CCK0DIR/CCK1DIR bits:
•

CCKyEN is used to enable the CCKDIV, and thus generates a clock for the external
sensors.

•

CCKyDIR is used to control the direction of the ADF_CCKy pin (input or output)
Table 393. Control of the common clock generation(1)

CCKyEN

CCKyDIR

Description

0

0

The ADF_CCKy pin is in input. An external clock can be connected to the ADF_CCKy pin
and used by the SITF0 in order to decode the serial stream

0

1

The ADF_CCKy pin is in output. No clock is generated. The ADF_CCKy pin is driven low.

1

1

The ADF_CCKy pin is in output. A clock is generated on the ADF_CCKy pin. The SITF0
can use this pin as clock source in order to decode the serial stream

1. The configuration with CCKyEN = 1 and CCyDIR = 0 is not shown must be avoided (no interest).

Note:

The adf_proc_ck must be enabled (by CKGDEN = 1) before enabling other blocks (such as
SITF0 or DFLT0).

RM0456 Rev 6

<!-- pagebreak -->

