cargo build --release --offline --quiet
for file in `ls ../tools/in/*.txt`
do
    OUT=`basename $file`
    RUST_BACKTRACE=1 ./target/release/ahc001 < $file > ./outfiles/$OUT
    if [ $? -gt 0 ]; then
        echo $OUT
        break
    fi
done