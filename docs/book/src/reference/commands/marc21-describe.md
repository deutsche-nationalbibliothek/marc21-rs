# marc21-describe(1)

## NAME

*marc21-describe* --- Creates a frequency table of all subfield codes


## SYNOPSIS

`marc21 describe` [_OPTIONS_] [_PATH_]...

## DESCRIPTION

The `describe` command creates a  table that lists, for each field, how
often a subfield code appears in the input. Since subfields appear only
in the data fields, control fields are not included in the output. The
columns `ind1` and `ind2` contain the values of the indicators.

## OPTIONS

`--tsv`
: Write output tab-separated (TSV)

`-o`, `--output <path>`
: Write output to `<path>` instead of `stdout`. If the filename ends in
`.tsv` or `.tsv.gz`, the output is automatically saved in TSV format.
The output is gzip-compressed when the filename ends with `.gz`.

### FILTER OPTIONS

{{ #include filter-opts.md }}

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

```
$ marc21 describe -s GND.mrc -o out.csv
10,220,897 records, 0 invalid | 472,874 records/s, elapsed: 00:00:21

$ head -10 out.csv
field,ind1,ind2,0,2,3,4,5,9,S,a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,z
024,7, ,10220897,10863183,0,0,0,198362,0,10863183,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
034, , ,132557,135686,50,0,0,136105,0,0,0,0,136068,136068,136068,136068,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
035, , ,0,0,0,0,0,5495629,0,20441794,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,6637796
040, , ,0,0,0,0,0,10220996,0,10220901,10220897,10220901,10220897,4524432,260659,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
042, , ,0,0,0,0,0,0,0,10220897,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
043, , ,0,0,0,0,0,15,0,0,0,9157636,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
065, , ,0,2617204,0,0,0,0,0,2617204,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
075, , ,0,20051640,0,0,0,0,0,0,20080983,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
079, , ,0,0,0,0,0,0,0,10220897,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,12469759,0,0,0,5126056,0,0,0,0
```
