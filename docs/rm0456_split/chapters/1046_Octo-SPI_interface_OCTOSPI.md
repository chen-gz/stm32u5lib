1045

Octo-SPI interface (OCTOSPI)

RM0456

28

Octo-SPI interface (OCTOSPI)

28.1

OCTOSPI introduction
The OCTOSPI supports most external serial memories such as serial PSRAMs, serial
NAND and serial NOR flash memories, HyperRAM™ and HyperFlash™ memories, with
the following functional modes:
•

indirect mode: all the operations are performed using the OCTOSPI registers to preset
commands, addresses, data, and transfer parameters.

•

automatic status-polling mode: the external memory status register is periodically read
and an interrupt can be generated in case of flag setting. This feature is only available
in regular-command protocol.

•

memory-mapped mode: the external memory is memory mapped and it is seen by the
system as if it was an internal memory, supporting both read and write operations.

The OCTOSPI supports the following protocols with associated frame formats:

28.2

<!-- pagebreak -->

