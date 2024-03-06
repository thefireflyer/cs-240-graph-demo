# CS240 | Graph Demos 

| | |
|-|-|
| Author | Aidan Beil |
| Date | 5/3/2024 |
| Class | CS240 2963 |
| Professor | Darrell Criss |

[![Rust](https://github.com/thefireflyer/cs-240-graph-demo/actions/workflows/rust.yml/badge.svg)](https://github.com/thefireflyer/cs-240-graph-demo/actions/workflows/rust.yml)

## Organization

- [Main entry points](src/main.rs)
- [CLI definitions](src/cli.rs)
- [Source code for interactive mode](src/interactive.rs)

## Why BFS and DFS

Breadth-first-search is great for path finding on unweighted graphs because it searches layer by layer, meaning it will naturally come to the shortest path first. I wrote a very quick path-finding demo you can try with `cargo run example pathfinding` and then writing `route Bellingham Seattle` or something similar. Though, the paths are not weighted by actual distance and might not make a ton of sense irl.

Conversely, depth-first-search is great for scheduling problems. DFS descends as deep as possible in a given sub-tree before processing other paths. This means when it does finally finish processing a node, all of its children have already been fully explored. That sounds just like a scheduling problem! One may want to install a note-taking app, but the note-taking app needs postgres and a networking library. So, before we can even start working on the actual app, we need to build it's dependencies, *it's children*. Again, its DFS all over again. I didn't really have time to write an interesting demo, but `cargo run example job-scheduling` demos the core idea.

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

