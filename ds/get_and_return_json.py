#!/usr/bin/env python3

import sys
import requests

HEADERS = {"Content-Type": "application/json"}
URL = "https://raw.githubusercontent.com/prin-r/ipfs/main/simple_json.json"


def main(sliced_index_input):
    try:
        sliced_index = int(sliced_index_input)
    except:
        ValueError("input must be integer")
    r = requests.get(URL)
    r.raise_for_status()

    return r.json()[sliced_index:]


if __name__ == "__main__":
    try:
        print(main(sys.argv[1]))
    except Exception as e:
        print(e, file=sys.stderr)
        sys.exit(1)
