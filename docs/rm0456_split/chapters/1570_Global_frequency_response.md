1599

Multi-function digital filter (MDF)

RM0456

microphone pair can share the same data line. In this case, only three I/Os are
required.
In this example, the data transfer to memory can be performed either using the
interleaved- or the independent-transfer mode.
•

Picture in the center: sensor providing several data lines with a common clock
Each data line can represent different parameters (such as current or voltage). The
common clock can be provided either by the sensor or by the MDF. The data line can
be shared or not by two sensors if the sensor allows it. In the figure below, the sensor
does not allow the sharing of the data lines.
In this example, the data transfer to memory can be performed either using the
interleaved- or independent-transfer mode.

•

Picture on the right: two independent sensors connected to the MDF
Each of them has its dedicated clock and data lines. In this case, the data transfer to
memory must use the independent-transfer mode.
Figure 357. Sensor connection examples

Digital microphones with one
connected individually
MDF_SDI0

DMIC1
(LP)

SD
CK

MDF_CCK0

Sensor with several ADCs
and a common clock

LR

Independent sensors

MDF_SDI0
MDF_SDI0

GND

MDF_SDI1
MDF_CKI0

MDF_SDI2

DMIC4

MDF

LR

CK

Vcc

MDF_SDI2

MDF

MDF
MDF_SDI3

SD

MDF_CCK1

DMIC3
CK

LR

MDF_SDI0x
MDF_CKIx

MDF_CCK0

GND

Sensor

CK

MDF_SDI1

SD

DMIC2
LR

39.7.4

MSv62695V1

Vcc

Global frequency response
Figure 358 shows the global frequency response for a 16 kHz audio signal with a digital
microphone working at 1.024 MHz. The filter configuration is the following:

<!-- pagebreak -->

