# 2.7 Incomplete Types

You can define structures, unions, and enumerations without listing their members (or values, in the case of enumerations). Doing so results in an incomplete type. You can’t declare variables of incomplete types, but you can work with pointers to those types.

struct point;

At some time later in your program you will want to complete the type. You do this by defining it as you usually would:

struct point
  {
    int x, y;
  };

This technique is commonly used to for linked lists:

struct singly_linked_list
  {
    struct singly_linked_list *next;
    int x;
    /* other members here perhaps */
  };
struct singly_linked_list *list_head;
