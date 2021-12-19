# `poker_prob`

Program to calculate the probabilities of winning hands in rust based on rule of 4 and 2

## Sample commands

### Help command

`poker_prob.exe -h`

Output:

```/bin/bash
    poker_prob.exe [OPTIONS] --mh <STRING> --ch <STRING>

OPTIONS:
    -a                   Set whether this is all in or not
        --ch <STRING>    Set community cards
    -h, --help           Print help information
        --mh <STRING>    Set my hand
    -V, --version        Print version information
```

### Calculate probabilities

`poker_prob.exe --ch Ad3h --mh 4h3c5c6h -a`

Output:

```/bin/bash
Straight has the probability of 8%
Two Pair has the probability of 6%
One Pair has the probability of 0%
Three Of A Kind has the probability of 4%
Flush has the probability of 20%
Full House has the probability of 10%
```
