# 5.2 Function Definitions

You write a function definition to specify what a function actually does. A function definition consists of information regarding the function’s name, return type, and types and names of parameters, along with the body of the function. The function body is a series of statements enclosed in braces; in fact it is simply a block (see Blocks).

Here is the general form of a function definition:

return-type
function-name (parameter-list)
{
  function-body
}

return-type and function-name are the same as what you use in the function declaration (see Function Declarations).

parameter-list is the same as the parameter list used in the function declaration (see Function Declarations), except you must include names for the parameters in a function definition.

Here is an simple example of a function definition—it takes two integers as its parameters and returns the sum of them as its return value:

int
add_values (int x, int y)
{
  return x + y;
}

For compatibility with the original design of C, you can also specify the type of the function parameters after the closing parenthesis of the parameter list, like this:

int
add_values (x, y)
    int x, int y;
{
  return x + y;
}

However, we strongly discourage this style of coding; it can cause subtle problems with type casting, among other problems. 
