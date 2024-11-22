# 5.1 Function Declarations

You write a function declaration to specify the name of a function, a list of parameters, and the function’s return type. A function declaration ends with a semicolon. Here is the general form:

return-type function-name (parameter-list);

return-type indicates the data type of the value returned by the function. You can declare a function that doesn’t return anything by using the return type void.

function-name can be any valid identifier (see Identifiers).

parameter-list consists of zero or more parameters, separated by commas. A typical parameter consists of a data type and an optional name for the parameter. You can also declare a function that has a variable number of parameters (see Variable Length Parameter Lists), or no parameters using void. Leaving out parameter-list entirely also indicates no parameters, but it is better to specify it explicitly with void.

Here is an example of a function declaration with two parameters:

int foo (int, double);

If you include a name for a parameter, the name immediately follows the data type, like this:

int foo (int x, double y);

The parameter names can be any identifier (see Identifiers), and if you have more than one parameter, you can’t use the same name more than once within a single declaration. The parameter names in the declaration need not match the names in the definition.

You should write the function declaration above the first use of the function. You can put it in a header file and use the #include directive to include that function declaration in any source code files that use the function. 
