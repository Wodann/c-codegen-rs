# 5.9 Static Functions

You can define a function to be static if you want it to be callable only within the source file where it is defined:

static int
foo (int x)
{
  return x + 42;
}

This is useful if you are building a reusable library of functions and need to include some subroutines that should not be callable by the end user.

Functions which are defined in this way are said to have static linkage. Unfortunately the static keyword has multiple meanings; Storage Class Specifiers. 
