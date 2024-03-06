# Code library 

| | |
|-|-|
| Author | Aidan Beil |
| Date | 5/3/2024 |
| Class | CS240 2963 |
| Professor | Darrell Criss |


## Organization

- [Main entry points](src/main.rs)
- [CLI definitions](src/cli.rs)
- [Source code for interactive mode](src/interactive.rs)

## Usage

```
cargo run help
```

> ```
> CS240 | Demo for BFS and DFS
> 
> Usage: graph-demo <COMMAND>
> 
> Commands:
>   new      
>   open     
>   example  
>   gui      
>   help     Print this message or the help of the given subcommand(s)
> 
> Options:
>   -h, --help
>           Print help (see a summary with '-h')
> ```

```
cargo run new -p test.yaml
```
> Creates a new graph file named test.yaml.

```
cargo run open -p test.yaml
```
> Opens a graph file named test.yaml and starts interactive mode.
> Type `help` for more info.

```
cargo run example pathfinding
```
> Opens the provided pathfinding example in interactive mode.

```
cargo run example job-scheduling
```
> Opens the provided job scheduling example in interactive mode.

### Interactive mode

```
> help
```

> ```
> 
> - add <node>
>     Add the given node to the graph
> - connect <from> <to>
>     Connects the two given nodes
> - disconnect <from> <to>
>     Disconnects the two given nodes
> - filter <filter>
>     Returns all nodes that match the filter
> - help 
>     Displays this message
> - inspect <node>
>     Returns all info on the given node
> - list 
>     Lists all nodes in the graph
> - quit 
>     Quits the application
> - remove <node>
>     Removes the given node from the graph
> - route <from> <to>
>     Find the shortest path from node a to node b
> - schedule 
>     Topo sort
> 
> ```

There is typo detection, and it will suggest valid commands.

All commands have error messages if something goes wrong.

## Sources

