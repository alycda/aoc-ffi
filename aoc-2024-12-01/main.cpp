// 2024 Day 1: Historian Hysteria
// Comparison of C qsort vs C++ std::sort

#include <iostream>
#include <vector>
#include <string>
#include <sstream>
#include <algorithm>
#include <chrono>
#include <cstdlib>
#include <cmath>

// ============================================================================
// C qsort approach
// ============================================================================

// Comparison function for C's qsort (ascending order)
// Must return: negative if a < b, zero if a == b, positive if a > b
int compare_i32(const void* a, const void* b) {
    int arg1 = *(const int*)a;
    int arg2 = *(const int*)b;
    return arg1 - arg2;
}

// Sort using C's qsort
void sort_with_qsort(std::vector<int>& vec) {
    qsort(vec.data(), vec.size(), sizeof(int), compare_i32);
}

// ============================================================================
// C++ std::sort approach
// ============================================================================

// Sort using C++'s std::sort (uses introsort - hybrid algorithm)
void sort_with_cpp_sort(std::vector<int>& vec) {
    std::sort(vec.begin(), vec.end());
}

// ============================================================================
// AOC Solution
// ============================================================================

// Parse input: transpose 2 columns of numbers
std::pair<std::vector<int>, std::vector<int>> unzip(const std::string& input) {
    std::vector<int> left, right;
    std::istringstream stream(input);
    std::string line;

    while (std::getline(stream, line)) {
        std::istringstream line_stream(line);
        int l, r;
        if (line_stream >> l >> r) {
            left.push_back(l);
            right.push_back(r);
        }
    }

    return {left, right};
}

// Process using qsort
int process_qsort(const std::string& input) {
    auto [left, right] = unzip(input);

    sort_with_qsort(left);
    sort_with_qsort(right);

    int sum = 0;
    for (size_t i = 0; i < left.size(); ++i) {
        sum += std::abs(left[i] - right[i]);
    }

    return sum;
}

// Process using std::sort
int process_cpp_sort(const std::string& input) {
    auto [left, right] = unzip(input);

    sort_with_cpp_sort(left);
    sort_with_cpp_sort(right);

    int sum = 0;
    for (size_t i = 0; i < left.size(); ++i) {
        sum += std::abs(left[i] - right[i]);
    }

    return sum;
}

// ============================================================================
// Performance Benchmarking
// ============================================================================

template<typename Func>
double benchmark(Func func, const std::string& input, int iterations = 1000) {
    auto start = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < iterations; ++i) {
        func(input);
    }

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;

    return elapsed.count() / iterations;
}

// Generate large test data
std::string generate_large_input(int size) {
    std::ostringstream oss;
    for (int i = 0; i < size; ++i) {
        oss << (rand() % 10000) << "   " << (rand() % 10000) << "\n";
    }
    return oss.str();
}

// ============================================================================
// Main
// ============================================================================

int main() {
    std::cout << "2024 Day 1: C qsort vs C++ std::sort\n";
    std::cout << "=====================================\n\n";

    // Test examples
    std::cout << "Running correctness tests...\n";
    int result1 = process_qsort("3 7");
    std::cout << "  qsort:     3 7 -> " << result1 << " (expected: 4)\n";

    int result2 = process_cpp_sort("3 7");
    std::cout << "  std::sort: 3 7 -> " << result2 << " (expected: 4)\n";

    // Example problem
    std::string example = R"(3   4
4   3
2   5
1   3
3   9
3   3)";

    int part1_qsort = process_qsort(example);
    int part1_cpp = process_cpp_sort(example);
    std::cout << "\nPart 1 (qsort):     " << part1_qsort << "\n";
    std::cout << "Part 1 (std::sort): " << part1_cpp << "\n";

    // Performance comparison
    std::cout << "\n=====================================\n";
    std::cout << "Performance Comparison\n";
    std::cout << "=====================================\n";

    // Test with different sizes
    std::vector<int> sizes = {100, 1000, 10000, 100000};

    for (int size : sizes) {
        std::string large_input = generate_large_input(size);

        double qsort_time = benchmark(process_qsort, large_input, 100);
        double sort_time = benchmark(process_cpp_sort, large_input, 100);
        double speedup = qsort_time / sort_time;

        std::cout << "\nSize: " << size << " elements\n";
        std::cout << "  C qsort():        " << (qsort_time * 1000) << " ms\n";
        std::cout << "  C++ std::sort():  " << (sort_time * 1000) << " ms\n";
        std::cout << "  Speedup:          " << speedup << "x faster\n";
    }

    std::cout << "\n=====================================\n";
    std::cout << "Key Differences:\n";
    std::cout << "  - qsort uses function pointers (runtime overhead)\n";
    std::cout << "  - std::sort uses templates (compile-time optimization)\n";
    std::cout << "  - std::sort uses introsort (quicksort + heapsort + insertion sort)\n";
    std::cout << "  - std::sort is O(N log N) guaranteed in C++11+\n";
    std::cout << "=====================================\n";

    return 0;
}
