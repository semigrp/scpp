// examples/example2.cpp
#include <iostream>
#include <vector>

int sum_of_elements(const std::vector<int>& numbers) {
    int sum = 0;
    for (int number : numbers) {
        sum += number;
    }
    return sum;
}

int main() {
    std::vector<int> numbers = {1, 2, 3, 4, 5};

    int sum = sum_of_elements(numbers);

    std::cout << "The sum of the elements in the vector is: " << sum << std::endl;

    int *array = new int[5]{1, 2, 3, 4, 5};
    int array_sum = 0;
    for (int i = 0; i < 5; ++i) {
        array_sum += array[i];
    }

    std::cout << "The sum of the elements in the array is: " << array_sum << std::endl;
    
    delete[] array;

    return 0;
}
