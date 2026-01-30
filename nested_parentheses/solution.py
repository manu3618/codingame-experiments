# https://www.codingame.com/ide/puzzle/level-of-nested-parentheses


def get_groups(s):
    """
    Args:
        s (str)
    Returns:
        [(level, group_start, group_end)]
    """
    final_groups = []
    open_groups = []  # [(level, start)]
    for idx, c in enumerate(s):
        if c == "(":
            if open_groups and min(l for (l, _) in open_groups) <= 1:
                open_groups = [(l + 1, start) for (l, start) in open_groups]
            open_groups.append((1, idx))

        if c == ")" and open_groups:
            # close the correct open group
            g = open_groups.pop()

            # put this group in final groups
            final_groups.append((g[0], g[1], idx))

    assert len(open_groups) == 0
    return final_groups


def get_level(s):
    """Get number of nested level for each index"""
    level = 0
    results = []
    for c in s:
        if c == "(":
            level += 1
        if c == ")":
            level -= 1
        results.append(level)
    assert level == 0
    return results


def get_group_level(s):
    groups = get_groups(s)
    levels = {a for (a, _, _) in groups}
    missing = [a for a in range(1, max(levels)) if a not in levels]
    while missing:
        gap = min(missing)
        groups = [(l, a, b) if l < gap else (l - 1, a, b) for (l, a, b) in groups]
        levels = {a for (a, _, _) in groups}
        missing = [a for a in range(1, max(levels)) if a not in levels]
    return groups


def get_line(n, s):
    """get the nth line"""
    groups = get_group_level(s)
    nested_levels = get_level(s)
    # print("".join(str(a) for a in nested_levels), file=sys.stderr)
    if not groups:
        return
    line = [" "] * len(s)
    if n == 0 or n == 1:
        char = "^" if n == 0 else "|"
        for _, a, b in groups:
            line[a] = char
            line[b] = char
        return "".join(line)
    n = n - 1
    for l, a, b in groups:
        if l > n:
            line[a] = "|"
            line[b] = "|"
        if l == n:
            for idx in range(a, b):
                line[idx] = "-"
            level = min(nested_levels[a:b])
            line[a] = str(level)
            line[b] = str(level)
    return "".join(line)


def print_answer(s):
    if not s:
        return
    print(s)
    groups = get_group_level(s)
    if not groups:
        return
    max_level = max(a for (a, _, _) in groups)
    for l in range(max_level + 2):
        print(get_line(l, s))


print_answer(input())
