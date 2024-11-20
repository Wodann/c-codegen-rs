# 3.10 Pointer Operators

You can use the address operator& to obtain the memory address of an object.

    int x = 5;
int *pointer_to_x = &x;

It is not necessary to use this operator to obtain the address of a function, although you can :

    extern int foo(void);
int (*fp1)(void) = foo;  /* fp1 points to foo */
int (*fp2)(void) = &foo; /* fp2 also points to foo */

Function pointers and data pointers are not compatible, in the sense that you cannot expect to store the address of a function into a data pointer, and then copy that into a function pointer and call it successfully.It might work on some systems, but it’s not a portable technique.

Given a memory address stored in a pointer, you can use the indirection operator* to obtain the value stored at the address.(This is called dereferencing the pointer.)

int x = 5;
int y;
int *ptr;

ptr = &x; /* ptr now holds the address of x. */

y = *ptr; /* y gets the value stored at the address
             stored in ptr. */

Avoid using dereferencing pointers that have not been initialized to a known memory location.
