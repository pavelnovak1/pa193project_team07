#! /bin/bash
treshold=25
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



	
