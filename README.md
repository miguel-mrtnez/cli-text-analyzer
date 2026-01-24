# CLI Text Analyzer

A minimal `wc` implementation.


## Usage

---

```bash
cargo run -- [OPTION] ... <FILE> ...
```

| Option             | Description                                |   
|--------------------|--------------------------------------------|
| `--l`, `--lines`   | Count the number of lines.                 |   
| `--w`, `--words`   | Count the number of words.                 |   
| `--c`, `--chars`   | Count the number of Unicode scalar values. |
| `--b`, `--bytes`   | Count the number of bytes.                 |
| `--t`, `--top <n>` | Print the top `n` most frequent words.     |


Default behavior (no flags provided) equals to:

```bash
cargo run -- --lines --words --chars <FILE> ...
```
