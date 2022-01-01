# wordtop
| sort | uniq -c but in top-like form (pipe stream, it counts words and displays stats every N seconds)
```
USAGE:
    wordtop [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -l, --line       Line mode - count same lines not words.
    -V, --version    Prints version information

OPTIONS:
    -o, --out <out>            Save total count into a file at the end.
    -r, --refresh <refresh>    Refresh every <N> seconds. [default: 2]
    -s, --sort <sort>          Sort by [default: count]  [possible values: count, rate]
    -t, --top <top>            Display top N words [default: 25]
```

```
(base) marek@nibble:~$ while true ; do cat * ; done | wordtop -o /tmp/summary.txt
the        [491246/s] 2954217
and        [295924/s] 1786906
of         [274029/s] 1645436
to         [191802/s] 1157063
a          [147575/s] 894189
in         [127540/s] 767234
I          [110490/s] 659471
that       [104080/s] 619937
he         [75208/s]  452447
his        [75089/s]  451934
was        [70870/s]  436817
with       [66105/s]  398408
for        [61381/s]  368046
it         [60481/s]  362249
be         [58447/s]  345213
is         [58180/s]  342783
And        [56943/s]  331515
not        [54379/s]  322274
as         [50267/s]  303437
you        [47262/s]  281015
my         [45620/s]  268144
they       [43971/s]  261303
had        [41875/s]  257275
have       [41487/s]  246151
all        [38553/s]  230762
```
