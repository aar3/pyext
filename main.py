import string

import levdist


# https://www.python-course.eu/levenshtein_distance.php
def levenshtein_distance(s: str, t: str, weight_dict={}) -> int:
    nrows = len(s) + 1
    ncols = len(t) + 1

    # alphabet = string.ascii_lowercase
    weights = dict((ch, (1, 1, 1)) for ch in string.ascii_lowercase + string.ascii_uppercase)
    if weight_dict:
        weights.update(weight_dict)

    # dist = [[0 for x in range(ncols)] for x in range(nrows)]
    matrix = [[0] * ncols for _ in range(nrows)]

    for row in range(1, nrows):
        matrix[row][0] = matrix[row - 1][0] + weights[s[row - 1]][0]

    for col in range(1, ncols):
        matrix[0][col] = matrix[0][col - 1] + weights[t[col - 1]][1]

    for col in range(1, ncols):
        for row in range(1, nrows):
            deletes = weights[s[row - 1]][0]
            inserts = weights[t[col - 1]][1]
            subs = max((weights[s[row - 1]][2], weights[t[col - 1]][2]))

            subs = 0 if s[row - 1] == t[col - 1] else subs

            matrix[row][col] = min(
                matrix[row - 1][col] + deletes,
                matrix[row][col - 1] + inserts,
                matrix[row - 1][col - 1] + subs,
            )

    return matrix[row][col]


def test_pythonic_levenshtein_distance(benchmark):
    _ = benchmark.pedantic(
        levenshtein_distance,
        args=("pseudopsychoanalytics", "pseudopsychoanalytics"),
        iterations=10,
        rounds=100,
    )


def test_rust_levenshtein_distance(benchmark):
    _ = benchmark.pedantic(
        levdist.levenshtein_distance,
        args=("pseudopsychoanalytics", "pseudopsychoanalytics"),
        iterations=10,
        rounds=100,
    )


def main() -> int:
    print(levenshtein_distance("racecar", "racecarsgood"))
    return


if __name__ == "__main__":

    main()
