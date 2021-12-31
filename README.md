# rust-resp-protocol

REdis Serialization Protocol

## Install

add `resp-protocol` to `Cargo.toml`
``` toml
[dependencies]
resp-protocol = "0.0.3"
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