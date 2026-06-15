const std = @import("std");

pub fn build(b: *std.Build) void {
    // Resolve target query for STM32U575 (Cortex-M33 bare metal with hard FPU)
    const target = b.resolveTargetQuery(.{
        .cpu_arch = .thumb,
        .os_tag = .freestanding,
        .abi = .eabihf,
        .cpu_model = .{ .explicit = &std.Target.arm.cpu.cortex_m33 },
        .cpu_features_add = std.Target.arm.cpu.cortex_m33.features,
    });

    // Set ReleaseSmall as the default optimization for embedded firmware
    const optimize = b.standardOptimizeOption(.{
        .preferred_optimize_mode = .ReleaseSmall,
    });

    // Define the executable
    const exe = b.addExecutable(.{
        .name = "stm32u5-led.elf",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    // Configure the custom entry point symbol
    exe.entry = .{ .symbol_name = "Reset_Handler" };

    // Add include paths for CMSIS and HAL driver to root_module
    exe.root_module.addIncludePath(b.path("deps/cmsis-core/CMSIS/Core/Include"));
    exe.root_module.addIncludePath(b.path("deps/cmsis-device-u5/Include"));
    exe.root_module.addIncludePath(b.path("deps/stm32u5xx-hal-driver/Inc"));
    exe.root_module.addIncludePath(b.path("src"));

    // Add C source files from HAL driver and CMSIS device to root_module
    exe.root_module.addCSourceFiles(.{
        .files = &.{
            "deps/cmsis-device-u5/Source/Templates/system_stm32u5xx.c",
            "deps/cmsis-device-u5/Source/Templates/gcc/startup_stm32u575xx.s",
            "deps/stm32u5xx-hal-driver/Src/stm32u5xx_hal.c",
            "deps/stm32u5xx-hal-driver/Src/stm32u5xx_hal_cortex.c",
            "deps/stm32u5xx-hal-driver/Src/stm32u5xx_hal_gpio.c",
            "deps/stm32u5xx-hal-driver/Src/stm32u5xx_hal_rcc.c",
            "deps/stm32u5xx-hal-driver/Src/stm32u5xx_hal_rcc_ex.c",
            "deps/stm32u5xx-hal-driver/Src/stm32u5xx_hal_pwr.c",
            "deps/stm32u5xx-hal-driver/Src/stm32u5xx_hal_pwr_ex.c",
        },
        .flags = &.{
            "-std=c99",
            "-DSTM32U575xx",
            "-DUSE_HAL_DRIVER",
            "-D__ARM_ARCH_PROFILE='M'",
            "-ffunction-sections",
            "-fdata-sections",
        },
    });

    // Set linker script
    exe.setLinkerScript(b.path("deps/cmsis-device-u5/Source/Templates/gcc/linker/STM32U575xx_FLASH.ld"));

    // Install the ELF file to the output directory
    b.installArtifact(exe);
}
