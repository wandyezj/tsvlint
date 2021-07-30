setlocal
set THISDIR=%~dp0
set THISDIR=%THISDIR:~,-1%

set data=%THISDIR%/data

cargo run "%data%/data.json" "%data%/data.tsv"