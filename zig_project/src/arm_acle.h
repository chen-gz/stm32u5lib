// Dummy arm_acle.h for freestanding target
#ifndef _DUMMY_ARM_ACLE_H_
#define _DUMMY_ARM_ACLE_H_

#include <stdint.h>

static inline void __wfi(void) { __asm__ volatile("wfi"); }
static inline void __wfe(void) { __asm__ volatile("wfe"); }
static inline void __sev(void) { __asm__ volatile("sev"); }
static inline uint32_t __rbit(uint32_t val) {
    uint32_t result;
    __asm__ volatile("rbit %0, %1" : "=r"(result) : "r"(val));
    return result;
}

#endif // _DUMMY_ARM_ACLE_H_
