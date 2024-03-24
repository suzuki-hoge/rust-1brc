# 1 Billion Row Challenge ( Rust )

## Requirements

```
$ rustup install nightly
```

## Setup

```
$ git clone https://github.com/gunnarmorling/1brc.git 1brc

$ cd 1brc

$ docker run -it -v '.:/work' -w /work amazoncorretto:21 ./mvnw clean verify

$ docker run -v '.:/work' -w /work amazoncorretto:21 ./create_measurements.sh 10000

$ mv measurements.txt path/to/your-project/data/10000.txt
```

## Record

#### 10000000 records

|                             | debug build | release build |
|-----------------------------|-------------|---------------|
| first implement             | 17.223 sec  | 1.335 sec     |
| use hashbrown and entry_ref | 15.272 sec  | 0.955 sec     |
| split by find               | 11.902 sec  | 0.756 sec     |

#### 1000000000 records

|                             | release build |
|-----------------------------|---------------|
| first implement             | 120.311 sec   |
| use hashbrown and entry_ref | 96.936 sec    |
| split by find               | 76.295 sec    |
