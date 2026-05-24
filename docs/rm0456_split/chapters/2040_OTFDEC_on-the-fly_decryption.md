2060

On-the-fly decryption engine (OTFDEC)

RM0456

The TZEN option bit in FLASH is used to activate TrustZone in the device.

52.3.3

•

TZEN = 1: TrustZone security is enabled in the product.

•

TZEN = 0: TrustZone security is disabled in the product.

OTFDEC on-the-fly decryption
Introduction
Typical usage for OTFDEC is shown on Figure 505.
Figure 505. Typical OTFDEC use in a SoC

data/system
cache

OCTOSPI

SoC boundary

SPI bus

OTFDEC

Instruction
cache

SPI NOR
Flash
MS48973V1

Original purpose of OTFDEC is to protect the confidentiality of read-only firmware libraries
stored in external SPI NOR flash devices.
A special locking scheme is available in OTFDEC in order to protect the integrity of the
decryption keys and also to protect the other configurations against software denial of
services attacks. OTFDEC access to most registers can be made privileged-only by setting
PRIV bit in OTFDEC_PRIVCFGR register. OTFDEC is only writeable by TrustZone CPU,
when TrustZone security is activated.
When OTFDEC is used in conjunction with OCTOSPI, it is mandatory to read the flash
memory using the Memory-mapped mode of the flash controller.
On top of decrypting on-the-fly, OTFDEC can also encrypt 32-bit word at a time (see
Section 52.5.3: Encrypting for OTFDEC for more details).

OTFDEC architecture
OTFDEC analyzes all AHB read transfers on the associated AHB bus. If the read request is
within one of the four regions programmed in OTFDEC, the control logic triggers a
keystream computation based on AES algorithm in counter mode. This keystream is then
used to decrypt on-the-fly the data present in the read transfer from the OCTOSPI AHB
master, tying low the HREADYOUT signal of this master while the keystream information is
being computed (this takes up to 11 cycles). Any accesses outside the enabled OTFDEC
regions belong to a non-encrypted region.
Each OTFDEC region is programmed through OTFDEC_RxCFGR,
OTFDEC_RxSTARTADDR, OTFDEC_RxENDADDR, OTFDEC_RxNONCER and

<!-- pagebreak -->

