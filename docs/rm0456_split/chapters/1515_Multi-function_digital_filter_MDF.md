RM0456 Rev 6

X

X

X

X

Res.

Res.

TRIMLP
OFFSETP[4:0]

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

Res.

Res.

OPAMP2_
LPOTR

0x18

Res.

Register
name

Res.

Offset

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

Table 367. OPAMP register map and reset values (continued)

TRIMLP
OFFSETN[4:0]
X

X

X

X

X

RM0456

Multi-function digital filter (MDF)

39

Multi-function digital filter (MDF)

39.1

MDF introduction
The multi-function digital filter (MDF) is a high-performance module dedicated to the
connection of external sigma-delta (Σ∆) modulators. It is mainly targeted for the following
applications:
•

audio capture signals

•

motor control

•

metering

The MDF features up to 6 digital serial interfaces (SITFx) and digital filters (DFLTx) with
flexible digital processing options to offer up to 24-bit final resolution.
The DFLTx of the MDF also include the filters of the audio digital filter (ADF).
The MDF can receive, via its serial interfaces, streams coming from various digital sensors.
The MDF supports the following standards allowing the connection of various ΣΔ modulator
sensors:
•

SPI interface

•

Manchester coded 1-wire interface

•

PDM interface

A flexible bitstream matrix (BSMX) allows the connection of any incoming bitstream to any
filter.
The MDF converts an input data stream into clean decimated digital data words. This
conversion is done thanks to low-pass digital filters and decimation blocks. In addition it is
possible to insert a high-pass filter or a DC offset correction block.
The conversion speed and resolution are adjustable according to configurable parameters
for digital processing: filter type, filter order, decimation ratio, integrator length. The
maximum output data resolution is up to 24 bits. There are two conversion modes: single
conversion and continuous modes. The data can be automatically stored in a system RAM
buffer through DMA, thus reducing the software overhead.
A flexible trigger interface can be used to control the conversion start. This timing control
can trigger simultaneous conversions or insert a programmable delay between conversions.
The MDF features an out-off limit detectors (OLD) function. There is one OLD for each
digital filter chain. Independent programmable thresholds are available for each OLD,
making it very suitable for over-current detection.
A short circuit detector (SCD) is also available for every selected bitstream. The SCD is able
to detect a short-circuit condition with a very short latency. Independent programmable
thresholds are offered in order to define the short circuit condition.
The digital processing is performed using only the kernel clock. The MDF requests the bus
interface clock (AHB clock) only when data must be transfered or when a specific event
requests the attention of the system processor.

RM0456 Rev 6

<!-- pagebreak -->

