# 3.3 Incrementing and Decrementing

The increment operator ++ adds 1 to its operand. The operand must be a variable of one of the primitive data types, a pointer, or an enumeration variable. You can apply the increment operator either before or after the operand. Here are some examples:

TODO:
```c
char w = '1';
int x = 5;
char y = 'B';
float z = 5.2;
int *p = &x;

++w;   /* w is now the character ‘2’ (not the value 2). */
x++;   /* x is now 6. */
++y;   /* y is now ‘C’ (on ASCII systems). */
z++;   /* z is now 6.2. */
++p;   /* p is now &x + sizeof(int). */
```

(Note that incrementing a pointer only makes sense if you have reason to believe that the new pointer value will be a valid memory address.)

A prefix increment adds 1 before the operand is evaluated. A postfix increment adds 1 after the operand is evaluated. In the previous examples, changing the position of the operator would make no difference. However, there are cases where it does make a difference:

TODO:
```c
int x = 5;
printf ("%d \n", x++);    /* Print x and then increment it. */
/* x is now equal to 6. */
printf ("%d \n", ++x);    /* Increment x and then print it. */
```

The output of the above example is:

```bash
5
7
```

Likewise, you can subtract 1 from an operand using the decrement operator:

TODO:
```c
int x = 5;

x--; /* x is now 4. */
```

The concepts of prefix and postfix application apply here as with the increment operator. 
