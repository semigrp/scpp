// examples/example3.cpp
#include <iostream>
#include <memory>

class Animal {
public:
    Animal(const std::string& name) : name_(name) {}
    virtual ~Animal() {}

    virtual void speak() const = 0;

protected:
    std::string name_;
};

class Dog : public Animal {
public:
    Dog(const std::string& name) : Animal(name) {}

    void speak() const override {
        std::cout << name_ << " says: Woof!" << std::endl;
    }
};

class Cat : public Animal {
public:
    Cat(const std::string& name) : Animal(name) {}

    void speak() const override {
        std::cout << name_ << " says: Meow!" << std::endl;
    }
};

int main() {
    std::unique_ptr<Animal> dog = std::make_unique<Dog>("Buddy");
    std::unique_ptr<Animal> cat = std::make_unique<Cat>("Whiskers");

    dog->speak();
    cat->speak();

    return 0;
}
