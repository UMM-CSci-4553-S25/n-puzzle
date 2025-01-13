# Run the given program n times, printing out the run time for each run
# Usage: run_n_times.sh <program> <n> <args>
# Example: run_n_times.sh ./my_program 10 arg1 arg2
for i in $(seq 1 $2); do
    echo "Run $i"
    time $1 ${@:3}
done
