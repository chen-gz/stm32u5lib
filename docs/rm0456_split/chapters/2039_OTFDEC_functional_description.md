On-the-fly 128-bit decryption during the OCTOSPI Memory-mapped read operations
(single or multiple).
–

Use of AES in counter (CTR) mode, with two 128-bit keystream buffers

–

Support for any read size

–

Physical address of the reads used for the encryption/decryption

Up to four independent encrypted regions
–

Granularity of the region definition: 4096 bytes

–

Region configuration write-locking mechanism

–

Each region has its own 128-bit key, two bytes firmware version, and eight bytes
application-defined nonce. At least one of those must be changed each time an
encryption is performed by the application.

Encryption keys confidentiality and integrity protection
–

Write-only registers, with software locking mechanism

–

Availability of 8-bit CRC as public key information

•

Support for OCTOSPI pre-fetching mechanism

•

Possibility to select an enhanced encryption mode to add a proprietary layer of
protection on top of AES stream cipher (execute only)

•

Privileged-aware AMBA AHB slave peripheral, accessible through 32-bit word single
accesses only (otherwise an AHB bus error is generated, and write accesses are
ignored)

•

Secure only programming if TrustZone security is enabled in the product

•

Encryption mode

RM0456 Rev 6

RM0456

On-the-fly decryption engine (OTFDEC)

52.3

OTFDEC functional description

52.3.1

OTFDEC block diagram
Figure 504. OTFDEC block diagram

OTFDEC
otfdec_it

IRQ
interface
Banked registers
(x = 1 to 4)
Control
Logic

RxADDR

RxNONCE

IVs

RxKEYR
Key

32-bit AHB bus

RxCFGR
AHB lite
slave
interface

...
AES-CTR

otfdec_hclk
To / from
control logic

32-bit AHB bus

hreadyout_o

Proprietary
XOR

hrdata_o[31:0]

hreadyout_i
hrdata_in[31:0]

XOR
haddr[31:0]

haddr[31:0]
AHB memory
interface

Other AHB signals

Other AHB signals

OCTOSPI (Slave)

AHB clock domain

AHB memory
interface

See (1)

Keystream[0]
Keystream[1]

1. otfdec_tzen

52.3.2

OTFDEC internal signals
Table 490 describes a list of useful to know internal signals available at OTFDEC level, not
at the product level (on pads).
Table 490. OTFDEC internal input/output signals
Signal name

Signal type

Description

otfdec_hclk

Digital input

otfdec_it

Digital output OTFDEC global interrupt request

otfdec_tzen

Digital input

AHB bus clock

OTFDEC TrustZone enable, controlling TrustZone features of the
peripheral (TZEN)

RM0456 Rev 6

<!-- pagebreak -->

