import pysws
import re
from pandas import DataFrame;

start = "[P{R;R;Out;L}]"
end = "[P{R;R;In;L}]"
singles = [
    "[H{RIn}]",
    "[C{RIn}]",
    "[OH{RIn}]",
    "[NH2{RIn}]",
    "[c{RIn}]1ccccc1",
]
duals = [
    "[c{In}]1cccc[c{Out}]1",
    "[c{In}]1ccc[c{Out}]c1",
    "[c{In}]1cc[c{Out}]cc1",
    "[c{In}]1cc[c{R}]c[c{Out}]1",
    "[c{In}]1c[c{R}]cc[c{Out}]1",
    "[c{In}]1cc[c{R}][c{Out}]c1",
    "[C{R;In;Out}]", "[C{R;In;Out}]", "[C{R;In;Out}]",
    "[C{R;R;In;Out}]", "[C{R;R;In;Out}]", "[C{R;R;In;Out}]",
]
for _ in range(0,10000):
    sws = pysws.rgs(start, [
        (2, 1, duals, "In", "Out", "-"),
        (1, 1, [end], "In", "Out", "-"),
        (-1, 1, singles, "RIn", "R", "-")
    ])
    smiles = re.sub("\{.*?\}", "", sws)
    [first, second] = pysws.ligand_index(sws)
    print(smiles, first, second)