import pysws

start = "[P{R;R;Out}]"
end = "[P{R;R;In}]"
singles = [
    "[H{RIn}]",
    "[C{RIn}]",
    "[OH{RIn}]",
    # "[NH2{RIn}]",
    "[c{RIn}]1ccccc1",
]
duals = [
    "[c{In}]1cccc[c{Out}]1",
    "[c{In}]1ccc[c{Out}]c1",
    "[c{In}]1cc[c{Out}]cc1",
    "[c{In}]1cc[c{R}]c[c{Out}]1",
    "[c{In}]1c[c{R}]cc[c{Out}]1",
    "[c{In}]1cc[c{R}][c{Out}]c1",
    "[C{R;In;Out}]","[C{R;In;Out}]","[C{R;In;Out}]",
    "[C{R;R;In;Out}]","[C{R;R;In;Out}]","[C{R;R;In;Out}]",
]

for _ in range(0, 2000):
    print(pysws.rgs(start, end, duals, singles, 3))

# for _ in range(0, 100):
#     print(pysws.rgs(start, end, duals, singles, 2))

# for _ in range(0, 100):
#     print(pysws.rgs(start, end, duals, singles, 1))
