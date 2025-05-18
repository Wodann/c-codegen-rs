# 5.4 Function Parameters

Function parameters can be any expression—a literal value, a value stored in variable, an address in memory, or a more complex expression built by combining these.

Within the function body, the parameter is a local copy of the value passed into the function; you cannot change the value passed in by changing the local copy.

int x = 23;
foo (x);
…
/* Definition for function foo. */
int foo (int a)
{
  a = 2 * a;
  return a;
}

In that example, even though the parameter a is modified in the function ‘foo’, the variable x that is passed to the function does not change. If you wish to use the function to change the original value of x, then you would have to incorporate the function call into an assignment statement:

x = foo (x);

If the value that you pass to a function is a memory address (that is, a pointer), then you can access (and change) the data stored at the memory address. This achieves an effect similar to pass-by-reference in other languages, but is not the same: the memory address is simply a value, just like any other value, and cannot itself be changed. The difference between passing a pointer and passing an integer lies in what you can do using the value within the function.

Here is an example of calling a function with a pointer parameter:

void
foo (int *x)
{
  *x = *x + 42;
}
…
int a = 15;
foo (&a);

The formal parameter for the function is of type pointer-to-int, and we call the function by passing it the address of a variable of type int. By dereferencing the pointer within the function body, we can both see and change the value stored in the address. The above changes the value of a to ‘57’.

Even if you don’t want to change the value stored in the address, passing the address of a variable rather than the variable itself can be useful if the variable type is large and you need to conserve memory space or limit the performance impact of parameter copying. For example:

struct foo
{
  int x;
  float y;
  double z;
};

void bar (const struct foo *a);

In this case, unless you are working on a computer with very large memory addresses, it will take less memory to pass a pointer to the structure than to pass an instance of the structure.

One type of parameter that is always passed as a pointer is any sort of array:

void foo (int a[]);
…
int x[100];
foo (x);

In this example, calling the function foo with the parameter a does not copy the entire array into a new local parameter within foo; rather, it passes x as a pointer to the first element in x. Be careful, though: within the function, you cannot use sizeof to determine the size of the array x—sizeof instead tells you the size of the pointer x. Indeed, the above code is equivalent to:

void foo (int *a);
…
int x[100];
foo (x);

Explicitly specifying the length of the array in the parameter declaration will not help. If you really need to pass an array by value, you can wrap it in a struct, though doing this will rarely be useful (passing a const-qualified pointer is normally sufficient to indicate that the caller should not modify the array). 
