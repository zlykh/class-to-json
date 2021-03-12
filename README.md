# class-to-json
Copy-paste tool to convert Java class to JSON object.     
Simple as `Copy -> c2j -> Paste`    
Generates JSON object with simple values inserted for each attribute.      
Comments, javadoc and annotations ignored.

## Quick Start
[VirusTotal report](https://www.virustotal.com/gui/file/4fa315b013d1bf77f96aa3688803508227315a01351a68806f528fa055164cec/detection)
1. Download pre-built binary `/bin/c2j.exe`
1. Put it to `windows/system32` so it can b invoked from everywhere
1. Copy class fields you would like to convert
    ```
        private String myString;
    
        /**
         * javadoc
         */
        public int myNumber;
    
        @DynamoDBRangeKey(attributeName = "SK")
        protected Currency curr;
    
        //comment
        private Clz clz;
    ```
1. Optionally create a `dict.txt` in `C:\Users\{username}\class-to-json` file with `{type}={value}` format   
See `dict-exmaple.txt`
    ```
    Currency=USD
    Clz={"a":333}
    ```
1. Run `c2j.exe` without any arguments   
1. Result
    ```
    {
    "gg":"asd",
    "curr":"USD",
    "clz":"{"a":333}"
    }
    ```

## Use with IntelliJ "Run Anything"
Copy to `windows/system32`, rename to `c2j`   
Then press Ctrl x2 times and type
![alt text](./img/class-to-json.JPG)

## Use as command line
`c2j --help`   
