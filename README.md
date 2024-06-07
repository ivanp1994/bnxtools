A simpler (one threaded, concurrency not yet implemented) option to filter bionano's molecules in BNX format.
Currently, valid options are:

`--input` - the BNX file to filter

`--labels` - the minimum number of labels a molecule should have

`--length` - the minimal length of a molecule

`--output` - a file to write the BNX file, if omitted, it's pushed to standardout

Things to implement:
1) "# Number of Molecules:" element in header should be updated to reflect a new number of molecules
2) Concurrency for parallelization
3) Rescaling (stretching) of molecules (similarish to fandom)
 
