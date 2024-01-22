if [ $# -lt 1 ]; then
    echo "Usage: $0 [puzzle] [main_picture]"
    exit 1
fi


echo "Running Cargo with picture $1"
cargo run $1

echo "Running Python"
python3 put_together.py

rm picture.txt