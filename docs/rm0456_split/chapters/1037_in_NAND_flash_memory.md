RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)
Figure 140. Access to non ‘CE don’t care’ NAND-flash

1. CPU wrote byte 0x00 at address 0x7001 0000.
2. CPU wrote byte A7~A0 at address 0x7002 0000.
3. CPU wrote byte A16~A9 at address 0x7002 0000.
4. CPU wrote byte A24~A17 at address 0x7002 0000.
5. CPU wrote byte A25 at address 0x7802 0000: FMC performs a write access using FMC_PATT timing
definition, where ATTHOLD ≥ 7 (providing that (7+1) × HCLK = 112 ns > tWB max). This guarantees that
NCE remains low until R/NB goes low and high again (only requested for NAND flash memories where
NCE is not don’t care).

When this functionality is required, it can be ensured by programming the MEMHOLD value
to meet the tWB timing. However any CPU read access to the NAND flash memory has a
hold delay of (MEMHOLD + 2) HCLK cycles and CPU write access has a hold delay of
(MEMHOLD) HCLK cycles inserted between the rising edge of the NWE signal and the next
access.
To cope with this timing constraint, the attribute memory space can be used by
programming its timing register with an ATTHOLD value that meets the tWB timing, and by
keeping the MEMHOLD value at its minimum value. The CPU must then use the common
memory space for all NAND flash read and write accesses, except when writing the last
address byte to the NAND flash device, where the CPU must write to the attribute memory
space.

27.7.6

Computation of the error correction code (ECC)
in NAND flash memory
The FMC NAND Card controller includes two error correction code computation hardware
blocks, one per memory bank. They reduce the host CPU workload when processing the
ECC by software.

RM0456 Rev 6

<!-- pagebreak -->

