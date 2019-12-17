import json
import pathlib
from typing import TextIO

fname = 'data/dictionary.json'


def process(out: TextIO):
    with open(pathlib.Path() / fname) as f:
        d = json.load(f)

    for i, (word, definition) in enumerate(d.items()):
        assert '\t' not in word
        assert '\t' not in definition

        if not i % 500:
            print(f'Processed: {i}')

        out.write(f'{word}\t{definition}')


def main():
    with open('dictomatic.tsv', 'w') as f:
        process(f)


if __name__ == '__main__':
    main()
