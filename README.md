# Flux

A command dispatcher using the
[Sorbet](https://github.com/thoq-jar/sorbet) file format.

## Example
```sorbet
sorbet => echo "Hello, Sorbet!"
```
or multiline:
```sorbet
multiline => echo "Line 1"
           > echo "Line 2"
           > echo "Line 3"
           > echo "Line 4"
```
Then to run you would run `flux run sorbet`
This is stored in `flux.srb`
