482

Power control (PWR)

RM0456

1. Refer to Table 100.
2. A wake-up event can be generated with the peripheral interrupt signal. Refer to Section : Exiting a low-power mode.

Table 100. Functionalities depending on the working mode(1)

-

-

-

CPU

Y

-

-

-

-

-

-

-

-

-

-

-

-

Flash memory

O(2)

O(2)

-

-(5)

-

-

-

-

-

-

-

-

-

SRAM1

Y(3)

Y(4)

O(8)

-

O(8)

-

O(8)

-

-

-

-

-

-

SRAM2

Y(3)

Y(4)

O(8)

O(5)

O(8)

-

O(8)

-

O(6)

-

-

-

-

SRAM3

Y(3)

Y(4)

O(8)

O(5)

O(8)

-

O(8)

-

-

-

-

-

-

SRAM4

Y(3)

Y(4)

O(8)

-

O(8)

-

O(8)

-

-

-

-

-

-

SRAM5(7)

Y(3)

Y(4)

O(8)

-

O(8)

-

O(8)

-

-

-

-

-

-

(7)

Y(3)

Y(4)

O(8)

-

O(8)

-

O(8)

-

-

-

-

-

-

O

Peripheral

(7)

SRAM6

Run

Sleep
-

Wake-up capability

-

Wake-up capability

Standby Shutdown

Wake-up capability

Stop 3

Wake-up capability

Stop 2

Wake-up capability

Stop 0/1

VBAT

BKPSRAM

O

O

O

O(5)

FSMC

O

O

-

-

-

-

-

-

-

-

-

-

-

OCTOSPIx(7) (x =1,2)

O

O

O

O

O

-

-

-

-

-

-

-

-

-

-

-

(7)

HSPI1

O

O

-

-

-

-

-

-

-

-

-

-

-

Backup registers

Y

Y

Y

-

Y

-

Y

-

Y

-

Y

-

Y

Brownout reset (BOR)

Y

Y

Y

Y

Y

Y

Y

Y

Y

Y

-

-

-

Programmable voltage
detector (PVD)

O

O

O

O

O

O

-

-

-

-

-

-

-

Peripheral voltage
monitor

O

O

O

O

O

O

-

-

-

-

-

-

-

GPDMA

O

O

O

O(9)

-

-

-

-

-

-

-

-

-

O

O(10)

O

O(10)

-

-

-

-

-

-

-

LPDMA

O

O

DMA2D(7)

O

O

High-speed internal
(HSI16)

O

O

(11)

-

(11)

-

-

-

-

-

-

-

-

Oscillator HSI48

O

O

-

-

-

-

-

-

-

-

-

-

-

High-speed external
(HSE)

O

O

-

-

-

-

-

-

-

-

-

-

-

Low-speed internal (LSI)

O

O

O

-

O

-

O

-

O

-

-

-

O

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)
Table 100. Functionalities depending on the working mode(1) (continued)

-

-

-

Low-speed external
(LSE)

O

O

O

-

O

-

O

-

O

-

O

-

O

Multi-speed internal
(MSIS and MSIK)

O

O

(11)

-

(11)

-

-

-

-

-

-

-

-

Clock security system
(CSS)

O

O

-

-

-

-

-

-

-

-

-

-

-

Clock security system on
LSE

O

O

O

O

O

O

O

O

O

O

O

O

O

Backup domain voltage
monitoring, temperature
monitoring

O

O

O

O

O

O

O

O

O

O

-

-

O

RTC/TAMP

O

O

O

O

O

O

O

O

O

O

O

O

O

Number of TAMP tamper
pins

8

8

8

O

8

O

8

O

8

O

8

O

8

OTG_FS(7), OTG_HS(7),
USB(7), UCPD(7)

O(12)

O(12)

-

O(13)

-

-

-

-

-

-

-

-

-

USARTx
(x=1,2(7),3,4,5,6(7))

O

O

O(14) O(14)

-

-

-

-

-

-

-

-

-

Low-power UART
(LPUART)

O

O

O(14) O(14) O(14) O(14)

-

-

-

-

-

-

-

I2Cx (x = 1,2,4,5(7),6(7))

O

O

O(15) O(15)

-

-

-

-

-

-

-

I2C3

O

O

O(15) O(15) O(15) O(15)

-

-

-

-

-

-

-

O

O(16)

O(16)

-

-

-

-

-

-

-

-

-

(16)

O(16)

O(16)

O(16)

Peripheral

SPIx (x = 1,2)

Run

O

Sleep
-

-

VBAT

SPI3

O

O

FDCAN1

O

O

-

-

-

-

-

-

-

-

-

-

-

SDMMCx (x = 1,2(7))

O

-

Wake-up capability

-

Wake-up capability

Standby Shutdown

Wake-up capability

Stop 3

Wake-up capability

Stop 2

Wake-up capability

Stop 0/1

O

O

-

-

-

-

-

-

-

-

-

-

-

(7))

O

O

-

-

-

-

-

-

-

-

-

-

-

(7))

O

O

-

-

-

-

-

-

-

-

-

-

-

O(17)

O(17)

O(17)

-

-

-

-

-

-

-

SAIx (x = 1,2

ADCx (x = 1,2
ADC4

O

O

O(17)

DAC1 (2 converters)

O

O

O

-

O

-

-

-

-

-

-

-

-

VREFBUF

O

O

O

-

O

-

-

-

-

-

-

-

-

OPAMPx (x = 1,2)

O

O

O

-

O

-

-

-

-

-

-

-

-

COMPx (x = 1,2)

O

O

O

O

O

O

-

-

-

-

-

-

-

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

Table 100. Functionalities depending on the working mode(1) (continued)

-

-

-

Temperature sensor

O

O

O

-

O

-

-

-

-

-

-

-

-

Timers (TIMx)

O

O

-

-

-

-

-

-

-

-

-

-

-

GFXTIM(7)

O

O

-

-

-

-

-

-

-

-

-

-

-

LPTIMx (x = 1,3,4)

O

O

O(18) O(18) O(18) O(18)

-

-

-

-

-

-

-

LPTIM2

O

O

O(18) O(18)

Independent watchdog
(IWDG)

O

O

O

Window watchdog
(WWDG)

O

O

SysTick timer

O

O

Multi-function digital filter
(MDF)

O

O

O(19) O(19)

Audio digital filter (ADF)

O

O

O(19) O(19) O(19) O(19)

LTDC(7)

O

O

-

-

-

DSI(7)

O(20)

O(20)

-

-

GFXMMU

O

O

-

(7)

GPU2D

O

O

JPEG(7)

O

Digital camera interface
(DCMI)

Peripheral

Run

Sleep
-

Wake-up capability

-

Wake-up capability

Standby Shutdown

Wake-up capability

Stop 3

Wake-up capability

Stop 2

Wake-up capability

Stop 0/1

VBAT

-

-

-

-

-

-

-

-

-

O

O

O

O

O

O

O

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

O

-

-

-

-

-

-

-

-

-

-

-

O

O

-

-

-

-

-

-

-

-

-

-

-

Parallel synchronous
slave interface (PSSI)

O

O

-

-

-

-

-

-

-

-

-

-

-

CORDIC
co-processor (CORDIC)

O

O

-

-

-

-

-

-

-

-

-

-

-

Filter mathematical
accelerator (FMAC)

O

O

-

-

-

-

-

-

-

-

-

-

-

Touch sensing controller
(TSC)

O

O

-

-

-

-

-

-

-

-

-

-

-

Random number
generator (RNG)

O

O

-

-

-

-

-

-

-

-

-

-

-

AES and secure AES(7)

O

O

-

-

-

-

-

-

-

-

-

-

-

(7)

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)
Table 100. Functionalities depending on the working mode(1) (continued)

-

-

-

Public key accelerator
(PKA)(7)

O

O

-

-

-

-

-

-

-

-

-

-

-

On-the-fly decryption
(OTFDEC)(7)

O

O

-

-

-

-

-

-

-

-

-

-

-

HASH accelerator

O

O

-

-

-

-

-

-

-

-

-

-

-

CRC calculation unit

O

O

-

-

-

-

-

-

-

-

-

-

-

GPIOs

O

O

O

O

O

O

(21)

24
pins

(21)

24
pins

(22)

24
pins

-

Peripheral

Run

Sleep
-

Wake-up capability

-

Wake-up capability

Standby Shutdown

Wake-up capability

Stop 3

Wake-up capability

Stop 2

Wake-up capability

Stop 0/1

VBAT

1. Y = yes (enable). O = optional (disable by default, can be enabled by software). - = not available.
Gray cells highlight the wake-up capability in each mode.
2. The flash memory can be configured in power-down mode. By default, it is not in power-down mode.
3. The SRAMs can be powered on or off independently.
4. The SRAM clock can be gated on or off independently.
5. ECC error interrupt or NMI wakes up from this Stop mode.
6. 8 Kbytes, 56 Kbytes or full SRAM2 content can be preserved.
7. This feature is only available on some STM32U5 Series devices. Refer to the device datasheet for availability of its
associated peripheral.
8. Sub-blocks or full SRAM1, SRAM3, SRAM5, SRAM6, full SRAM2 and SRAM4 can be powered-off to save power
consumption. SRAM1, SRAM2, SRAM3, SRAM4, SRAM5, and SRAM6 can be accessed by GPDMA in Stop 0 and Stop 1
modes. SRAM4 can be accessed by LPDMA in Stop 0, Stop 1, and Stop 2 modes.
9. GPDMA transfers are functional and autonomous in Stop mode, and generates a wake-up interrupt on transfer events.
10. LPDMA transfers are functional and autonomous in Stop mode, and generates a wake-up interrupt on transfer events.
11. Some peripherals with autonomous mode and wake-up from Stop capability can request HSI16, MSIS or MSIK to be
enabled. In this case, the oscillator is woken up by the peripheral, and is automatically put off when no peripheral needs it.
12. OTG_FS and USB are functional in voltage scaling range 1, 2, and 3. OTG_HS is functional in voltage scaling
range 1 and 2.
13. OTG_HS cannot wake up from Stop 1 mode.
14. USART and LPUART reception and transmission is functional and autonomous in Stop mode, in asynchronous, and
in SPI master modes, and generates a wake-up interrupt on transfer events.
15. I2C reception and transmission is functional and autonomous in Stop mode, and generates a wake-up interrupt
on transfer events.
16. SPI reception and transmission is functional and autonomous in Stop mode, and generates a wake-up interrupt
on transfer events.
17. ADC conversion is functional and autonomous in Stop mode, and generates a wake-up interrupt on conversion events.
18. LPTIM is functional and autonomous in Stop mode, and generates a wake-up interrupt on events.
19. MDF and ADF is functional and autonomous in Stop mode, and generates a wake-up interrupt on events.
20. DSI is functional in voltage scaling range 1 and 2.
21. I/Os can be configured with internal pull-up, pull-down or floating in Standby mode.
22. I/Os can be configured with internal pull-up, pull-down or floating in Shutdown mode but the configuration is lost
when exiting Shutdown mode.

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

In addition, the power consumption in Run mode can be reduced by one of the following
means:
•

Slowing down the system clocks and configuring voltage scaling to lower-power
ranges.

•

Gating the clocks to the APB and AHB peripherals when they are unused.

•

Powering off unused RAMs

When a SRAM has been powered off, it can be powered on again by following the
procedure:
1.

Reset SRAMxPD in PWR_CR1.

2.

Wait for 1.6 µs.

3.

Set SRAMxEN in RCC_AHBxENR.

Debug mode
By default, the debug connection is lost if the application puts the MCU in Stop 0, Stop 1,
Stop 2, Stop 3, Standby, or Shutdown mode while the debug features are used. This is due
to the fact that the Cortex-M33 core is no longer clocked.
However, by setting some configuration bits in the DBGMCU control registers, the software
can be debugged even when using the low-power modes extensively. For more details,
refer to Section 75.2.5: Debug and low-power modes.

10.7.2

Autonomous peripherals and low-power background autonomous
mode (LPBAM)
Several peripherals support the autonomous mode which allows it to be functional and
perform DMA transfers in Stop 0, Stop 1, and Stop 2 modes. In addition, the low-power
background autonomous mode (LPBAM) is supported in Stop 2 mode, allowing to build
more complex use cases with autonomous peripherals, without any CPU wake-up thanks to
DMA transfers.

Stop 0 and Stop 1 modes
In Stop 0 and Stop 1 modes, the autonomous peripherals are ADC4, DAC1, LPTIMx
(x = 1 to 4), USARTx (x = 1 to 6), LPUART1, SPIx (x = 1 to 3), I2Cx (x = 1 to 6), MDF1,
ADF1, GPDMA1 and LPDMA1:
•

ADC4, DAC1, LPTIM1, LPTIM3, LPUART1, SPI3, I2C3 and ADF1 are autonomous
only with LPDMA1 and SRAM4.

•

LPTIM2, USARTx (x = 1 to 6), SPI1, SPI2, I2C1, I2C2, I2C4, I2C5, I2C6, and MDF1
are autonomous only with GPDMA1 and SRAM1 to SRAM6.

Stop 2 mode
In Stop 2 mode, the autonomous peripherals are ADC4, DAC1, LPTIM1, LPTIM3,
LPUART1, SPI3, I2C3, ADF1, and LPDMA1. In this mode, the SRAM4 can be accessed
by the LPDMA1.

Autonomous peripherals and LPBAM features
These autonomous peripherals support the following features:
•

<!-- pagebreak -->

Functionality in Stop mode thanks to its own independent clock (named kernel clock)
request capability: the peripheral kernel clock is automatically switched on when

RM0456 Rev 6

RM0456

Power control (PWR)
requested by a peripheral, and automatically switched off when no peripheral
requests it.
•

DMA transfers supported in Stop mode thanks to the system clock request capability:
the system clock (MSIS or HSI16) automatically switched on when requested by a
peripheral, and automatically switched off when no peripheral requests it. When the
system clock is requested by an autonomous peripheral, the system clock is woken up
and distributed to all peripherals enabled in the RCC. This allows the DMA to access
the enabled SRAM, and any enabled peripheral register (for instance GPIO or LPGPIO
registers).

•

Automatic start of the peripheral thanks to the hardware synchronous or asynchronous
triggers (such as I/Os edge detection and low-power timer event)

•

wake-up from Stop mode with peripheral interrupt

The GPDMA1 and LPDMA1 are fully functional and the linked-list is updated in Stop mode,
allowing the different DMA transfers to be linked without any CPU wake-up. This can be
used to chain different peripherals transfers, or to write peripherals registers in order to
change their configuration while remaining in Stop mode. LPBAM application drivers and
tools are available in STM32CubeMX, to help building those peripherals scenarios in
Stop 2 mode, thanks to LPDMA1 linked-list transfers.
The DMA transfers from memory to memory can be started by hardware synchronous or
asynchronous triggers, and the DMA transfers between peripherals and memories can also
be gated by those triggers.
Here below some use-cases that can be done while remaining in Stop mode:
•

•

ADC or DAC conversion triggered by a low-power timer (or any other trigger)
–

Wake-up from Stop mode on analog watchdog if the ADC conversion result is out
of the programmed thresholds

–

Wake-up from Stop mode on DMA buffer event

Audio digital filter data transfer into SRAM
–

•

I2C slave reception or transmission, SPI reception, UART/LPUART reception
–

•

•

•

Wake-up at the end of peripheral transfer or on DMA buffer event

I2C master transfer, SPI transmission, UART/LPUART transmission, triggered by a
low-power timer (or any other trigger)
–

Example: sensor periodic read

–

Wake-up at the end of peripheral transfer or on DMA buffer event

Bridges between peripherals
–

•

Wake-up from Stop on sound activity detection

Example: ADC converted data transferred by communication peripherals

Data transfer from/to GPIO/LPGPIO to/from SRAM for:
–

Controlling external components

–

Implementing data transmission and reception protocols

Data transfer from a SRAM to another one

RM0456 Rev 6

<!-- pagebreak -->

