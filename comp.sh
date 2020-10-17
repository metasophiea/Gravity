compile=false
production=false

for ((a = 1; a <= $#; a+=2)); do
    b=$(($a + 1))
    case ${!a} in
        -runTest) testToRun=${!b}; ;;
        -compile) compile=true; ((a--)); ;;
        -production) production=true; ((a--)); ;;
        -target) target=${!b}; ;;
        --help) 
            echo "-runTest : compile the debug version and run this specified test (eg. -runTest 1)"
            echo "-compile : declare that we are to compile"
            echo "-production : declare that we are to compile the optimised release version"
            echo "-target : select which target to use"
            echo "-list : list out the (potential) production compilation targets"
            echo "-listCompiled : list out the compiled targets"
            exit;
        ;;
        -list) 
            rustc --print target-list
            exit;
        ;;
        -listCompiled) 
            cat availableVersions
            exit;
        ;;
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

if $compile; then
    if $production; then
        if [ -z "$target" ]; then 
            target=$(rustup show | grep Default | cut -c15-)
        fi

        echo "building for target \"$target\""
        cargo build --release --target $target
    else 
        cargo build
    fi

    ls target | grep -v 'debug\|release' > availableVersions
fi