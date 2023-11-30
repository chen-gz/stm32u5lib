## Introduction


This is a HAL library for stm32. The code are writing for u5 and it should be
easy to extend to other stm32 chips.


## Design

This library use stateless design. That means you can call any function at any
time without worrying about the state of the peripheral. The library will use
the state based on the hardware to determine what to do next. If the function is
not applicable in the current state, it will return an error code.

There are some function can not write as stateless design. In that case, we will
have document and put warning in the function description.

