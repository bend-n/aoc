def sm(j):
    match j:
        case dict():
            if "red" in j.values():
                return 0
            return sum(map(sm, j.values()))
        case int():
            return j
        case list():
            return sum(map(sm, j))
        case str():
            return 0


print(sm(__import__("json").loads(input())))
