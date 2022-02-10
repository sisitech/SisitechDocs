# PorstgreSQL

## Installing


```
sudo apt update

sudo apt install postgresql postgresql-contrib

```

##  Roles and Databases
Upon installation, Postgres is set up to use peer authentication, meaning that it associates Postgres roles with a matching Unix/Linux system account. If a role exists within Postgres, a Unix/Linux username with the same name is able to sign in as that role.

The installation procedure created a user account called postgres that is associated with the default Postgres role. In order to use Postgres, you can log into that account.


## Creating a New Role
```
sudo -u postgres createuser --interactive
```

!!! success
    Enter name of role to add: sammy

    Shall the new role be a superuser? (y/n) y

## Switching Over to the postgres Account

Switch to the postgres user 
```
sudo -i -u postgres
```

Create a db
```
createdb database1
```

Access the postgres command promnt
```
psql
```

Exit the command prompt
```
\q
```


Updating user password
```
ALTER USER user_name WITH PASSWORD 'new_password';
```


## Update the listening interface 
### Get the `pg_hba.conf` location
Open postgres interactive session
```
sudo -i -u postgres
psql
```

From the postgres  interactive session

``` sql
SHOW config_file;
```

!!! success
    ```
                   config_file 
    -----------------------------------------
    /etc/postgresql/12/main/postgresql.conf
    ```

Open the config file
```
nano /etc/postgresql/12/main/postgresql.conf
```

Search for the `listen` and uncomment and update with the required network interface ip or `0.0.0.0` for any interface.
``` title="/etc/postgresql/12/main/postgresql.conf" linenums="25"
#listen_addresses = 'localhost'         # what IP address(es) to listen on;
```

example

``` title="/etc/postgresql/12/main/postgresql.conf" linenums="25"
listen_addresses = '192.168.60.1'         # what IP address(es) to listen on;
```

Edit the  `pg_hba.conf` file and add
```
host    all         all         ip-address/24    trust
```



Save and close the file.
Restart postgres.

```
sudo systemctl restart postgresql
```




