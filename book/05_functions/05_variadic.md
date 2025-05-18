# 5.5 Variable Length Parameter Lists

You can write a function that takes a variable number of arguments; these are called variadic functions. To do this, the function needs to have at least one parameter of a known data type, but the remaining parameters are optional, and can vary in both quantity and data type.

You list the initial parameters as normal, but then after that, use an ellipsis: ‘...’. Here is an example function prototype:

int add_multiple_values (int number, ...);

To work with the optional parameters in the function definition, you need to use macros that are defined in the library header file ‘<stdarg.h>’, so you must #include that file. For a detailed description of these macros, see The GNU C Library manual’s section on variadic functions.

Here is an example:

int
add_multiple_values (int number, ...)
{
  int counter, total = 0;
  
  /* Declare a variable of type ‘va_list’. */
  va_list parameters;

  /* Call the ‘va_start’ function. */
  va_start (parameters, number);

  for (counter = 0; counter < number; counter++)
    {
      /* Get the values of the optional parameters. */
      total += va_arg (parameters, int);
    }

  /* End use of the ‘parameters’ variable. */
  va_end (parameters);

  return total;
}

To use optional parameters, you need to have a way to know how many there are. This can vary, so it can’t be hard-coded, but if you don’t know how many optional parameters you have, then you could have difficulty knowing when to stop using the ‘va_arg’ function. In the above example, the first parameter to the ‘add_multiple_values’ function, ‘number’, is the number of optional parameters actually passed. So, we might call the function like this:

sum = add_multiple_values (3, 12, 34, 190);

The first parameter indicates how many optional parameters follow it.

Also, note that you don’t actually need to use ‘va_end’ function. In fact, with GCC it doesn’t do anything at all. However, you might want to include it to maximize compatibility with other compilers.

See Variadic Functions in The GNU C Library Reference Manual. 
