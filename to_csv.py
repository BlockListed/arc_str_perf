import pandas as pd
import sys

df = pd.read_json(sys.stdin)

df = df[['threads', 'type', 'drop', 'time_millis']]

df.drop_duplicates(subet=['threads', 'type', 'drop'])

df.to_csv(sys.stdout, index=False)
