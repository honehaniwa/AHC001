./run.sh
rm ./trans_output/*
python devisor.py 
cd ../tools
rm ./out/*.svg
./trans.sh
cd ../AHC001