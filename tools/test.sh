sum=0
for file in `ls ../AHC001/outfiles/*.txt`
do
    OUT=`basename $file`
    POINT=`cargo run --bin vis ./in/$OUT $file`
    sum=$(expr $sum + $POINT)
    if [ $? -gt 0 ]; then
        echo $file
        break
    fi
done
echo 合計: $sum
echo 平均: $(expr $sum / 50)