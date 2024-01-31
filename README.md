# Natord PG

Natural ordering operators for PostgreSQL `text` type

## Installation

### Ubuntu

```shell
cargo install cargo-pgrx
sudo apt install bison flex libreadline-dev postgresql-14

git clone https://github.com/mpajkowski/natord_pg.git
cd natord_pg/
cargo pgrx init
cargo pgrx package

sudo cp -v target/release/natord_pg-pg14/usr/share/postgresql/14/extension/natord_pg.control /usr/share/postgresql/14/extension/
sudo cp -v target/release/natord_pg-pg14/usr/share/postgresql/14/extension/natord_pg-*.sql  /usr/share/postgresql/14/extension/
sudo cp -v target/release/natord_pg-pg14/usr/lib/postgresql/14/lib/natord_pg.so /usr/lib/postgresql/14/lib/
sudo systemctl restart postgresql
```

## Usage

```sql
CREATE EXTENSION natord_pg;
```

## Operators

```
~~> - Natural greater than
<~~ - Natural less than
~>= - Natural greater equal
<=~ - Natural less equal
```
