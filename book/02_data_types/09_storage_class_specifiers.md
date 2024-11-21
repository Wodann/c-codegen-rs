# 2.9 Storage Class Specifiers

There are four storage class specifiers that you can prepend to your variable declarations which change how the variables are stored in memory: auto, extern, register, and static.

You use auto for variables which are local to a function, and whose values should be discarded upon return from the function in which they are declared. This is the default behavior for variables declared within functions.

void
foo (int value)
{
  auto int x = value;
  …
  return;
}

register is nearly identical in purpose to auto, except that it also suggests to the compiler that the variable will be heavily used, and, if possible, should be stored in a register. You cannot use the address-of operator to obtain the address of a variable declared with register. This means that you cannot refer to the elements of an array declared with storage class register. In fact the only thing you can do with such an array is measure its size with sizeof. GCC normally makes good choices about which values to hold in registers, and so register is not often used.

static is essentially the opposite of auto: when applied to variables within a function or block, these variables will retain their value even when the function or block is finished. This is known as static storage duration.

int
sum (int x)
{
  static int sumSoFar = 0;
  sumSoFar = sumSoFar + x;
  return sumSoFar;
}

You can also declare variables (or functions) at the top level (that is, not inside a function) to be static; such variables are visible (global) to the current source file (but not other source files). This gives an unfortunate double meaning to static; this second meaning is known as static linkage. Two functions or variables having static linkage in separate files are entirely separate; neither is visible outside the file in which it is declared.

Uninitialized variables that are declared as extern are given default values of 0, 0.0, or NULL, depending on the type. Uninitialized variables that are declared as auto or register (including the default usage of auto) are left uninitialized, and hence should not be assumed to hold any particular value.

extern is useful for declaring variables that you want to be visible to all source files that are linked into your project. You cannot initialize a variable in an extern declaration, as no space is actually allocated during the declaration. You must make both an extern declaration (typically in a header file that is included by the other source files which need to access the variable) and a non-extern declaration which is where space is actually allocated to store the variable. The extern declaration may be repeated multiple times.

extern int numberOfClients;

…

int numberOfClients = 0;

See Program Structure and Scope, for related information. 
