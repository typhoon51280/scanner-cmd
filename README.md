# scanner-cmd

Keyboard Input Simulator 1.0.0

USAGE:
    scanner-cmd.exe [OPTIONS] --code `<code>`
```
OPTIONS:
    -c, --code <code>      Inline Text Input (single quote): 'ipselorumdixit'
    -d, --delay <delay>    Delay before input simulation start
    -f, --file <file>      Filename Text Input
    -h, --help             Print help information
    -p, --parse <parse>    Parse mode enabled
    -V, --version          Print version information
```

## Parse Mode

### ***Disabled*** (default)

The text is entered as-is, no handling of special keys (SHIFT,ALT,...).

### ***Enabled***

The text is parsed before keyboard sequence is entered.

The following brackets are available:

- UNICODE
- CTRL
- SHIFT
- META
- ALT

```
{+UNICODE}Hello world 😀{-UNICODE}
{+CTRL}a{-CTRL}
{+SHIFT}Hello World{-SHIFT}
{+META}Hello World{-META}
{+ALT}Hello World{-ALT}
```

Thera are also special conversion of the following characters:
```
- {CR} => \r
- {LF} => \n
```

Examples:

```
{+SHIFT}hello{CR}{LF}world{-SHIFT}
```

is entered as follows: 

```
HELLO
WORLD
```
