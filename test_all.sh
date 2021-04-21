#! /bin/bash
treshold=30
results=()
worst=()
worst_points=()
sum=0
i=0
for filename in test_dataset/*.txt; do
	file=$(echo $filename | rev | cut -c 5- | rev)
	echo $filename
	cargo run tmp.json $filename 2> /dev/null
	res=$(./test_dataset/output_compare.py $file.json tmp.json)
	echo $res
	if [[ $res -lt $treshold ]]
	then
		worst+=( $filename )
		worst_points+=( $res )
	fi
	sum=$((sum+res))
	results+=( $res )
	i=$((i+1))
done
rm -f tmp.json
echo "=================================="
echo "Total files processed: $i"
echo "Average points per file: $((sum/i))"
echo "=================================="
echo "Worst results ( points < $treshold ):"

len=${#worst[@]}
for (( j=0; j<$len; j++ )); do 
	echo "${worst[$j]}:" 
	echo "${worst_points[$j]}"
done

echo "test_dataset/0939V3b_pdf.txt - 54pts:"
echo "Problems: Biblio (Numbers)"
echo "test_dataset/1019V2b_pdf.txt - 48pts:"
echo "Problems: Biblio (Numbers)"
echo "test_dataset/1022b_pdf.txt - 40pts:"
echo "Problems: Biblio (References missing)"
echo "test_dataset/1040b_pdf.txt - 52pts:"
echo "Problems: Biblio (References missing)"
echo "test_dataset/1059b_pdf.txt - 56pts:"
echo "Problems: Biblio (Numbers)"
echo "test_dataset/1107b_pdf.txt - 33pts:"
echo "Problems: Versions, Biblio (References)"
echo "test_dataset/anssi-cible-cc-2020_72en.txt - 55pts"
echo "Problems: Biblio (References missing)"
echo "test_dataset/NSCIB-CC_0075541-ST.txt - 19pts"
echo "Problems: Title"
echo "test_dataset/NSCIB-CC-0095534-STLite.txt - 53pts"
echo "Problems: Biblio (Number)"
echo "test_dataset/NSCIB-CC-0145426-ST_rev_C-final.txt - 11pts"
echo "Problems: Title (spatny original .json), Biblio (REFERENCED LITERATURE)"
echo "test_dataset/nscib-cc-0229285eac-stv1.2.txt - 48pts"
echo "Problems: Biblio (references)"
echo "test_dataset/nscib-cc-0229286sscdkeygen-stv1.2.txt - 43pts"
echo "Problems: Biblio (references)"
echo "test_dataset/NSCIB-CC-0229287(SSCDkeyImp)-STv1.2.txt - 42pts"
echo "Problems: Biblio (references)"





	 
