# 2.8 Type Qualifiers

There are two type qualifiers that you can prepend to your variable declarations which change how the variables may be accessed: const and volatile.

const causes the variable to be read-only; after initialization, its value may not be changed.

const float pi = 3.14159f;

In addition to helping to prevent accidental value changes, declaring variables with const can aid the compiler in code optimization.

volatile tells the compiler that the variable is explicitly changeable, and seemingly useless accesses of the variable (for instance, via pointers) should not be optimized away. You might use volatile variables to store data that is updated via callback functions or signal handlers. Sequence Points and Signal Delivery.

volatile float currentTemperature = 40.0;
