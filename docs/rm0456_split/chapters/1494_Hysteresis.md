1500

Comparator (COMP)

RM0456
Figure 319. Window mode
COMPx WINMODE = 0
COMPx WINOUT = 1

COMPx_INP

Input

+
COMPx_INM

Upper threshold

COMPx_VALUE
COMPx

COMPx_OUT

-

COMPy WINMODE = 1
COMPy_INP

COMPy WINOUT = 0
+
COMPy

COMPy_INM

Lower threshold

COMPy_VALUE

COMPy_OUT

MSv42191V1

37.4.5

Hysteresis
The comparator includes a programmable hysteresis to avoid spurious output transitions in
case of noisy signals. The hysteresis can be disabled if it is not needed (for instance when
exiting a low-power mode) to be able to force the hysteresis value using external
components.
Figure 320. Comparator hysteresis
INP

INM
INM - Vhyst

COMP_OUT

MS19984V1

37.4.6

Comparator output-blanking function
The blanking function prevents the current regulation to trip upon short current spikes at the
beginning of the PWM period (typically the recovery current in power switches anti parallel
diodes). This blanking function consists of a selection of a blanking window that is a timer
output compare signal. The selection is done by the software (refer to the comparator
register description for possible blanking signals).
The complementary of the blanking signal is ANDed with the comparator output to provide
the wanted comparator output (see the example in the figure below).

<!-- pagebreak -->

