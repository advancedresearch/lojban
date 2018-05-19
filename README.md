# lojban (advancedresearch-lojban)
A Lojban parser in Piston-Meta

Help us expand the vocabulary! PRs are welcome!

[Piston-Meta](https://github.com/pistondevelopers/meta) is a meta-parsing language for human-readable text documents.

[Lojban](https://mw.lojban.org/papri/Lojban) is a carefully constructed language based on formal logic.

This project aim to explore the possibilities of converting Lojban into Piston-Meta's meta data format for usage in Natural Language Processing.

For example, "I give this to you" is in Lojban:

```
mi dunda ti do
```

Is represented in JSON format of meta data (English in parentheses):

```
"bridi":{
 "sumti(object)":{
  "mi(I)":true
 },
 "selbri":{
  "tanru":{
   "brivla":{
    "dunda(gives__gift_to)":true
   }
  }
 },
 "sumti(object)":{
  "ti(this_near_speaker)":true
 },
 "sumti(object)":{
  "do(you)":true
 }
}
```

Instead of parsing a generic Lojban format, this parser annotates meta data
with English definitions. This helps researchers since Lojban is unfamiliar to most people.

### Goals

- A functional Lojban vocabulary
- Rust AST representation of the Lojban language for use in code


### Acknowledgments

We have included a parallel English-Lojban corpus from [Tatoeba](https://en.wikipedia.org/wiki/Tatoeba).
It is licensed under [a Creative Commons Attribution 2.0 license](https://en.wikipedia.org/wiki/Tatoeba#License)