RM0456 Rev 6

RM0456

LCD-TFT display controller (LTDC)
The internal signals of the LTDC are given in the table below.
Table 429. LTDC internal signals
Names

Signal type

Description

ltdc_hclk

Input

LTDC AHB clock

ltdc_pclk

Input

LTDC APB clock for register access

ltdc_ker_ck

Input

LTDC kernel clock used for LCD_CLK (pixel clock) generation

ltdc_it

Output

LTDC global interrupt request

ltdc_err_it

Output

LTDC global error interrupt request

ltdc_li

Output

LTDC line interrupt flag

The table below shows how the LTDC flags are connected.
Table 430. LTDC trigger interconnection

43.3.3

Trigger name

Direction

Trigger source/destination

ltdc_li

Output

gpdma_trigsel[47]

LTDC reset and clocks
The LTDC controller peripheral uses the following clock domains:
•

AHB clock domain (ltdc_aclk)
This domain contains the LTDC AHB master interface for data transfer from the
memories to the layer FIFO and the frame-buffer configuration register.

•

APB clock domain (ltdc_pclk)
This domain contains the global configuration registers and the interrupt register.

•

Pixel clock domain (LCD_CLK)
This domain contains the pixel data generation, the layer configuration register as well
as the LTDC interface signal generator. The LCD_CLK output must be configured
following the panel requirements. The LCD_CLK is generated from a specific PLL
output (refer to the reset and clock control section).

The table below summarizes the clock domain for each register.
Table 431. Clock domain for each register
LTDC register

Clock domain

LTDC_LxCR
LTDC_LxCFBAR

ltdc_hclk

LTDC_LxCFBLR
LTDC_LxCFBLNR

RM0456 Rev 6

<!-- pagebreak -->

1741

LCD-TFT display controller (LTDC)

RM0456

Table 431. Clock domain for each register (continued)
LTDC register

Clock domain

LTDC_SRCR
LTDC_IER

ltdc_pclk

LTDC_ISR
LTDC_ICR
LTDC_SSCR
LTDC_BPCR
LTDC_AWCR
LTDC_TWCR
LTDC_GCR
LTDC_BCCR
LTDC_LIPCR
LTDC_CPSR
LTDC_CDSR

Pixel clock (LCD_CLK)

LTDC_LxWHPCR
LTDC_LxWVPCR
LTDC_LxCKCR
LTDC_LxPFCR
LTDC_LxCACR
LTDC_LxDCCR
LTDC_LxBFCR
LTDC_LxCLUTWR

Care must be taken while accessing the LTDC registers, the APB bus is stalled during:
•

six ltdc_pclk periods + five LCD_CLK periods (five ltdc_hclk periods for register on AHB
clock domain) for register write access and update

•

seven ltdc_pclk periods + five LCD_CLK periods (five ltdc_hclk periods for register on
AHB clock domain) for register read access

For registers on ltdc_pclk clock domain, APB bus is stalled for six ltdc_pclk periods during
the register write accesses, and for seven ltdc_pclk periods during read accesses.
The LTDC controller can be reset by setting the corresponding bit in the RCC. It resets the
three clock domains.

<!-- pagebreak -->

