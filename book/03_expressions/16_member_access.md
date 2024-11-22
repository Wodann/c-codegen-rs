# 3.16 Member Access Expressions

You can use the member access operator . to access the members of a structure or union variable. You put the name of the structure variable on the left side of the operator, and the name of the member on the right side.

struct point
{
  int x, y;
};

struct point first_point;

first_point.x = 0;
first_point.y = 5;

You can also access the members of a structure or union variable via a pointer by using the indirect member access operator ->. x->y is equivalent to (*x).y.

struct fish
  {
    int length, weight;
  };

struct fish salmon;

struct fish *fish_pointer = &salmon;

fish_pointer->length = 3;
fish_pointer->weight = 9;

See Pointers. 
