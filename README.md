# kaprekar

Generates Kaprekar constants and series.

## Usage

```bash
Usage: kaprekar [OPTIONS]

Options:
  -n, --number <NUMBER>          Perform Kaprekar's routine on a number [default: 0]
  -s, --start <START>            Perform Kaprekar's routine starting from a number [default: 0]
  -e, --end <END>                Perform Kaprekar's routine up to a number [default: 0]
  -a, --all                      Perform Kaprekar's routine on all numbers
  -i, --iterations <ITERATIONS>  Number of iterations to perform [default: 20]
  -t, --truncate                 Empty out non-series and non-constant vectors.
  -o, --output <OUTPUT>          Output the results to a csv file
  -c, --cont <CONT>              Continue adding results to an existing csv file
  -v, --verbose                  Print the Kaprekar routine
      --symlink                  Create a symlink in /usr/local/bin
  -h, --help                     Print help
  -V, --version                  Print version
```

##
