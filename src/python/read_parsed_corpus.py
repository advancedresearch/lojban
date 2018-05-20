from __future__ import print_function
import sys
import json

from corpus_utils import pretty_print_examples

def read_parsed_corpus(fname):
    ls = []
    with open(fname) as f:
        obj = json.load(f)
        while 'tail' in obj:
            ls.append(obj['head'])
            obj = obj['tail']
        ls.append(obj['head'])

    return ls


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print('Usage: {} <corpus json file>'.format(sys.argv[0]))
        sys.exit(-1)

    examples = read_parsed_corpus(sys.argv[1])
    pretty_print_examples(examples)

