# Lint TSV files

TSV files are often used for data exchange.

Provide a linter to lint TSV files.

## Using Rules

Want a dll that can be loaded in multiple applications

%USERPROFILE%\.cargo\bin

## Thoughts

Idea is that each column can be validated with a regex

Some rules will be built in for convenienve

Instead of using bytes for numbers like uint use terms like whole_less_than_million, this is more human readable which is the point of the metadata, how it's represented in the machine can then be mapped appropriately

Collect metadata about the column? What is the longest string value in each column? (useful for formatting)

Skip validation of lines with inappropriate number of values

Report line number of lines with not enough values

Suggest built in regex types based off scan? Able to generate metadata from simply looking at data? Able to assemble regex types based off scan?

TSV must not have blank lines


probably better to have a single namespace and then use versioning, if want the new built in rules then need to version appropriately

can have a generic test for each column to see if it can be easily converted to CSV (simply make sure there are no commas `,` and no double quotes `"`). Can also do the same for CSV to see if it can be easily converted to TSV.

Generate commands to import into a sqlite database, generate the table. Will need to be able to specify concentrations of types.

Need a generic iterator that can be passed to the scanner that can iterate by row and then by value. That way different file formats can be supported.

Simpler to simply specify a delimiter and the format supported and to not allow double quote `"` perhaps simply ban this character all together. Weird because of non uniform standard for handling and escaping text in CSV.


## Test Cases

enable running scan without file so can have built in test cases. This should be done by simply having a line value stream into the scan function, this can then become scan, without needing to worry about how the lines and values are broken up.

- empty file
- only header present
- header has more values than metadata
- header has fewer values than metadata
- line has more values than metadata
- line has fewer values than metadata
- regex does not match value

- performance on large files