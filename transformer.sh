BABEL_LIBDIR=/usr/lib64/openbabel3/

i=1
generated=$(python main.py)
deduplicated=$(obabel -:"${generated}" -ocan | sort | uniq)

echo $deduplicated | nl > ./mol2/all.smi

for line in $deduplicated
do
echo $i
obabel -:"${line}" -oml2 --gen3d -O ./mol2/$i.mol2
((i=i+1))
done