RM0456 Rev 6

RM0456

27.4

Flexible static memory controller (FSMC)

AHB interface
The AHB slave interface allows internal CPUs and other bus master peripherals to access
the external memories.
AHB transactions are translated into the external device protocol. In particular, if the
selected external memory is 16- or 8-bit wide, 32-bit wide transactions on the AHB are split
into consecutive 16- or 8-bit accesses. The FMC chip select (FMC_NEx) does not toggle
between the consecutive accesses except in case of Access mode D when the Extended
mode is enabled.
The FMC generates an AHB error in the following conditions:
•

When reading or writing to a FMC bank (Bank 1 to 4) which is not enabled.

•

When reading or writing to the NOR flash bank while the FACCEN bit is reset in the
FMC_BCRx register.

The effect of an AHB error depends on the AHB master which has attempted the R/W
access:
•

If the access has been attempted by the Cortex®-M33 CPU, a hard fault interrupt is
generated.

•

If the access has been performed by a DMA controller, a DMA transfer error is
generated and the corresponding DMA channel is automatically disabled.

The AHB clock (HCLK) is the reference clock for the FMC.

27.4.1

Supported memories and transactions
General transaction rules
The requested AHB transaction data size can be 8-, 16- or 32-bit wide whereas the
accessed external device has a fixed data width. This may lead to inconsistent transfers.
Therefore, some simple transaction rules must be followed:
•

AHB transaction size and memory data size are equal
There is no issue in this case.

•

AHB transaction size is greater than the memory size:
In this case, the FMC splits the AHB transaction into smaller consecutive memory
accesses to meet the external data width. The FMC chip select (FMC_NEx) does not
toggle between the consecutive accesses. If the bus turnaround timings is configured
to any other value than 0, the FMC chip select (FMC_NEx) toggles between the
consecutive accesses. This feature is required when interfacing with FRAM memory.

•

AHB transaction size is smaller than the memory size:
The transfer may or not be consistent depending on the type of external device:
–

Accesses to devices that have the byte select feature (SRAM, ROM, PSRAM)
In this case, the FMC allows read/write transactions and accesses to the right data
through its byte lanes NBL[1:0].
Bytes to be written are addressed by NBL[1:0].
All memory bytes are read (NBL[1:0] are driven low during read transaction) and
the useless ones are discarded.

RM0456 Rev 6

<!-- pagebreak -->

