# minigrep
Followed the Rust book to build a mini project. Nothing special.

# Usage

## Word Search
`cargo run -- find queryWord filepath`

minigrep will dump all lines from the file that contain the query.

`export CASE_INSENSITIVE=true`

minigrep queries will be case insensitive for the current terminal session.

Note: *String literals are also supported as file input.*

## Announce
`cargo run -- meow`

minigrep will meow at you.


# Planned Features
1. Word count.
2. Character count.
3. Refactor using [Clap](https://docs.rs/clap/latest/clap/).
