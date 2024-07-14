# 1BRC

1️⃣🐝🏎️ [The One Billion Row Challenge](https://github.com/gunnarmorling/1brc) -- A fun exploration of how quickly 1B rows from a text file can be aggregated. This repo contains my implementation in Rust! 

I wrote a detailed blog about my implementation approach, you can check it out [here](). 
## Record of iterations

Final implementation approach looks like this: 



Here is a more detailed record of each individual iteration:

| Attempt Number | Approach | Execution Time (min:sec) | Diff | Commit | Flamegraph |
|-----------------|---|---|---|--| -- |
|1| Naive Implementation: Read temperatures into a map of cities. Iterate serially over each key (city) in map to find min, max and average temperatures.| 4:21 | | [455609a](https://github.com/Naveenaidu/rust-1brc/commit/455609a22e844759779a0a2c152047a8dfe0c981) | [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/01-Naive-Implementation/flamegraph.svg)  | |
|2|Remove redundant vector creation |4:01|-0.20| [fb2dda8](https://github.com/Naveenaidu/rust-1brc/commit/fb2dda8491d40630bb20437483b76f80ee1145f8)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/02-Use-iterator-instead-of-collect-read-line/flamegraph.svg)  | |
|3|Use BufReader for reading the file |2:28|-1.30|[1f411b6](https://github.com/Naveenaidu/rust-1brc/commit/1f411b68b3c711dbe3ddd32c32c0247f5d61e322)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/03-use-buffreader/flamegraph.svg)  | |
|4|Faster float parsing|2:23|-0.5|[df29672](https://github.com/Naveenaidu/rust-1brc/commit/df29672ee4962a931800a06b005e03020b150e9b)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/04-use-fast-float-SIMD/flamegraph.svg)  | |
|5|Use a faster hashing algorithm and hashmap(FxHashMap) |2.05|-0.18|[afc73a1](https://github.com/shraddhaag/1brc/commit/b7b1781f58fd258a06940bd6c05eb404c8a14af6)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/05-use-Fxhashmap/flamegraph.svg)  | |
|6|Use `read_line` instead of `read_lines`. This maintains a single buffer to store each line we read |1.45|-0.20|[d4b60cf](https://github.com/Naveenaidu/rust-1brc/commit/d4b60cfe8ba58804b45a896e4bd5230cfeb8596d)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/06-use-read_line/flamegraph.svg)  | |
|7|Use bytes `[u8]` instead of `String`|1.38|-0.7|[2bc8cc3](https://github.com/Naveenaidu/rust-1brc/commit/2bc8cc3c53909d9889c7a9de2a430a982bb3533b)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/07-use-bytes-instead-of-string/flamegraph.svg)  | |
|8|Use mmap to read the file, this gives us data in `&[u8]`|1:30|-0.8|[38bdd01](https://github.com/Naveenaidu/rust-1brc/commit/38bdd0131d320b9ed92a2bfa3c86d00796c1c95f)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/08-use-mmap-byte-everywhere/flamegraph.svg)  | |
|9|Use memchr to split the string, memchr has SIMD optimizations|1:18|-0.12|[d24e56f](https://github.com/Naveenaidu/rust-1brc/commit/d24e56fefb699fa953def653d53efd9d6b611139)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/09-use-memchr/flamegraph.svg)  | |
|10|Parallelization - 1, A single producer and multiple receiver with an additional unprocessed_buffer to store the data that cannot be sent in a chunk|0:12|-1.10|[14f33a0](https://github.com/Naveenaidu/rust-1brc/commit/14f33a068b89e6808ef9292570913c525c6756de)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/10-use-multithreading/flamegraph.svg)  | |
|11|Parallelization - 2, uses only a single buffer to store the chunks  |0:9|-0.3|[ad0ea99](https://github.com/Naveenaidu/rust-1brc/commit/ad0ea9998858a4c60b6100f5bbdaccd42ffd4230)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/11-multithreading-single-memory-space/flamegraph.svg)  | |