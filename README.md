# 1BRC

1️⃣🐝🏎️ [The One Billion Row Challenge](https://github.com/gunnarmorling/1brc) -- A fun exploration of how quickly 1B rows from a text file can be aggregated. This repo contains my implementation in Rust! 

I wrote a detailed blog about my implementation approach, you can check it out [here](https://naveenaidu.dev/tackling-the-1-billion-row-challenge-in-rust-a-journey-from-5-minutes-to-9-seconds). 
## Record of iterations

Final implementation approach looks like this: 
![final iteration visualised](/img/final-approach.png)


Here is a more detailed record of each individual iteration:

| Attempt Number | Approach | Execution Time (sec) | Diff | Commit | Flamegraph |
|-----------------|---|---|---|--| -- |
|1| Naive Implementation: Read temperatures into a map of cities. Iterate serially over each key (city) in map to find min, max and average temperatures.| 253 | | [455609a](https://github.com/Naveenaidu/rust-1brc/commit/455609a22e844759779a0a2c152047a8dfe0c981) | [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/01-Naive-Implementation/flamegraph.svg)  | |
|2|Remove redundant vector creation |241|-12| [fb2dda8](https://github.com/Naveenaidu/rust-1brc/commit/fb2dda8491d40630bb20437483b76f80ee1145f8)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/02-Use-iterator-instead-of-collect-read-line/flamegraph.svg)  | |
|3|Use BufReader for reading the file |137|-104|[1f411b6](https://github.com/Naveenaidu/rust-1brc/commit/1f411b68b3c711dbe3ddd32c32c0247f5d61e322)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/03-use-buffreader/flamegraph.svg)  | |
|4|Faster float parsing|134|-3|[df29672](https://github.com/Naveenaidu/rust-1brc/commit/df29672ee4962a931800a06b005e03020b150e9b)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/04-use-fast-float-SIMD/flamegraph.svg)  | |
|5|Use a faster hashing algorithm and hashmap(FxHashMap) |123|-11|[afc73a1](https://github.com/Naveenaidu/rust-1brc/commit/afc73a15857f06e30bb493e492089202bb3d3b57)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/05-use-Fxhashmap/flamegraph.svg)  | |
|6|Use `read_line` instead of `read_lines`. This maintains a single buffer to store each line we read |105|-18|[d4b60cf](https://github.com/Naveenaidu/rust-1brc/commit/d4b60cfe8ba58804b45a896e4bd5230cfeb8596d)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/06-use-read_line/flamegraph.svg)  | |
|7|Use bytes `[u8]` instead of `String`|83|-22|[2bc8cc3](https://github.com/Naveenaidu/rust-1brc/commit/2bc8cc3c53909d9889c7a9de2a430a982bb3533b)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/07-use-bytes-instead-of-string/flamegraph.svg)  | |
|8|Use mmap to read the file, this gives us data in `&[u8]`|78|-5|[38bdd01](https://github.com/Naveenaidu/rust-1brc/commit/38bdd0131d320b9ed92a2bfa3c86d00796c1c95f)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/08-use-mmap-byte-everywhere/flamegraph.svg)  | |
|9|Use memchr to split the string, memchr has SIMD optimizations|71|-7|[d24e56f](https://github.com/Naveenaidu/rust-1brc/commit/d24e56fefb699fa953def653d53efd9d6b611139)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/09-use-memchr/flamegraph.svg)  | |
|10|Parallelization - 1, A single producer and multiple receiver with an additional unprocessed_buffer to store the data that cannot be sent in a chunk|12|-59|[14f33a0](https://github.com/Naveenaidu/rust-1brc/commit/14f33a068b89e6808ef9292570913c525c6756de)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/10-use-multithreading/flamegraph.svg)  | |
|11|Parallelization - 2, uses only a single buffer to store the chunks  |9|-3|[ad0ea99](https://github.com/Naveenaidu/rust-1brc/commit/ad0ea9998858a4c60b6100f5bbdaccd42ffd4230)| [flamegraph](https://github.com/Naveenaidu/rust-1brc/blob/main/flamegraphs/11-multithreading-single-memory-space/flamegraph.svg)  | |
