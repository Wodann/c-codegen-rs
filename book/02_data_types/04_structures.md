# 2.4 Structures

A structure is a programmer-defined data type made up of variables of other data types (possibly including other structure types).
• Defining Structures:	  	
• Declaring Structure Variables:	  	
• Accessing Structure Members:	  	
• Bit Fields:	  	
• Size of Structures:	  	

## 2.4.1 Defining Structures

You define a structure using the struct keyword followed by the declarations of the structure’s members, enclosed in braces. You declare each member of a structure just as you would normally declare a variable—using the data type followed by one or more variable names separated by commas, and ending with a semicolon. Then end the structure definition with a semicolon after the closing brace.

You should also include a name for the structure in between the struct keyword and the opening brace. This is optional, but if you leave it out, you can’t refer to that structure data type later on (without a typedef, see The typedef Statement).

Here is an example of defining a simple structure for holding the X and Y coordinates of a point:

struct point
  {
    int x, y;
  };

That defines a structure type named struct point, which contains two members, x and y, both of which are of type int.

Structures (and unions) may contain instances of other structures and unions, but of course not themselves. It is possible for a structure or union type to contain a field which is a pointer to the same type (see Incomplete Types).

## 2.4.2 Declaring Structure Variables

You can declare variables of a structure type when both you initially define the structure and after the definition, provided you gave the structure type a name.
• Declaring Structure Variables at Definition:	  	
• Declaring Structure Variables After Definition:	  	
• Initializing Structure Members:	  	

### 2.4.2.1 Declaring Structure Variables at Definition

You can declare variables of a structure type when you define the structure type by putting the variable names after the closing brace of the structure definition, but before the final semicolon. You can declare more than one such variable by separating the names with commas.

struct point
  {
    int x, y;
  } first_point, second_point;

That example declares two variables of type struct point, first_point and second_point.

### 2.4.2.2 Declaring Structure Variables After Definition

You can declare variables of a structure type after defining the structure by using the struct keyword and the name you gave the structure type, followed by one or more variable names separated by commas.

struct point
  {
    int x, y;
  };
struct point first_point, second_point;

That example declares two variables of type struct point, first_point and second_point.

### 2.4.2.3 Initializing Structure Members

You can initialize the members of a structure type to have certain values when you declare structure variables.

If you do not initialize a structure variable, the effect depends on whether it has static storage (see Storage Class Specifiers) or not. If it is, members with integral types are initialized with 0 and pointer members are initialized to NULL; otherwise, the value of the structure’s members is indeterminate.

One way to initialize a structure is to specify the values in a set of braces and separated by commas. Those values are assigned to the structure members in the same order that the members are declared in the structure in definition.

struct point
  {
    int x, y;
  };
struct point first_point = { 5, 10 };

In that example, the x member of first_point gets the value 5, and the y member gets the value 10.

Another way to initialize the members is to specify the name of the member to initialize. This way, you can initialize the members in any order you like, and even leave some of them uninitialized. There are two methods that you can use. The first method is available in C99 and as a C89 extension in GCC:

struct point first_point = { .y = 10, .x = 5 };

You can also omit the period and use a colon instead of ‘=’, though this is a GNU C extension:

struct point first_point = { y: 10, x: 5 };

You can also initialize the structure variable’s members when you declare the variable during the structure definition:

struct point
  {
    int x, y;
  } first_point = { 5, 10 };

You can also initialize fewer than all of a structure variable’s members:

struct pointy
  {
    int x, y;
    char *p;
  };
struct pointy first_pointy = { 5 };

Here, x is initialized with 5, y is initialized with 0, and p is initialized with NULL. The rule here is that y and p are initialized just as they would be if they were static variables.

Here is another example that initializes a structure’s members which are structure variables themselves:

struct point
  {
    int x, y;
  };

struct rectangle
  {
    struct point top_left, bottom_right;
  };

struct rectangle my_rectangle = { {0, 5}, {10, 0} };

That example defines the rectangle structure to consist of two point structure variables. Then it declares one variable of type struct rectangle and initializes its members. Since its members are structure variables, we used an extra set of braces surrounding the members that belong to the point structure variables. However, those extra braces are not necessary; they just make the code easier to read.

## 2.4.3 Accessing Structure Members

You can access the members of a structure variable using the member access operator. You put the name of the structure variable on the left side of the operator, and the name of the member on the right side.

struct point
  {
    int x, y;
  };

struct point first_point;

first_point.x = 0;
first_point.y = 5;

You can also access the members of a structure variable which is itself a member of a structure variable.

struct rectangle
  {
    struct point top_left, bottom_right;
  };

struct rectangle my_rectangle;

my_rectangle.top_left.x = 0;
my_rectangle.top_left.y = 5;

my_rectangle.bottom_right.x = 10;
my_rectangle.bottom_right.y = 0;

## 2.4.4 Bit Fields

You can create structures with integer members of nonstandard sizes, called bit fields. You do this by specifying an integer (int, char, long int, etc.) member as usual, and inserting a colon and the number of bits that the member should occupy in between the member’s name and the semicolon.

struct card
  {
    unsigned int suit : 2;
    unsigned int face_value : 4;
  };

That example defines a structure type with two bit fields, suit and face_value, which take up 2 bits and 4 bits, respectively. suit can hold values from 0 to 3, and face_value can hold values from 0 to 15. Notice that these bit fields were declared as unsigned int; had they been signed integers, then their ranges would have been from -2 to 1, and from -8 to 7, respectively.

More generally, the range of an unsigned bit field of N bits is from 0 to 2^N - 1, and the range of a signed bit field of N bits is from -(2^N) / 2 to ((2^N) / 2) - 1.

Bit fields can be specified without a name in order to control which actual bits within the containing unit are used. However, the effect of this is not very portable and it is rarely useful. You can also specify a bit field of size 0, which indicates that subsequent bit fields not further bit fields should be packed into the unit containing the previous bit field. This is likewise not generally useful.

You may not take the address of a bit field with the address operator & (see Pointer Operators).

## 2.4.5 Size of Structures

The size of a structure type is equal to the sum of the size of all of its members, possibly including padding to cause the structure type to align to a particular byte boundary. The details vary depending on your computer platform, but it would not be atypical to see structures padded to align on four- or eight-byte boundaries. This is done in order to speed up memory accesses of instances of the structure type.
