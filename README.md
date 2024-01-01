# Step 1: Gather bench data
With JSON set to true in `src/main.rs` run
`./run.sh | jq --slurp "sort_by(.threads)" > comparison_data.json` to benchmark
all pairs of SHOULD_DROP and TOTAL_THREAD_COUNT, you may need to
adjust the `TOTAL_THREAD_COUNT_VALUES` variable, if you don't have
12 threads or want to benchmark a different configuraiton.

The `jq` option `--slurp` turns the JSON Lines coming from our program
into a normal JSON array, which is then sorted.

NOTE: It may also be necessary to remove the `no` option from 
`SHOULD_DROP_VALUES`, since it uses quite a bit of memory with
the default amount of samples.

# Step 2: Transform to CSV for further processing
Libreoffice calc does not accept JSON files, so we use the python script
`to_csv.py` to transform the data to CSV.
The command is `python3 to_csv.py < comparison_data.json > comparison_data.csv`.
This also deduplicates the date. Mainly, because of the repeated testing of `Rc<str>`.

Be ware that the script requires `pandas`.

# Step 4: Use the data
Libreoffice Calc or Excel in my case
