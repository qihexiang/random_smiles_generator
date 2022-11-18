import pysws
from openbabel import openbabel

start = "[P{R;R;Out}]"
end = "[P{R;R;In}]"
singles = [
    "[H{RIn}]",
    "[C{RIn}]",
    "[OH{RIn}]",
    "[NH2{RIn}]",

]
duals = [
    "[c{In}]1cc[c{R}]c[c{Out}]1",
    "[c{In}]1c[c{R}]cc[c{Out}]1",
    "[C{R;In;Out}]", "[C{R;In;Out}]", "[C{R;In;Out}]",
    "[C{R;R;In;Out}]", "[C{R;R;In;Out}]", "[C{R;R;In;Out}]",
]

ligands = []

reader = openbabel.OBConversion()
reader.SetInFormat("smi")
canWriter = openbabel.OBConversion()
canWriter.SetOutFormat("can")
ml2Writer = openbabel.OBConversion()
ml2Writer.SetOutFormat("ml2")

for _ in range(0, 100):
    generated = pysws.rgs(start, [
        (2, 1, duals, "In", "Out", "-"),
        (1, 1, [end], "In", "Out", "-"),
        (-1, 1, singles, "RIn", "R", "-")
    ])
    mol = openbabel.OBMol()
    reader.ReadString(mol, generated)
    can = canWriter.WriteString(mol).removesuffix("\t\n")
    print((generated, can))

   

# for _ in range(0, 100):
#     print(pysws.rgs("[Fe+2{LIn;LIn;LIn;LIn}]", [(2, 2, ligands, "L", "LIn", "-")]))
