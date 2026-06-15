const std = @import("std");

const c = @cImport({
    @cDefine("STM32U575xx", "");
    @cDefine("USE_HAL_DRIVER", "");
    @cDefine("__ARM_ARCH_PROFILE", "'M'");
    @cInclude("stm32u5xx_hal.h");
});

pub export fn main() c_int {
    // Initialize the HAL Library
    _ = c.HAL_Init();

    // Enable GPIO clocks for:
    // - GPIOB (Blue LED, Pin 7)
    // - GPIOC (Green LED, Pin 7)
    // - GPIOG (Red LED, Pin 2)
    // Also enable PWR clock to allow configuring VDDIO2 for GPIOG
    c.RCC.*.AHB2ENR1 |= c.RCC_AHB2ENR1_GPIOBEN | c.RCC_AHB2ENR1_GPIOCEN | c.RCC_AHB2ENR1_GPIOGEN;
    c.RCC.*.AHB3ENR |= c.RCC_AHB3ENR_PWREN;
    
    // Read back to delay and synchronize clock enablement
    _ = c.RCC.*.AHB2ENR1;
    _ = c.RCC.*.AHB3ENR;

    // Enable VDDIO2 for GPIOG (since GPIOG is in VDDIO2 independent domain)
    c.PWR.*.SVMCR |= c.PWR_SVMCR_IO2SV;

    // Configure GPIO pins for the LEDs
    var gpio_init = std.mem.zeroes(c.GPIO_InitTypeDef);
    gpio_init.Mode = c.GPIO_MODE_OUTPUT_PP;
    gpio_init.Pull = c.GPIO_NOPULL;
    gpio_init.Speed = c.GPIO_SPEED_FREQ_LOW;

    // Initialize Blue LED (PB7)
    gpio_init.Pin = c.GPIO_PIN_7;
    c.HAL_GPIO_Init(c.GPIOB, &gpio_init);

    // Initialize Green LED (PC7)
    gpio_init.Pin = c.GPIO_PIN_7;
    c.HAL_GPIO_Init(c.GPIOC, &gpio_init);

    // Initialize Red LED (PG2)
    gpio_init.Pin = c.GPIO_PIN_2;
    c.HAL_GPIO_Init(c.GPIOG, &gpio_init);

    while (true) {
        // Blink sequence: Blue -> Green -> Red -> Repeat
        c.HAL_GPIO_WritePin(c.GPIOB, c.GPIO_PIN_7, c.GPIO_PIN_SET);
        c.HAL_Delay(250);
        c.HAL_GPIO_WritePin(c.GPIOB, c.GPIO_PIN_7, c.GPIO_PIN_RESET);

        c.HAL_GPIO_WritePin(c.GPIOC, c.GPIO_PIN_7, c.GPIO_PIN_SET);
        c.HAL_Delay(250);
        c.HAL_GPIO_WritePin(c.GPIOC, c.GPIO_PIN_7, c.GPIO_PIN_RESET);

        c.HAL_GPIO_WritePin(c.GPIOG, c.GPIO_PIN_2, c.GPIO_PIN_SET);
        c.HAL_Delay(250);
        c.HAL_GPIO_WritePin(c.GPIOG, c.GPIO_PIN_2, c.GPIO_PIN_RESET);
    }
}

// Override SysTick_Handler to increment the HAL tick counter
pub export fn SysTick_Handler() void {
    c.HAL_IncTick();
}

// Required freestanding panic handler
pub fn panic(msg: []const u8, error_return_trace: ?*std.builtin.StackTrace, ret_addr: ?usize) noreturn {
    _ = msg;
    _ = error_return_trace;
    _ = ret_addr;
    while (true) {
        @breakpoint();
    }
}

// Dummy __libc_init_array to satisfy the startup assembly requirements
pub export fn __libc_init_array() void {}

