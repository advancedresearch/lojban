from __future__ import print_function
import sys
import xml.etree.ElementTree as ET
from collections import defaultdict

from corpus_utils import pretty_print_examples

def read_corpus(fname):
    tree = ET.parse(fname)
    root = tree.getroot()

    # get body
    body = None
    for child in root:
        if child.tag == 'body':
            body = child
    assert body is not None

    # get rows
    rows = []
    for row_node in body:
        row = defaultdict(str)
        for entry_node in row_node:
            lang = list(entry_node.attrib.values())[0]
            for seg_node in entry_node:
                row[lang] += seg_node.text
        rows.append(row)
        
    return rows


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print('Usage: {} <corpus json file>'.format(sys.argv[0]))
        sys.exit(-1)

    examples = read_corpus(sys.argv[1])
    pretty_print_examples(examples)
