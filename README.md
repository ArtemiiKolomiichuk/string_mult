# String Multiplication

The library provides a parser for simple commands that allow multiplying strings. The library supports commands for multiplying numbers in strings, duplicating strings, and multiplying all numbers in a string. The results of the parsing process are used to evaluate the commands and return their results. 

The library uses the `pest` parser generator to define the grammar of the commands. 

## Parsing Process

1. **Grammar Definition**: The grammar is defined in the `gramm.pest` file. It includes rules for:
   - `num`: Recognizes signed floating point or integer numbers.
   - `int`: Recognizes signed integer numbers.
   - `mult`: Recognizes multiplication operators with optional indices.
   - `multAll`: Recognizes multiplication operators for multiplying all numbers in a string.
   - `duplicate`: Recognizes duplication operators.
   - `inner_str_text`: Recognizes parts of the string parameters that are not numbers.
   - `str_param`: Recognizes string parameters surrounded by quote marks.
   - `command`: Recognizes complete commands for multiplying strings.
   - `commands_list`: Recognizes a list of commands.

![parsing scheme illustraition](https://raw.githubusercontent.com/ArtemiiKolomiichuk/string_mult/refs/heads/master/scheme.png)

2. **Evaluation**: The library includes `evaluate` and `evaluate_list` functions that parse string and string from file respectively and return the result of the command execution.

## Commands examples

1. Multiply first number in string by provided number
    - `"15 packs, 10mg/l" * 10` -> `150 packs, 10mg/l`
\
&nbsp;

2. Multiply n<sup>th</sup> number in string by provided number
    - `"15 packs, 10mg/l" *[0] 10` -> `150 packs, 10mg/l`
    - `"15 packs A, 10 packs B" *[1] 10` -> `15 packs A, 100 packs B`

    ###### Multiply n<sup>th</sup> from end number in string by provided number
    - `"15 packs A, 10 packs B, 9..." *[-1] 10` -> `15 packs A, 10 packs B, 90...`  
    - `"15 packs A, 10 packs B, 9..." *[-2] 10` -> `15 packs A, 100 packs B, 9...`
\
&nbsp;

3. Multiply all numbers in string by provided number
    - `"15 packs A, 10 packs B, 9..." ** 10` -> `150 packs A, 100 packs B, 90...`
\
&nbsp;

4. Duplicate a string n times
    - `"123" *** 3` -> `123123123`
    - `"123" *** 0` -> ` `
    ###### Duplicate reversed string n times 
    - `"123" *** -1` -> `321`
    - `"123" *** -2` -> `321321`  