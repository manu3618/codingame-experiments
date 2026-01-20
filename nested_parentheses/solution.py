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
            # increase level of each group if needed open group
            if open_groups and min(l for (l, _) in open_groups) == 1:
                open_groups = [(l + 1, start) for (l, start) in open_groups]

            # create new opened group
            open_groups.append((1, idx))

        if c == ")":
            # close the correct open group
            g = open_groups.pop()
            # put this group in final groups
            final_groups.append((g[0], g[1], idx))

    assert len(open_groups) == 0
    return final_groups


def get_line(n, s):
    """get the nth line"""
    groups = get_groups(s)
    if not groups:
        return
    max_level = max(a for (a, _, _) in groups)
    if n > max_level + 1:
        assert False, "unreachable"
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
            line[a] = str(max_level - l + 1)
            line[b] = str(max_level - l + 1)
    return "".join(line)


def print_answer(s):
    print(s)
    groups = get_groups(s)
    if not groups:
        return
    max_level = max(a for (a, _, _) in groups)
    for l in range(max_level + 2):
        print(get_line(l, s))
