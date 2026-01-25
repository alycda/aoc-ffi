#pragma once
#include <algorithm>
#include <vector>
#include <cstdint>

namespace cpp_sorting {

// Sort an array in-place via raw pointer + length
// This avoids std::vector conversion across the FFI boundary
inline void sort_i32_array(int32_t* data, size_t len) {
    std::sort(data, data + len);
}

} // namespace cpp_sorting
