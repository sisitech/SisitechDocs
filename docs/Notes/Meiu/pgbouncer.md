# PG Bouncer

PostgreSQL has a rather heavyweight connection handling architecture. For each incoming connection, the postmaster (the main Postgres daemon) forks out a new process (conventionally called a backend) to handle it. While this design provides better stability and isolation, it does not make it particularly efficient at handling short-lived connections. A new Postgres client connection involves TCP setup, process creation and backend initialization – all of which are costly in terms of time and system resources. [For More...](https://pgdash.io/blog/pgbouncer-connection-pool.html)

## Installation

``` title="Debian/Ubuntu"
sudo apt-get install pgbouncer
```

## Usage

PgBouncer relies on a main configuration file, typically stored as `/etc/pgbouncer/pgbouncer.ini`. You can invoke pgbouncer as a systemd service, or simply run it even without superuser privileges with the path to this configuration file.

## Configuration

``` title="pgbouncer.ini" line
[databases]
db1 = host=localhost dbname=db1

[pgbouncer]
listen_addr = 127.0.0.1
listen_port = 16432
auth_file = userlist.txt
```
