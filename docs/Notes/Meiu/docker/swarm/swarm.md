# Docker Swarm

- Master slave configurations
- At least three masters(From two is okay)- Odd numbers for masters is recommended
- A maximum of 7 master nodes is recommended by docker

## Creating a swarm
- Identify the master
- initiaize the cluster
- Chose an interface to advertise


```bash title="bash"
swarm init  --advertise-addr 192.168.56.1
```
Output:


## Managers
- Getting command to run as manager
```bash title="bash-MANAGER1"
docker swarm join-token manager
```
 
### Promoting a worker node to manager

- You can promote a node from a manager 
```bash title="bash-MASTER"
docker node promote docker-node2
```
`docker-node2` is the hostname of the node

- To update the hostname
```bash  title="bash-NODE2"
hostname master-3
```

### Prevent manager from running services
- Run the command from a manager
```bash title="bash-MANAGER"
docker node update --availability drain docker-master

```
`docker-master` is the hostname.


!!! note
    If the cluster is left with less than half of the managers, reinitializing the swarm can fix it if bringing the managers back up is not possible.<br>
    command: `docker swarm init --force-new-cluster --advertise-addr 192.168.56.1`

## Nodes
- Getting command to run as manager
```bash title="bash-MANAGER"
docker swarm join-token worker
```

- Listing
```bash title="bash-MANAGER"
docker node ls 
```

- Leaving a swarm
```bash title="bash-NODE"
docker swarm leave
```
- Takes some time to reflect on the master 

- Leaving 
```bash title="bash-MASTER"
docker node rm bash-NODE
```

## Services

Two types of services

   - Replicated -
   - Global (Placed on every node, eg monitoring tools) 




