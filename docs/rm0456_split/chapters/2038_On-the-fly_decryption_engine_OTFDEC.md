2037

On-the-fly decryption engine (OTFDEC)

52

RM0456

On-the-fly decryption engine (OTFDEC)
This section only applies to STM32U545/585/5Ax/5Gx devices.

52.1

Introduction
OTFDEC allows on-the-fly decryption of the AHB traffic based on the read request address
information. Four independent and non-overlapping encrypted regions can be defined in
OTFDEC.
OTFDEC uses AES-128 in counter mode to achieve the lowest possible latency. As a
consequence, each time the content of one encrypted region is changed, the entire region
must be re-encrypted with a different cryptographic context (key or initialization vector). This
constraint makes OTFDEC suitable to decrypt read-only data or code, stored in external
NOR flash.

Note:

When OTFDEC is used in conjunction with OCTOSPI, it is mandatory to access the flash
memory using the memory-mapped mode of the flash memory controller.
When security is enabled in the product, OTFDEC can be programmed only by a secure
host.

52.2

OTFDEC main features
•

•

•

<!-- pagebreak -->

