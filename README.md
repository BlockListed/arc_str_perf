# Step 1: Gather bench data
With JSON set to true in `src/main.rs` run `./run.sh > comparison_data.jsonl` to benchmark
all pairs of SHOULD_DROP and TOTAL_THREAD_COUNT, you may need to
adjust the `TOTAL_THREAD_COUNT_VALUES` variable, if you don't have
12 threads or want to benchmark a different configuraiton.

NOTE: It may also be necessary to remove the `no` option from 
`SHOULD_DROP_VALUES`, since it uses quite a bit of memory with
the default amount of samples.

# Step 2: Turn JSON lines into thread-sorted JSON
Use `jq --slurp "sort_by(.threads)" < comparison_data.jsonl > comparison_data.json`
to transform the data into a sorted json array.

The `--slurp` option is necessary to transform JSON Lines to a JSON array.

# Step 3: Transform to CSV for further processing
Libreoffice calc does not accept JSON files, so we use the python script
`to_csv.py` to transform the data to CSV.
The command is `python3 to_csv.py < comparison_data.json > comparison_data.csv`.

Be ware that the script requires `pandas`.

# Step 4: Use the data
Libreoffice Calc or Excel in my case
