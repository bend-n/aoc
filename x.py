print(sum(map(lambda line: len(line) - 1 - eval(f"len({line})"), open(0).readlines())))
