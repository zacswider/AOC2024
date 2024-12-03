import time

def open_input(p: str) -> list[str]:
    with open(p, 'r') as f:
        lines = f.readlines()
    return lines

def parse_muls_in_chunk(line: str) -> int:
    muls = []
    mul_starts = line.split('mul(')
    for mul_start in mul_starts:
        potentially_valid_mul = mul_start.split(')')[0]
        nums = potentially_valid_mul.split(',')
        if not len(nums) == 2:
            continue
        try:
            nums = list(map(int, nums))
        except ValueError:
            continue
        muls.append(nums[0] * nums[1])
    return sum(muls)

def part1(lines: list[str]) -> None:
    muls = []
    for line in lines:
        muls.append(parse_muls_in_chunk(line))
    print(f'part1: {sum(muls)}')

def part2(lines: list[str]) -> None:
    muls = []
    line = ""
    for _line in lines:
        line += _line
    do_not_splits = line.split("don't()")
    valid = do_not_splits[0]
    do_not_splits = do_not_splits[1:]
    muls.append(parse_muls_in_chunk(valid))
    for do_not_split in do_not_splits:
        valid = ""
        for chunk in do_not_split.split('do()')[1:]:
            valid += chunk
        muls.append(parse_muls_in_chunk(valid))
    print(f'part2: {sum(muls)}')

def main() -> None:
    p = '/Users/zacswider/code/AOC2024/day3/day3.txt'
    lines = open_input(p)
    # lines = ['xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))']
    part1(lines)
    # lines = ["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", ""]
    part2(lines)

if __name__ == '__main__':
    start = time.time()
    main()
    print(f'time elapsed: {(time.time() - start)*1000:.2f}ms')