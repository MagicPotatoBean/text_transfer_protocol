# Error codes
0. Success
1. Not correct format
2. Not correct data
3. Generic server error
4. Generic client error
5. File doesn't exist
6. Unsupported request

# Prerequisites
## Data format
The following represents a single byte with value of 0

|0|
|-|

The following represents the bytes [0, 3, 116]
(quoted letters represent their ASCII value)

|0|3|'t'|
|-|-|-|

## Assembling requests
Requests as shown below are simply appended together to form a request i.e.

|0|1|2|
|-|-|-|

followed by

|3|4|5|
|-|-|-|

becomes

|0|1|2|3|4|5|
|-|-|-|-|-|-|

# Requests
## Request a file
### Header
Starts with 0 to represent "Version 0"
Then 0 to represent a query
Then every label begins with the length of the label (0 for "end")

|  0  |  0  |  3  | 't' | 't' | 'p' |  4  | 't' | 'e' | 's' | 't' | '3' | 'c' | 'o' | 'm' |  0  |
|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|

### Path
Starts with an 8 to represent "File path is 8 digits long"

|  8  | 'f' | 'i' | 'l' | 'e' | '.' | 't' | 'x' | 't' |
|-|-|-|-|-|-|-|-|-|

### Body-length
The body is empty, so body-length is represented by 2^0 bytes = 1 byte
Then the next byte is the length, which is zero

|  0  |  0  |
|-|-|

### Body
This request has no body so is simply not provided

## Upload file (replaces existing file if there is one)
Starts with 0 to represent "Version 0"
Then 1 to represent overwriting upload
Then every label begins with the length of the label (0 for "end")
### Header

|  0  |  1  |  3  | 't' | 't' | 'p' |  4  | 't' | 'e' | 's' | 't' | '3' | 'c' | 'o' | 'm' |  0  |
|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|

### Path
Starts with an 8 to represent "File path is 8 digits long"

|  8  | 'f' | 'i' | 'l' | 'e' | '.' | 't' | 'x' | 't' |
|-|-|-|-|-|-|-|-|-|

### Body-length
The body is only 12 bytes so can be represented by 2^0 = 1 byte
Body is 12 bytes long, so second byte is 12 bytes

|  0  | 12  |
|-|-|

### Body
Starts with 12 to represent "The body is 12 bits long"

| 'h' | 'e' | 'l' | 'l' | 'o' | ' ' | 'w' | 'o' | 'r' | 'l' | 'd' | '!' |
|-|-|-|-|-|-|-|-|-|-|-|-|

## Upload file (Dont overwrite, store elsewhere and return the new path if taken)
Starts with 0 to represent "Version 0"
Then 2 to represent overwriting upload
Then every label begins with the length of the label (0 for "end")
### Header

|  0  |  2  |  3  | 't' | 't' | 'p' |  4  | 't' | 'e' | 's' | 't' | '3' | 'c' | 'o' | 'm' |  0  |
|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|

### Path
Starts with an 8 to represent "File path is 8 digits long"

|  8  | 'f' | 'i' | 'l' | 'e' | '.' | 't' | 'x' | 't' |
|-|-|-|-|-|-|-|-|-|

### Body-length
The body is only 12 bytes so can be represented by 2^0 = 1 byte
Body is 12 bytes long, so second byte is 12 bytes

|  0  | 12  |
|-|-|

### Body
Starts with 12 to represent "The body is 12 bits long"

| 'h' | 'e' | 'l' | 'l' | 'o' | ' ' | 'w' | 'o' | 'r' | 'l' | 'd' | '!' |
|-|-|-|-|-|-|-|-|-|-|-|-|

# Responses
## Return a file
### Header
Starts with 0 to represent "Version 0"
Then 0 to represent a response to a query
Then 0 to represent "success" (values 1-256 represent errors, the text value is returned in the body)

|  0  |  0  |  0  |
|-|-|-|

### Path
Starts with an 8 to represent "File path is 8 digits long"

|  8  | 'f' | 'i' | 'l' | 'e' | '.' | 't' | 'x' | 't' |
|-|-|-|-|-|-|-|-|-|

### Body-length
The body is empty, so body-length is represented by 2^0 bytes = 1 byte
Then the next byte is the length, which is zero

|  0  | 12  |
|-|-|

### Body
The body is the value of the file

| 'h' | 'e' | 'l' | 'l' | 'o' | ' ' | 'w' | 'o' | 'r' | 'l' | 'd' | '!' |
|-|-|-|-|-|-|-|-|-|-|-|-|

## Upload file (replaces existing file if there is one)
Starts with 0 to represent "Version 0"
Then 1 to represent a response to overwriting upload
Then 0 to represent "success" (values 1-256 represent errors, the text value is returned in the body)
### Header

|  0  |  1  |  0  |
|-|-|-|

### Path
Starts with a 0 since there is no meaningful path to return

|  0  |
|-|

### Body-length
The body is empty so can be represented by 2^0 = 1 byte
Body is 0 bytes long, so second byte is 0 bytes

|  0  |  0  |
|-|-|

### Body
The body is empty since the server returned success

## Upload file (Dont overwrite, store elsewhere and return the new path if taken)
Starts with a 0 to represent "Version 0"
Then 2 to represent a response to overwriting upload
Then a 0 to represent "success"
### Header

|  0  |  2  |  0  |
|-|-|-|

### Path
In this case, the server will move the file to "file2.txt" instead of overwriting "file.txt", so will return the new filepath
Starts with an 9 to represent "File path is 8 digits long"

|  9  | 'f' | 'i' | 'l' | 'e' | '2' | '.' | 't' | 'x' | 't' |
|-|-|-|-|-|-|-|-|-|-|

### Body-length
The body is only 12 bytes so can be represented by 2^0 = 1 byte
Body is 12 bytes long, so second byte is 12 bytes

|  0  |  0  |
|-|-|

### Body
Body is empty so nothing is returned
