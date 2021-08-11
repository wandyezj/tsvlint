@setlocal
@echo off
set THISDIR=%~dp0
set THISDIR=%THISDIR:~,-1%

set data=%THISDIR%/data

set metadata=%data%/metadata.json
set data=%data%/error-data.tsv

cargo run "%metadata%" "%data%"