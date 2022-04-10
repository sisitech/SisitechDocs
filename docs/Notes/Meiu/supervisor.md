# Supervisor

## Installation

Install in the users home directory

```
pip3 install --user supervisor
```

## Configuration

Sample Config

``` conf
[unix_http_server]
file=/home/myuser/apps/myapp/supervisor.sock

[supervisord]
logfile=/home/myuser/logs/myapp/supervisord.log
logfile_maxbytes=50MB
logfile_backups=10
loglevel=info
pidfile=/home/myuser/apps/myapp/supervisord.pid

[rpcinterface:supervisor]
supervisor.rpcinterface_factory = supervisor.rpcinterface:make_main_rpcinterface

[supervisorctl]
serverurl=unix:///home/myuser/apps/myapp/supervisor.sock

[program:myapp]
directory=/home/myuser/apps/myapp/myproject
environment=PYTHONPATH=/home/myuser/apps/myapp/myproject
command=/home/myuser/apps/myapp/env/bin/daphne -b 0.0.0.0 -p NNNNN myproject.asgi:application
stdout_logfile=/home/myuser/logs/myapp/myapp_access.log
stderr_logfile=/home/myuser/logs/myapp/myapp_error.log
```

## Starting

```
supervisord -c ~/supervisor.conf
```

## Check the status

```
supervisorctl -c ~/supervisor.conf
```

## Restarting

```
supervisorctl -c ~/supervisor.conf

restart myapp
```

## Automating restart

```
supervisorctl  -c supervisor.conf <<EOF restart myapp
EOF
```
