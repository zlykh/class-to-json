# class-to-json
Converts Java class to sample JSON object.   
Simple basic values inserted to each attribute.

## Quick Start
Copy class fields you would like to convert
```
    private String gg;
    private Currency curr;
    private Clz clz;
```

Optionally create a `dict.txt` file with `{type}={value}` format.   
Use `-d` to specify full path without file name.
```
Currency=USD
Clz={"a":333}
```

Result
```
{
"gg":"asd",
"curr":"USD",
"clz":"{"a":333}"
}
```

Nested objects will be wrapped in quotes, it is a known issue.    
Better use with plain objects.

## Use with IntelliJ "Run Anything"
Copy to `windows/system32`, rename to `c2j`   
Then press Ctrl x2 times and type
![alt text](./img/class-to-json.JPG)

## Use as command line
`c2j --help`   
`--input`: text to convert
`--dict`: `dict.txt` file path. Dict file to have `{type}={value}` format


```
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dict <dict>      [default: .]
    -i, --input <input>
```

