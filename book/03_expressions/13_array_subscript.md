# 3.13 Array Subscripts

You can access array elements by specifying the name of the array, and the array subscript (or index, or element number) enclosed in brackets. Here is an example, supposing an integer array called my_array:

my_array[0] = 5;

The array subscript expression A[i] is defined as being identical to the expression (*((A)+(i))). This means that many uses of an array name are equivalent to a pointer expression. It also means that you cannot subscript an array having the register storage class.
