import json
import pathlib
from typing import TextIO, Dict
from glob import glob

glb = './data/wordset-dictionary/data/*.json'


def process(fname: str) -> Dict:
    with open(pathlib.Path() / fname, encoding='utf-8') as f:
        d = json.load(f)

    print(f'Words: {len(d)}')

    out = dict()
    for i, (word, data) in enumerate(d.items()):
        if not i % 500:
            print(f'Processed: {i}')

        if 'meanings' not in data:
            continue

        elems = []
        for v in data['meanings']:
            one_defn = []
            for pt in ['speech_part', 'def', 'example']:
                # Missing values become "-", but we also subst newlines
                one_defn.append(v.get(pt, '-').replace('\n', ' '))

            elems.append('\t'.join(one_defn))

        # Note: dict[str, list[str]]
        out[word.strip()] = elems

    return out

def main():
    bigd = dict()
    for fname in glob(glb):
        d = process(fname)
        bigd.update(d)

    print(bigd['alibi'])

    with open('words.txt', 'w', encoding='utf-8', newline='\n') as words, \
            open('defns.txt', 'w', encoding='utf-8', newline='\n') as defns:
        for word in sorted(bigd):
            words.write(f'{word}\n')
            defns.write(f'{json.dumps(bigd[word])}\n')


if __name__ == '__main__':
    main()
