# LeetCode CLI cheat sheet

Commands verified against the installed `leetcode 0.5.5` CLI. Run them from this directory with your LeetCode account and CLI configuration already set up.

## Solve a problem

Use problem **645 — Set Mismatch**, which already has a Rust solution file in this directory:

```bash
# View/pick the problem
leetcode pick 645

# Open the solution in your configured editor, using Rust
leetcode edit 645 --lang rust

# Run tests through LeetCode
leetcode test 645

# Run a custom testcase
leetcode test 645 '[1,2,2,4]'

# Run the testcases saved in the existing file
leetcode test 645 "$(cat 645.set-mismatch.tests.dat)"

# Automatically test when the solution file changes
leetcode test 645 --watch

# Submit the solution to LeetCode
leetcode exec 645
```

Replace `645` with another problem ID as needed. Edit `645.set-mismatch.rs` directly if you prefer your own editor. Testing and submission use LeetCode's service and require network access.

## Find problems

```bash
# List all problems or search by title
leetcode list
leetcode list 'set mismatch'

# Easy problems you have not completed
leetcode list --query eD

# Filter by tag or ID range
leetcode list --tag array
leetcode list --range 50 100

# Show statistics for the filtered list
leetcode list --query eD --stat

# Pick a problem with filters
leetcode pick --query eD --tag array
```

Query flags can be combined. Uppercase negates a flag:

| Flag | Meaning | Uppercase meaning |
| --- | --- | --- |
| `e` | Easy | `E`: medium or hard |
| `m` | Medium | `M`: easy or hard |
| `h` | Hard | `H`: easy or medium |
| `d` | Completed | `D`: not completed |
| `l` | Locked | `L`: not locked |
| `s` | Starred | `S`: not starred |

For example, `eDL` selects easy, uncompleted, unlocked problems.

## Daily challenge

```bash
leetcode pick --daily
leetcode edit --daily --lang rust
leetcode test --daily
leetcode exec --daily
```

## Maintenance and help

```bash
# Refresh the problem cache
leetcode data --update

# Show a submissions chart
leetcode stat

# Enable Bash completion in the current shell
source <(leetcode completions bash)

# Check version and command options
leetcode --version
leetcode --help
leetcode test --help

# Enable debug output when troubleshooting
leetcode --debug list
```

Short aliases are also available: `p` (pick), `e` (edit), `t` (test), `x` (exec), `l` (list), `d` (data), `s` (stat), and `c` (completions). For example, `leetcode t 645` tests problem 645 and `leetcode x 645` submits it.
