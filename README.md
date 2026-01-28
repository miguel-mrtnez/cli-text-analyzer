# CLI Text Analyzer

A minimal `wc` implementation.

```bash
cargo run -- [OPTION] ... <FILE> ...
```

Reads one or more files and prints the specified counts. When no flags are provided, lines, words and char counts are displayed. At least one FILE path must be passed.

| Option             | Description                                |   
|--------------------|--------------------------------------------|
| `--l`, `--lines`   | Count the number of lines.                 |   
| `--w`, `--words`   | Count the number of words.                 |   
| `--c`, `--chars`   | Count the number of Unicode scalar values. |
| `--b`, `--bytes`   | Count the number of bytes.                 |
| `--t`, `--top <n>` | Print the top `n` most frequent words.     |

A word is considered a nonempty sequence of non white spaces, delimited by white space characters, or the limits of the file.