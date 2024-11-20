# Global Variables in C

Global variables are variables declared at file scope (outside any function). Understanding the distinction between declaration and definition, as well as initialization rules, is crucial for correct code generation.

## Declaration vs Definition

### Declaration
A declaration introduces an identifier and its type without allocating storage:
```c
extern int counter;        // declares counter without defining it
extern float pi;          // declares pi without defining it
```

### Definition
A definition declares an identifier and also allocates storage:
```c
int counter = 0;          // defines and initializes counter
float pi = 3.14159f;      // defines and initializes pi
int uninitialized_var;    // defines with implicit initialization to 0
```

## Storage Classes

### External Linkage (default)
Variables with external linkage are accessible from other translation units:
```c
int global_var = 42;      // external linkage by default
extern int global_var;    // declaration for use in other files
```

### Static (Internal Linkage)
Static variables are only accessible within their translation unit:
```c
static int file_var = 10; // internal linkage
```

## Initialization

### Rules
1. Must use constant expressions for initializers
2. Uninitialized globals are automatically zero-initialized
3. Initialization happens before program startup

```c
int count = 42;           // OK: constant expression
const int max = 100;      // OK: constant expression
int array[3] = {1,2,3};   // OK: constant expressions

// Not allowed - non-constant expressions:
int x = rand();           // Error: function call not constant
int y = count + 1;        // Error: involves non-constant variable
```

### Zero Initialization
```c
int zero;                 // automatically initialized to 0
int zeros[100];          // all elements initialized to 0
char empty[10];          // all elements initialized to '\0'
```
