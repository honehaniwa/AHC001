for file in `ls ../AHC001/trans_output/*.txt`
do
    OUT=`basename $file .txt`
    cargo run --bin vis ../AHC001/in $file $OUT
done