# Different ways of initializing in C++

``` C++
/* 1*/ int i = 1; // Direct assignment (copy initialization)
/* 2*/ int i(2); // Constructor-style initialization (direct initialization)
/* 3*/ int i{3}; // Uniform initialization (brace-initialization)
/* 4*/ int i = {4}; // Copy initialization with braces
/* 5*/ int i; i = 5; // Assignment after declaration
/* 6*/ int i = j = 6; // Chained assignment
/* 7*/ c_tor(int i = 7) : _i(i) {}; // Constructor with member initializer list (default parameter)
/* 8*/ c_tor(int i = 8) : _i{i} {}; // Member initializer list (brace-initialization)
/* 9*/ struct S { int i } s; s.i = 9; // Struct member initialization
/*10*/ s = {10}; //  Aggregate initialization using braces
/*11*/ s = S{11}; //  Using a temporary instance for initialization
/*12*/ s = { .i = 12 };  // Designated initializer (since C++20)
/*13*/ std::memset(i, 13, sizeof(i)); //  Memory manipulation (memset)
/*14*/ const int i = boolean ? 14 : 0; //  Conditional operator for const initialization
/*15*/ static int i = 15; //  Static variable initialization
/*16*/ cin >> i; //  Input from standard input stream
/*17*/ std::string i{'a', 'b', 'c'}; //  String initialization using brace-enclosed characters
/*18*/ char a[3] = {'a', 'b'};  // Character array with implicit null terminator
/*19*/ std::string j = std::move(i);  // Move initialization (since C++11)
/*20*/ volatile int i = 1; // Initialization with volatile
```
