# rust-markov-chains

This is my first real Rust project.

```
Markov chain generator
Generates strings of text based on a source file

USAGE:
    rust-markov-strings [OPTIONS] <SOURCE>

ARGS:
    <SOURCE>    Source file to read from

OPTIONS:
    -h, --help                       Print help information
    -n, --num-tokens <NUM_TOKENS>    Number of tokens to attempt to generate [default: 128]
    -s, --start <START>              Token to start with
```

Though I tend to be a perfectionist, since I was so unfamiliar with the language and I'm not used to low level programming I forbid myself from trying to write good code -- working code will be enough. It's rough and there's a lot of edge cases to go over but I'm proud of having been able of tackling something so outside my usual area of work.

```
➜  ~/repos/rust-markov-chains git:(master) ✗ rust-markov-strings -n 32 -s the neuromancer.txt
the drastic simplification of data .  the works either side with his eyes ,  point ,  beneath a mask of their foreheads or somethin' goin' funny choice but if you needed
```
