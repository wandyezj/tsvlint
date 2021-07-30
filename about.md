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

## Test Cases

- empty file
- only header present
- header has more values than metadata
- header has fewer values than metadata
- line has more values than metadata
- line has fewer values than metadata
- regex does not match value
