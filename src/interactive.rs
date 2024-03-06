///////////////////////////////////////////////////////////////////////////////

use std::{
    collections::{BTreeMap, BinaryHeap},
    io::{self, Write},
};

use anyhow::{Error, Ok, Result};
use cs_240_library::data_structures::graphs::{
    breadth_first_search, depth_first_search, directed_graph::DirectedGraph, Graph, GraphMut,
};

///////////////////////////////////////////////////////////////////////////////

pub type StrGraph = DirectedGraph<String>;

//---------------------------------------------------------------------------//

type Callback = fn(&mut StrGraph, Vec<String>, &BTreeMap<String, Action>) -> Result<bool>;

///////////////////////////////////////////////////////////////////////////////
struct Action {
    desc: String,
    callback: Callback,
    pattern: String,
}

//---------------------------------------------------------------------------//

fn action(desc: &str, cb: Callback, pt: &str) -> Action {
    Action {
        desc: desc.to_owned(),
        callback: cb,
        pattern: pt.to_owned(),
    }
}

//---------------------------------------------------------------------------//

fn add_action(
    actions: &mut BTreeMap<String, Action>,
    nm: &str,
    pt: &str,
    desc: &str,
    cb: Callback,
) {
    actions.insert(normalize(nm), action(desc, cb, pt));
}

///////////////////////////////////////////////////////////////////////////////

fn build_actions() -> BTreeMap<String, Action> {
    let mut actions = BTreeMap::<String, Action>::new();

    add_action(
        &mut actions,
        "list",
        "",
        "Lists all nodes in the graph",
        list,
    );
    add_action(
        &mut actions,
        "add",
        "<node>",
        "Add the given node to the graph",
        add,
    );
    add_action(
        &mut actions,
        "remove",
        "<node>",
        "Removes the given node from the graph",
        remove,
    );
    add_action(
        &mut actions,
        "connect",
        "<from> <to>",
        "Connects the two given nodes",
        connect,
    );
    add_action(
        &mut actions,
        "disconnect",
        "<from> <to>",
        "Disconnects the two given nodes",
        disconnect,
    );
    add_action(
        &mut actions,
        "filter",
        "<filter>",
        "Returns all nodes that match the filter",
        filter,
    );
    add_action(
        &mut actions,
        "inspect",
        "<node>",
        "Returns all info on the given node",
        inspect,
    );
    add_action(
        &mut actions,
        "route",
        "<from> <to>",
        "Find the shortest path from node a to node b",
        route,
    );
    add_action(&mut actions, "schedule", "", "Topo sort", schedule);
    add_action(&mut actions, "help", "", "Displays this message", help);
    add_action(&mut actions, "quit", "", "Quits the application", quit);

    actions
}

///////////////////////////////////////////////////////////////////////////////

pub fn interactive(graph: &mut StrGraph) -> Result<()> {
    let mut running = true;
    let mut input = String::new();

    let actions = build_actions();

    println!("Type `help` for usage");

    while running {
        print!("> ");
        io::stdout().flush()?;
        match io::stdin().read_line(&mut input) {
            std::result::Result::Ok(_) => match handle_input(&input, graph, &actions) {
                std::result::Result::Ok(res) => running = res,
                Err(err) => println!("{}", err),
            },
            Err(err) => println!("Error reading user input: {}", err),
        };
        input.clear();
        println!();
    }

    Ok(())
}

//---------------------------------------------------------------------------//

fn handle_input(
    input: &str,
    graph: &mut StrGraph,
    actions: &BTreeMap<String, Action>,
) -> Result<bool> {
    println!();
    let input = normalize(input);
    let args: Vec<String> = input.split_whitespace().map(|e| e.to_owned()).collect();
    let name = args.get(0).ok_or(Error::msg("No command given"))?;

    if let Some(action) = actions.get(name) {
        (action.callback)(graph, args, &actions)
    } else {
        println!("Unknown command, did you mean one of these?");
        find_similar(name, &actions);
        Ok(true)
    }
}

///////////////////////////////////////////////////////////////////////////////

fn normalize(input: &str) -> String {
    input.to_owned().trim().to_lowercase()
}

//---------------------------------------------------------------------------//

fn find_similar(input: &str, actions: &BTreeMap<String, Action>) {
    let mut suggestions = BinaryHeap::with_capacity(actions.len());

    for other in actions.keys() {
        suggestions.push(RankedWord {
            ranking: levenshtein_distance(&input, &other),
            word: other.to_string(),
        })
    }

    for suggestion in suggestions.into_sorted_vec().iter().take(3) {
        println!("    {}", suggestion.word);
    }
}

///////////////////////////////////////////////////////////////////////////////

fn list(graph: &mut StrGraph, _: Vec<String>, _: &BTreeMap<String, Action>) -> Result<bool> {
    let mut nodes = graph.get_all();
    nodes.sort();

    for node in nodes {
        println!("- {}", node);
    }

    Ok(true)
}

//---------------------------------------------------------------------------//

fn add(graph: &mut StrGraph, args: Vec<String>, _: &BTreeMap<String, Action>) -> Result<bool> {
    let node = args.get(1).ok_or(Error::msg("Missing <node> argument"))?;

    graph.insert_node(node.to_string(), vec![]);

    Ok(true)
}

//---------------------------------------------------------------------------//

fn remove(graph: &mut StrGraph, args: Vec<String>, _: &BTreeMap<String, Action>) -> Result<bool> {
    let node = args.get(1).ok_or(Error::msg("Missing <node> argument"))?;

    graph.remove_node(node.to_string());

    Ok(true)
}

//---------------------------------------------------------------------------//

fn connect(graph: &mut StrGraph, args: Vec<String>, _: &BTreeMap<String, Action>) -> Result<bool> {
    let from = args.get(1).ok_or(Error::msg("Missing <from> argument"))?;
    let to = args.get(2).ok_or(Error::msg("Missing <to> argument"))?;

    graph.insert_edge(from.to_string(), to.to_string());

    Ok(true)
}

//---------------------------------------------------------------------------//

fn disconnect(
    graph: &mut StrGraph,
    args: Vec<String>,
    _: &BTreeMap<String, Action>,
) -> Result<bool> {
    let from = args.get(1).ok_or(Error::msg("Missing <from> argument"))?;
    let to = args.get(2).ok_or(Error::msg("Missing <to> argument"))?;

    graph.remove_edge(from.to_string(), to.to_string());

    Ok(true)
}

//---------------------------------------------------------------------------//

fn filter(graph: &mut StrGraph, args: Vec<String>, _: &BTreeMap<String, Action>) -> Result<bool> {
    let filter = args.get(1).ok_or(Error::msg("Missing <filter> argument"))?;

    for node in graph.get_all() {
        if node.starts_with(filter) {
            println!("- {}", node);
        }
    }

    Ok(true)
}

//---------------------------------------------------------------------------//

fn inspect(graph: &mut StrGraph, args: Vec<String>, _: &BTreeMap<String, Action>) -> Result<bool> {
    let node = args.get(1).ok_or(Error::msg("Missing <node> argument"))?;

    println!("- {}", node);
    for adj in graph.get_adj(node) {
        println!("    - {}", adj);
    }

    Ok(true)
}

//---------------------------------------------------------------------------//

fn route(graph: &mut StrGraph, args: Vec<String>, _: &BTreeMap<String, Action>) -> Result<bool> {
    let from = args.get(1).ok_or(Error::msg("Missing <from> argument"))?;
    let to = args.get(2).ok_or(Error::msg("Missing <to> argument"))?;

    let res = breadth_first_search(graph.clone(), from.to_owned());

    if let Some(path) = res.get(to) {
        for item in path {
            println!("- {}", item);
        }
        println!("- {}", to);
    } else {
        println!("Couldn't reach {} from {}", to, from);
    }

    Ok(true)
}

//---------------------------------------------------------------------------//

fn schedule(graph: &mut StrGraph, _: Vec<String>, _: &BTreeMap<String, Action>) -> Result<bool> {
    let (_, order, cyclic) = depth_first_search(graph.clone());

    if cyclic {
        Err(Error::msg("Unable to sort cyclical graph"))
    } else {
        for node in order {
            print!(" -> {}", node);
        }
        println!();
        Ok(true)
    }
}

//---------------------------------------------------------------------------//

fn help(_: &mut StrGraph, _: Vec<String>, actions: &BTreeMap<String, Action>) -> Result<bool> {
    for (name, action) in actions {
        println!("- {} {}", name, action.pattern);
        println!("    {}", action.desc);
    }

    Ok(true)
}

//---------------------------------------------------------------------------//

fn quit(_: &mut StrGraph, _: Vec<String>, _: &BTreeMap<String, Action>) -> Result<bool> {
    Ok(false)
}

///////////////////////////////////////////////////////////////////////////////

/// Ternary operator
///
/// Returns a if c is true, otherwise b
#[inline]
fn t<T>(c: bool, a: T, b: T) -> T {
    if c {
        a
    } else {
        b
    }
}

/// 3 way min
#[inline]
fn min<T>(a: T, b: T, c: T) -> T
where
    T: Ord,
{
    a.min(b).min(c)
}

///////////////////////////////////////////////////////////////////////////////

/// Edit distance between two words
fn levenshtein_distance(first: &str, second: &str) -> usize {
    // iterative single row based on [30], [29], [27], and [31]

    // just changing the types to be easier to work with
    let first: Vec<char> = first.chars().collect();
    let second: Vec<char> = second.chars().collect();

    // create the row [0..|s|+1]
    let mut row: Vec<usize> = (0..second.len() + 1).into_iter().collect();

    // variables to track what would have been previous rows
    let mut previous_diagonal;
    let mut previous_above = 0;

    // for each character in `first`
    for i in 0..first.len() {
        row[0] = i + 1;

        // for each character in `second`
        for j in 0..second.len() {
            // check if the characters are equal
            let indicator = t(first[i] != second[j], 1, 0);
            // update tracking variables
            previous_diagonal = previous_above;
            previous_above = row[j + 1];
            // update current row entry with 3 way minimum from deleting,
            // inserting and substituting
            row[j + 1] = min(
                previous_above + 1,            // deleting
                row[j] + 1,                    // inserting
                previous_diagonal + indicator, // substituting
            );
        }
    }

    // return the last entry in the row
    row[second.len()]
}
///////////////////////////////////////////////////////////////////////////////

/// Struct for priority queue entries
#[derive(Debug, Eq, PartialOrd)]
struct RankedWord {
    ranking: usize,
    word: String,
}

//---------------------------------------------------------------------------//

impl PartialEq for RankedWord {
    fn eq(&self, other: &Self) -> bool {
        self.ranking == other.ranking && self.word == other.word
    }
}

impl Ord for RankedWord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ranking.cmp(&other.ranking)
    }
}

///////////////////////////////////////////////////////////////////////////////
