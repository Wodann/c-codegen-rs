# 2.5 Arrays

An array is a data structure that lets you store one or more elements consecutively in memory. In C, array elements are indexed beginning at position zero, not one.
• Declaring Arrays:	  	
• Initializing Arrays:	  	
• Accessing Array Elements:	  	
• Multidimensional Arrays:	  	
• Arrays as Strings:	  	
• Arrays of Unions:	  	
• Arrays of Structures:	  	

## 2.5.1 Declaring Arrays

You declare an array by specifying the data type for its elements, its name, and the number of elements it can store. Here is an example that declares an array that can store ten integers:

int my_array[10];

For standard C code, the number of elements in an array must be positive.

## 2.5.2 Initializing Arrays

You can initialize the elements in an array when you declare it by listing the initializing values, separated by commas, in a set of braces. Here is an example:

int my_array[5] = { 0, 1, 2, 3, 4 };

You don’t have to explicitly initialize all of the array elements. For example, this code initializes the first three elements as specified, and then initializes the last two elements to a default value of zero:

int my_array[5] = { 0, 1, 2 };

When using either ISO C99, or C89 with GNU extensions, you can initialize array elements out of order, by specifying which array indices to initialize. To do this, include the array index in brackets, and optionally the assignment operator, before the value. Here is an example:

int my_array[5] = { [2] 5, [4] 9 };

Or, using the assignment operator:

int my_array[5] = { [2] = 5, [4] = 9 };

Both of those examples are equivalent to:

int my_array[5] = { 0, 0, 5, 0, 9 };

When using GNU extensions, you can initialize a range of elements to the same value, by specifying the first and last indices, in the form [first] ... [last] . Here is an example:

int new_array[100] = { [0 ... 9] = 1, [10 ... 98] = 2, 3 };

That initializes elements 0 through 9 to 1, elements 10 through 98 to 2, and element 99 to 3. (You also could explicitly write [99] = 3.) Also, notice that you must have spaces on both sides of the ‘...’.

If you initialize every element of an array, then you do not have to specify its size; its size is determined by the number of elements you initialize. Here is an example:

int my_array[] = { 0, 1, 2, 3, 4 };

Although this does not explicitly state that the array has five elements using my_array[5], it initializes five elements, so that is how many it has.

Alternately, if you specify which elements to initialize, then the size of the array is equal to the highest element number initialized, plus one. For example:

int my_array[] = { 0, 1, 2, [99] = 99 };

In that example, only four elements are initialized, but the last one initialized is element number 99, so there are 100 elements.

## 2.5.3 Accessing Array Elements

You can access the elements of an array by specifying the array name, followed by the element index, enclosed in brackets. Remember that the array elements are numbered starting with zero. Here is an example:

my_array[0] = 5;

That assigns the value 5 to the first element in the array, at position zero. You can treat individual array elements like variables of whatever data type the array is made up of. For example, if you have an array made of a structure data type, you can access the structure elements like this:

struct point
{
  int x, y;
};
struct point point_array[2] = { {4, 5}, {8, 9} };
point_array[0].x = 3;

## 2.5.4 Multidimensional Arrays

You can make multidimensional arrays, or “arrays of arrays”. You do this by adding an extra set of brackets and array lengths for every additional dimension you want your array to have. For example, here is a declaration for a two-dimensional array that holds five elements in each dimension (a two-element array consisting of five-element arrays):

int two_dimensions[2][5] = { {1, 2, 3, 4, 5}, {6, 7, 8, 9, 10} };

Multidimensional array elements are accessed by specifying the desired index of both dimensions:

two_dimensions[1][3] = 12;

In our example, two_dimensions[0] is itself an array. The element two_dimensions[0][2] is followed by two_dimensions[0][3], not by two_dimensions[1][2].

## 2.5.5 Arrays as Strings

You can use an array of characters to hold a string (see String Constants). The array may be built of either signed or unsigned characters.

When you declare the array, you can specify the number of elements it will have. That number will be the maximum number of characters that should be in the string, including the null character used to end the string. If you choose this option, then you do not have to initialize the array when you declare it. Alternately, you can simply initialize the array to a value, and its size will then be exactly large enough to hold whatever string you used to initialize it.

There are two different ways to initialize the array. You can specify of comma-delimited list of characters enclosed in braces, or you can specify a string literal enclosed in double quotation marks.

Here are some examples:

char blue[26];
char yellow[26] = {'y', 'e', 'l', 'l', 'o', 'w', '\0'};
char orange[26] = "orange";
char gray[] = {'g', 'r', 'a', 'y', '\0'};
char salmon[] = "salmon";

In each of these cases, the null character \0 is included at the end of the string, even when not explicitly stated. (Note that if you initialize a string using an array of individual characters, then the null character is not guaranteed to be present. It might be, but such an occurrence would be one of chance, and should not be relied upon.)

After initialization, you cannot assign a new string literal to an array using the assignment operator. For example, this will not work:

char lemon[26] = "custard";
lemon = "steak sauce";      /* Fails! */

However, there are functions in the GNU C library that perform operations (including copy) on string arrays. You can also change one character at a time, by accessing individual string elements as you would any other array:

char name[] = "bob";
name[0] = 'r';

It is possible for you to explicitly state the number of elements in the array, and then initialize it using a string that has more characters than there are elements in the array. This is not a good thing. The larger string will not override the previously specified size of the array, and you will get a compile-time warning. Since the original array size remains, any part of the string that exceeds that original size is being written to a memory location that was not allocated for it.

## 2.5.6 Arrays of Unions

You can create an array of a union type just as you can an array of a primitive data type.

union numbers
  {
    int i;
    float f;
  };
union numbers number_array [3];

That example creates a 3-element array of union numbers variables called number_array. You can also initialize the first members of the elements of a number array:

union numbers number_array [3] = { {3}, {4}, {5} };

The additional inner grouping braces are optional.

After initialization, you can still access the union members in the array using the member access operator. You put the array name and element number (enclosed in brackets) to the left of the operator, and the member name to the right.

union numbers number_array [3];
number_array[0].i = 2;

## 2.5.7 Arrays of Structures

You can create an array of a structure type just as you can an array of a primitive data type.

struct point
  {
    int x, y;
  };
struct point point_array [3];

That example creates a 3-element array of struct point variables called point_array. You can also initialize the elements of a structure array:

struct point point_array [3] = { {2, 3}, {4, 5}, {6, 7} };

As with initializing structures which contain structure members, the additional inner grouping braces are optional. But, if you use the additional braces, then you can partially initialize some of the structures in the array, and fully initialize others:

struct point point_array [3] = { {2}, {4, 5}, {6, 7} };

In that example, the first element of the array has only its x member initialized. Because of the grouping braces, the value 4 is assigned to the x member of the second array element, not to the y member of the first element, as would be the case without the grouping braces.

After initialization, you can still access the structure members in the array using the member access operator. You put the array name and element number (enclosed in brackets) to the left of the operator, and the member name to the right.

struct point point_array [3];
point_array[0].x = 2;
point_array[0].y = 3;
