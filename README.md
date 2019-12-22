[![Licence](https://img.shields.io/badge/License-AGPL%203-purple.svg)](https://opensource.org/licenses/AGPL-3.0)
[![Download Windows](https://img.shields.io/badge/download-windows-green.svg?logo=windows)](https://github.com/cjrh/dictomatic/releases/download/v0.0.3/dictomatic.exe)
[![Download Linux](https://img.shields.io/badge/download-linux-green.svg?logo=linux)](https://github.com/cjrh/dictomatic/releases/download/v0.0.3/dictomatic)

# dictomatic
Static, offline, command-line CLI dictionary

## Demo

Provide a word, get the definitions back in a tab-delimited table:

```shell script
$ dictomatic.exe snag
snag    noun    an unforeseen obstacle  -
snag    noun    an opening made forcibly as by pulling apart    -
snag    noun    a sharp protuberance    -
snag    verb    hew jaggedly    -
snag    verb    catch on a snag I snagged my stocking
snag    verb    get by acting quickly and smartly       snag a bargain

```

## Features

- Definitions from the [Wordset Project](https://github.com/wordset/wordset-dictionary),
  which is licensed under the _Creative Commons Attribution-ShareAlike 4.0 International License_
- Statically compiled, word lists are linked in. No dependencies. 
  Just download an executable for your target platform.
- Fast; takes about 40 ms to emit the words. This makes it easy to drive 
  from your editor, and will work offline.
  
## Install

Just download the executable. Check out the Releases tab.

## Overview

The demo output further up looks a bit odd in the demo above because the 
sections are separated by tabs `\t`. Tabs work well as a separator in 
text-based applications because they almost never appear in text. This 
output format is designed to be easy to use in unix command-line pipelines.

There are always four sections (separated by tabs) in each line:

```shell script
WORD    POS     DEFINITION                              EXAMPLE
snag    verb    get by acting quickly and smartly       snag a bargain
```

If a section is missing, it will appear as a single `-`. For instance, there
are not example usages of _snag_ in the noun form, but there is one example
usage for _snag_ as a verb: _snag a bargain_.

## Multiple words

Multiple words are supported, because it might be convenient to see a bunch of
related definitions at the same time:

```shell script
$ dictomatic.exe word wordy words
word    noun    a unit of language that <snip>  Words are the blocks from which sentences<snip>
word    noun    information about recent<snip   What's the word on the new smart phone?
word    noun    a secret word or phrase <snip>  I forgot the word, but can you still let me in?
word    noun    a brief statement               I didn't say a word about it to anyone.
word    noun    an exchange of views on <snip>  We sat down and had words about politics.
word    noun    a verbal command for act<snip>  When I give the word, charge!
word    noun    a highly valued promise         I gave my word to you, yet you still mana<snip>
word    noun    a string of bits stored <snip>  Large computers use words up to 64 bits long.
word    verb    to put into words or an <snip>  I worded my apology badly.

wordy   adjective       using or containing too many words      -

words   noun    language that is spoken <snip>  They have a gift for words.
words   noun    words making up the dial<snip>  -
words   noun    the text of a popular so<snip>  -
words   noun    the words that are spoken       I listened to their words very closely.
words   noun    an angry dispute                -

```

This is why the the word itself appears in the results: so that downstream
pipeline applications can do things with it.

## Pipeline processing

If you don't supply any words as arguments, it will read a list of words from
stdin. For instance, given a file `test.txt` with contents `rust\ngold`:

```shell script
$ cat test.txt | dictomatic.exe
rust    adjective       of the brown color of rust      -
rust    noun    any of various fungi causing rust disease in plants    <all snipped>
rust    noun    the formation of reddish-brown ferric oxides on iron
rust    noun    a plant disease that produces a reddish-brown discol
rust    noun    a red or brown oxide coating on iron or steel caused
rust    verb    become coated with oxide        -
rust    verb    become destroyed by water, air, or a corrosive such 
rust    verb    cause to deteriorate due to the action of water, air

gold    adjective       having the deep slightly brownish color of g
gold    adjective       made from or covered with gold  gold coins
gold    noun    a deep yellow color     -
gold    noun    something likened to the metal in brightness or prec
gold    noun    great wealth    Whilst that for which all virtue now
gold    noun    coins made of gold      -
gold    noun    a soft yellow malleable ductile (trivalent and univa

```

This means that other programs that can emit "one word per line" can 
feed that output into _dictomatic_.

## Tips & Tricks

Take advantage of CLI filters! How about extracting only the parts of speech:

```shell script
$ dictomatic.exe jump | cut -f2
noun
noun
noun
noun
noun
noun
verb
verb
verb
verb
verb
verb
verb
verb
verb
verb
verb
verb
verb
verb
verb

```

Count the parts of speech (`awk NF` to drop the blank line):

```shell script
$ dictomatic.exe jump | awk NF | cut -f2 | sort | uniq -c
      6 noun
     15 verb
```

Filter definitions and extract the definition only:

```shell script
$ dictomatic.exe jump | grep attack | cut -f3
make a sudden physical attack on
```

Extract only definitions and remove punctuation:

```shell script
$ dictomatic.exe jump | cut -f3  | tr -d [:punct:]
the act of jumping
descent with a parachute
a sudden involuntary movement
film an abrupt transition from one scene to another
an abrupt transition
a sudden and decisive increase
go back and forth
rise in rank or status
increase suddenly and significantly
pass abruptly from one state or topic to another
bypass
enter eagerly into
make a sudden physical attack on
start a car engine whose battery is dead by connecting it to another cars battery
move or jump suddenly as if in surprise or alarm
move forward by leaps and bounds
cause to jump or leap
jump from an airplane and descend with a parachute
run off or leave the rails
jump down from an elevated point
be highly noticeable

```

Extract only usage-examples (filtering out `-`), with the part-of-speech
in trailing brackets:

```shell script
$ dictomatic.exe jump | awk NF | awk -F '\t' '$4!="-"{ print $4 " [" $2 "]" }'
a jump in attendance [noun]
My new novel jumped high on the bestseller list. [verb]
Prices jumped overnight [verb]
We jumped into the game. [verb]
The muggers jumped the couple coming out of the gallery. [verb]
the trainer jumped the tiger through the hoop [verb]
the parachutist didn't want to jump [verb]
```

The full roundtrip, where we 
- find definitions of "jump",
- filter those by appearance of the word "sudden",
- remove punctuation,
- convert spaces to newlines (so each word in the definition is on its own line),
- remove duplicates,
- filter for words longer than 8 characters,
- and feed back into _dictomatic_ for definitions of _those_ words!!

```shell script
$ dictomatic.exe jump \
    | cut -f3 \
    | grep sudden \
    | tr -d [:punct:] \
    | tr [:blank:] '\n' \
    | sort -u \
    | awk 'length($0) > 8' \
    | dictomatic.exe
involuntary     adjective       not subject to the control of the will  involuntary manslaughter
involuntary     adjective       controlled by the autonomic nervous system      -

significantly   adverb  in a significant manner our budget will be significantly affected by these new cuts
significantly   adverb  in an important way or to an important degree   -
significantly   adverb  in a statistically significant way      the two groups differed significantly

```
