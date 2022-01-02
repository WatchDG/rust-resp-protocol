# rust-resp-protocol

REdis Serialization Protocol

## Install

add `resp-protocol` to `Cargo.toml`
``` toml
[dependencies]
resp-protocol = "0.0.9"
```

## Usage

``` rust
use resp_protocol;
```

## Types
* Simple string
* Error
* Integer
* Bulk string
* Array

### Simple string

#### Examples

##### Value

``` text
"+OK\r\n"
```

##### Build

``` rust
use resp_protocol::SimpleString;

let simple_string: SimpleString = SimpleString::new(b"OK");
```

##### Parse

``` rust
use resp_protocol::SimpleString;

let string: &str = "+OK\r\n";
let simple_string: SimpleString = SimpleString::parse(string.as_bytes(), &mut 0, &string.len()).unwrap();
```

### Error

#### Examples

##### Value

``` text
"-ERROR\r\n"
```

##### Build

``` rust
use resp_protocol::Error;

let error: Error = Error::new(b"ERROR");
```

##### Parse

``` rust
use resp_protocol::Error;

let string: &str = "-ERROR\r\n";
let error: Error = Error::parse(string.as_bytes(), &mut 0, &string.len()).unwrap();
```

### Integer

#### Examples

##### Value

``` text
":100\r\n"
```

##### Build

``` rust
use resp_protocol::Integer;

let integer: Integer = Integer::new(-100i64);
```

##### Parse

``` rust
use resp_protocol::Integer;

let string: &str = ":-100\r\n";
let integer: Integer = Integer::parse(string.as_bytes(), &mut 0, &string.len()).unwrap();
```

### Bulk string

#### Examples

##### Value

``` text
"$6\r\nfoobar\r\n"
```

##### Build

``` rust
use resp_protocol::BulkString;

let bulk_string: BulkString = BulkString::new(b"foobar");
```

##### Parse

``` rust
use resp_protocol::BulkString;

let string: &str = "$6\r\nfoobar\r\n";
let bulk_string: BulkString = BulkString::parse(string.as_bytes(), &mut 0, &string.len()).unwrap();
```

### Array

#### Examples

##### Value

``` text
"*0\r\n"                            // empty array
"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n"  // bulk strings array
"*2\r\n:1\r\n$6\r\nfoobar\r\n"      // mixed types array
```

##### Build

``` rust
use resp_procotol::{Array, ArrayBuilder, RespType, Integer, BulkString};

let mut array_builder: ArrayBuilder = ArrayBuilder::new();
array_builder.insert(RespType::Integer(Integer::new(100)));
array_builder.insert(RespType::BulkString(BulkString::new(b"foobar")));

let array: Array = array_builder.build();
println!("{:?}", array); // Array(b"*2\r\n:100\r\n$6\r\nfoobar\r\n")
```

##### Parse

``` rust
use resp_protocol::Array;

let string = "*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";
let array = Array::parse(string.as_bytes(), &mut 0, &string.len()).unwrap();
println!("{:?}", array); // Array(b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n")
```