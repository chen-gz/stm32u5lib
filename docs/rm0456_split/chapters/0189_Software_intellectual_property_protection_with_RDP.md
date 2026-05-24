RM0456 Rev 6

RM0456

3.12.1

System security

Software intellectual property protection with RDP
As described in Section 3.10.1, the hardware RDP mechanism automatically controls the
accesses to secrets provisioned in the device.
The protection of these secrets are defined in the table below.
Table 24. Software intellectual property protection with RDP

RDP protection level

Secrets protection

Level 0

Device open

No special protections.

Level 0.5(1)

Device partially
closed

All peripherals and memories mapped as secure during secure boot cannot be
dumped, debugged, or traced

Level 1

Data and code stored in embedded flash memory, encrypted external flash
Device memories
memory(2), SRAM2, and backup registers are no more accessible through the
protected
debugger.

Level 2

Device closed

All data and code stored in the device or encrypted in external flash memory
cannot be dumped clear-text, debugged or traced.

1. Only applicable when TrustZone security is activated in the product.
2. External flash memory area decrypted on-the-fly with OTFDEC peripheral.

3.12.2

Software intellectual property protection with OTFDEC
As described in Section 3.9.3, the OTFDEC associated with the OCTOSPI is able to decrypt
on the fly, the encrypted images stored in external SPI flash memories.
Thanks to this feature, the devices allow the installation of intellectual properties, in one of
the following ways:
•

over the air, with the image already encrypted with a key provisioned in the device

•

through a provisioning host located in a trusted or a non-trusted environment/facility

Figure 17 illustrates this last case, with the provisioning, in a non-trusted environment, of
software intellectual properties both in the embedded flash memory and in an external SPI
flash memory (encrypted).
Note:

Since the OTFDEC uses the AES in counter mode (CTR) to achieve the lowest possible
latency, each time the content of one encrypted region is changed, the corresponding
cryptographic context (key or initialization vector) must be changed. This constraint makes
OTFDEC suitable to decrypt read-only data or code, stored in external NOR flash memory.

RM0456 Rev 6

<!-- pagebreak -->

191

System security

RM0456
Figure 17. External flash memory protection using SFI

Provisioning
STM32
JTAG

SRAM2
Secure
3
bootloader
part II

1

...

6

SPI

ROM

I2C

Secure
bootloader
part I

FDCAN

1

USB

Option
bytes

2

USART

Host

Flash

4

OTF
DEC

5

OctoSPI

Flash
memory

6

SRAM
1

See details in section “Provisioning” below the figure.

Secure boot
STM32
Application

Flash
Secure boot

1 OTF
DEC

tamper
1

...

2

OctoSPI

Flash
memory

2

See details in section “Secure boot” below the figure.
MSv65211V2

Provisioning
Assuming the device is virgin, the first step is to provision both flash memories, as detailed
below:
1.

2.

<!-- pagebreak -->

