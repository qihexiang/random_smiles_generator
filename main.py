import pysws
import re

start = "[P{R;R;Out;L}]"
end = "[P{R;R;In;L}]"
singles = [
    "[H{RIn}]",
    "[C{RIn}]",
    "[OH{RIn}]",
    "[NH2{RIn}]",
    
]
duals = [
    "[c{In}]1cccc[c{Out}]1",
    "[c{In}]1ccc[c{Out}]c1",
    "[c{In}]1cc[c{R}]c[c{Out}]1",
    "[c{In}]1c[c{R}]cc[c{Out}]1",
    "[C{R;In;Out}]", "[C{R;In;Out}]", "[C{R;In;Out}]",
    "[C{R;R;In;Out}]", "[C{R;R;In;Out}]", "[C{R;R;In;Out}]",
]

ligands = []

for _ in range(0,100):
    ligands.append(pysws.rgs(start, [
        (2, 1, duals, "In", "Out", "-"),
        (1, 1, [end], "In", "Out", "-"),
        (-1, 1, singles, "RIn", "R", "-")
    ]))

for _ in range(0, 100):
    print(pysws.rgs("[Fe+2{LIn;LIn;LIn;LIn}]", [(2, 2, ligands, "L", "LIn", "-")]))
