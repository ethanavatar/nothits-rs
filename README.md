# nohits-rs

A command line tool to find which strings don't exist in an input file

## Usage

```bash
$ ./nohits.exe --help
Takes a file (values) that contains a list of strings and outputs a list of which string didn't
appear in the input file (input)

USAGE:
    nohits.exe --values <VALUES> --input <INPUT>

OPTIONS:
    -h, --help               Print help information
    -i, --input <INPUT>      The path to the file containing contents to match against
    -v, --values <VALUES>    The path to the file containing the values to search for. (cases
                             separated by newlines)
```

### Example

query.txt

```txt
Jack Russell
James Bond
```

dogs.txt

```txt
Jack Russell
King Charles
Dandie Dinmont
Gordon Setter
```

output

```bash
$ nohits --values query.txt --input dogs.txt
James Bond
```
