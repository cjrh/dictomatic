import json
import pathlib
from typing import TextIO, Dict

files = [
    r"G:\Documents\repos\dictprocess\data\chambers1908\pg38700.txt",
    r"G:\Documents\repos\dictprocess\data\chambers1908\pg37683.txt",
    r"G:\Documents\repos\dictprocess\data\chambers1908\pg38538.txt",
    r"G:\Documents\repos\dictprocess\data\chambers1908\pg38699.txt",
]


def process(fname: str) -> Dict:
    with open(fname, encoding='utf-8') as f:
        data = f.read()

    chunks = data.split('\n\n')

    sentinel = r'*       *       *       *       *'

    section_indexes = []
    for i, c in enumerate(chunks):
        if sentinel in c:
            section_indexes.append(i)

    print(f'Found sentinels at: {section_indexes}')

    chunks = chunks[section_indexes[4] + 1:section_indexes[-1]]
    print(chunks[0])
    print(chunks[-1])

    out = dict()
    for chunk in chunks:
        word, _, defn  = chunk.partition(',')
        if not word.isupper():
            continue

        out[word.lower()] = chunk.replace('\n', ' ')

    if 'sabella' in out:
        print(out['sabella'])

    return out


def main():
    bigd = dict()
    for fname in files:
        print(f'FILE: {fname}')
        print()
        d = process(fname)
        bigd.update(d)

    with open('dictomatic-cham.json', 'w', encoding='utf-8') as f:
        json.dump(bigd, f)


if __name__ == '__main__':
    main()
