0

CMDSTARTADDR[31:4]

DCACHE_
CMDREADDRR
Reset value

0

Res.

0x02C

0

0

DCACHE_
CMDRSADDRR
Reset value

1

WHITMON[31:0]

Reset value
0x028

0

CERRF

Res.

Res.

Res.

Res.

0

Res.

Res.

0

Res.

Res.

0

Res.

Res.

0

Res.

Res.

0

Res.

Res.

0

Res.

Res.

0

Res.

Res.

0

Res.

Res.

0

Res.

Res.

0

Res.

Res.

0

Res.

Res.

0

Res.

Res.

0

Res.

Res.

0

0

0

Res.

0x024

0

0

RHITMON[31:0]

Res.

0x020

0

DCACHE_
RMMONR

Res.

0x014

Reset value
DCACHE_
RHMONR
Reset value

Res.

0x010

DCACHE_FCR

Res.

0x00C

Res.

Reset value

0
CMDENDIE

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DCACHE_IER

Res.

0x008

Res.

Reset value

CCMDENDF

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

Res.

Res.

Res.

Res.

Res.

DCACHE_SR

Res.

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

0x004

Register
name

Res.

Offset

Res.

Table 94. DCACHE register map and reset values (continued)

RM0456

Power control (PWR)

10

Power control (PWR)

10.1

PWR introduction
The power controller manages all device power supplies and power modes transitions.

10.2

PWR main features
The power controller (PWR) main features are:
•

•

•

•

10.3

Power supplies and supply domains
–

Core domain (VCORE)

–

VDD domain

–

Backup domain

–

Analog domain (VDDA)

–

Supply for the SMPS power stage (available on SMPS packages)

–

VDDIO2 domain on port PG[15:2]

–

VDDUSB and optional VDD11USB for USB transceiver

–

VDDDSI and VDD11DSI for DSI transceiver (only for STM32U59x/5Ax/5Fx/5Gx)

System supply voltage regulation
–

SMPS step-down converter

–

Linear voltage regulator (LDO)

Power supply supervision
–

BOR monitor

–

PVD monitor

–

PVM monitor (VDDA, VDDUSB, VDDIO2)

–

Out of functional range temperature monitor

–

Out of functional range Backup domain voltage monitor

Power management
–

Operating modes

–

Voltage scaling control

–

Low-power modes

•

VBAT battery charging

•

TrustZone security and privileged protection

PWR pins and internal signals
Table 95. PWR input/output pins
Pin name

Signal type

Description

VDD

Supply

Main supply

GND

Supply

Main ground

RM0456 Rev 6

<!-- pagebreak -->

