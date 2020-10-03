production=false

for ((a = 1; a <= $#; a+=2)); do
    b=$(($a + 1))
    case ${!a} in
        -runTest) testToRun=${!b}; ;;
        -production) production=true; ((a--)); ;;
        -target) target=${!b}; ;;
    esac
done


if [ ! -z "$testToRun" ]; then 
    echo "Running test $testToRun" echo;

    echo "- compiling and running"
    cargo run -- -r test/$testToRun/input.txt -o test/$testToRun/output.txt 
    echo;
    
    echo "- comparing output"
    if cmp test/$testToRun/correct-output.txt test/$testToRun/output.txt; then
        echo "  > output matches"
    else 
        echo "  > output does not match"
        diff test/$testToRun/correct-output.txt test/$testToRun/output.txt 
    fi
fi

if $production; then
    if [ ! -z "$target" ]; then 
        cargo build --release --target $target
    else 
        cargo build --release
    fi
fi