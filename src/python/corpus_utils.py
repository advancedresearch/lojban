from __future__ import print_function

def pretty_print_examples(examples):
    for example in examples:
        for lang in example:
            print(lang, example[lang])
        print()

