//! 2024 Day 3: Mull It Over

/// Parse a number (1-3 digits) starting at position `pos` in the byte slice.
/// Returns (parsed_number, new_position) or None if no valid number found.
fn parse_num(input: &[u8], pos: usize) -> Option<(i64, usize)> {
    let start = pos;
    let mut end = pos;
    while end < input.len() && end - start < 3 && input[end].is_ascii_digit() {
        end += 1;
    }
    if end == start {
        return None;
    }
    let s = std::str::from_utf8(&input[start..end]).ok()?;
    Some((s.parse().ok()?, end))
}

/// Try to parse `mul(X,Y)` at position `pos`.
/// Returns (product, new_position) or None.
fn parse_mul(input: &[u8], pos: usize) -> Option<(i64, usize)> {
    let rest = &input[pos..];
    if !rest.starts_with(b"mul(") {
        return None;
    }
    let pos = pos + 4;
    let (x, pos) = parse_num(input, pos)?;
    if input.get(pos) != Some(&b',') {
        return None;
    }
    let pos = pos + 1;
    let (y, pos) = parse_num(input, pos)?;
    if input.get(pos) != Some(&b')') {
        return None;
    }
    Some((x * y, pos + 1))
}

fn part1(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let mut sum = 0;
    let mut i = 0;
    while i < bytes.len() {
        if let Some((product, _next)) = parse_mul(bytes, i) {
            sum += product;
        }
        i += 1;
    }
    sum
}

fn part2(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let mut sum = 0;
    let mut enabled = true;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i..].starts_with(b"do()") {
            enabled = true;
        } else if bytes[i..].starts_with(b"don't()") {
            enabled = false;
        } else if enabled {
            if let Some((product, _next)) = parse_mul(bytes, i) {
                sum += product;
            }
        }
        i += 1;
    }
    sum
}

fn main() {
    println!("2024 Day 3: Mull It Over\n");

    let sample1 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let sample2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()mul(8,5))";

    assert_eq!(part1(sample1), 161);
    assert_eq!(part2(sample2), 48);

    println!("Part 1: {}", part1(sample1));
    println!("Part 2: {}", part2(sample2));
    println!("\n✓ All tests passed!");
}
