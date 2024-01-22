if [ $# -lt 2 ]; then
    echo "Usage: $0 [puzzle] [main_picture]"
    exit 1
fi


echo "Running Cargo $1 with $2"
cargo run $1 $2

echo "Running Python"
python3 put_together.py

rm picture.txt