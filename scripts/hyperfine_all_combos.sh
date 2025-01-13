for algorithm in "a-star" "ida-star"; do # "bfs" "dfs" "id-dfs"
    for heuristic in "taxicab" "num-incorrect"; do
        # 8-puzzle
        # hyperfine -N "target/release/main --algorithm $algorithm --heuristic $heuristic --pieces 7,8,5,3,1,4,6,2 --x-blank 0 --y-blank 2"
        # Wikipedia 15-puzzle
        hyperfine -N "target/release/main --algorithm $algorithm --heuristic $heuristic --pieces 1,2,4,3,5,6,7,8,9,10,11,15,13,14,12 --x-blank 3 --y-blank 3"
    done
done
