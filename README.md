# rust-resp-protocol

REdis Serialization Protocol

## install

add `resp-protocol` to `Cargo.toml`
``` toml
[dependencies]
resp-protocol = "0.0.1"
```

## usage

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

``` rust
"+OK\r\n"
```

##### Build

``` rust
use resp_protocol::SimpleString;

let simple_string = SimpleString::new(b"OK");
```

##### Parse

``` rust
use resp_protocol::SimpleString;

let string = "+OK\r\n";
let simple_string = SimpleString::parse(string.as_bytes(), &mut 0, &string.len()).unwrap();
``