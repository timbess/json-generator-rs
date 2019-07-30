# Json Generator

## Installation
After you have installed [rustup](https://rustup.rs/).
```bash
$ cargo install json-generator
```

## Usage

```bash
$ json-generator -h

json-generator 0.1.0
Timothy Bess <tdbgamer@gmail.com>
Takes in JSON DSL and outputs correctly formatted JSON

USAGE:
    json-generator [FLAGS] <dsl_text>

FLAGS:
    -h, --help       Prints help information
    -p, --pretty     
    -V, --version    Prints version information

ARGS:
    <dsl_text>    
```

### Simple ES Query
```bash
$ json-generator -p 'query=bool=must=[match=foo=bar, match=bar=200]' 
{
  "query": {
    "bool": {
      "must": [
        {
          "match": {
            "foo": "bar"
          }
        },
        {
          "match": {
            "bar": 200
          }
        }
      ]
    }
  }
}
```


### Objects
```bash
$ json-generator -p 'a=b c=d d=e e=f'
{
  "a": "b",
  "c": "d",
  "d": "e",
  "e": "f"
}
```

### Arrays
```bash
$ json-generator -p '[a, b, c, d, e, f]'
[
  "a",
  "b",
  "c",
  "d",
  "e",
  "f"
]
```

### Numbers
```bash
$ json-generator -p '[1, 1.1, 1.2e10, -100]'
[
  1,
  1.1,
  12000000000.0,
  -100
]
```
