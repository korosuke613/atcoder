# atcoder
My AtCoder problems.

## Require
- Deno
- Rust

## Add new problem
Create a .rs file and add the bin configuration to Cargo.toml.
```
./add_new_problem.sh <ATCODER_PROBLEM_URL>
```

Example: `./add_new_problem.sh https://atcoder.jp/contests/abs/tasks/abc081_b`.

## Run
```
./run.sh <CONTEST_NAME>_<TASK_NAME>
```

Example: `./run.sh abs_abc081_b`.

## Asakatsu
Create a commit and pull request.

```
./asakatsu.sh <YEAR> <MONTH> <DAY>
```

Example: `./asakatsu.sh 2021 10 11`.
