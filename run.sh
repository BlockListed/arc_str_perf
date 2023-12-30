#!/bin/bash

# Define the executable path
EXECUTABLE="./target/release/arc_str_perf"

# Define the possible values for SHOULD_DROP and TOTAL_THREAD_COUNT
SHOULD_DROP_VALUES=("yes" "no")
TOTAL_THREAD_COUNT_VALUES=(1 2 4 8 12)

# Iterate through combinations
for SHOULD_DROP in "${SHOULD_DROP_VALUES[@]}"; do
  for TOTAL_THREAD_COUNT in "${TOTAL_THREAD_COUNT_VALUES[@]}"; do
    # Set environment variables and run the executable
    env SHOULD_DROP=$SHOULD_DROP TOTAL_THREAD_COUNT=$TOTAL_THREAD_COUNT $EXECUTABLE

    # You can add additional commands here if needed, such as logging or handling the output
  done
done
