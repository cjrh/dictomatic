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
            if 'speech_part' in v:
                one_defn.append(f"[{v['speech_part']}]")
            if 'def' in v:
                one_defn.append(v['def'])
            if 'example' in v:
                one_defn.append(f"[example] {v['example']}")

            elems.append(' '.join(one_defn))

        defns = '\n'.join(elems)
        out[word] = defns

    return out

def main():
    bigd = dict()
    for fname in glob(glb):
        d = process(fname)
        bigd.update(d)

    with open('dictomatic.json', 'w') as f:
        json.dump(bigd, f)


if __name__ == '__main__':
    main()
