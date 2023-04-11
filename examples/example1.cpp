// examples/example1.cpp
#include <iostream>

int main() {
    int a = 42;
    int b = 2;

    int c = a + b;

    std::cout << "The sum of a and b is: " << c << std::endl;

    int *d = new int;
    *d = a * b;

    std::cout << "The product of a and b is: " << *d << std::endl;
    
    delete d;

    return 0;
}
