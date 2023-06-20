// use std::env::args;
use std::fs::File;
// use std::cmp::min;
// use std::collections::HashSet;
// // use std::collections::HashMap;
// use std::collections::VecDeque;
mod edge;
use crate::edge::Edge;
// use crate::edge::build_edge;
// use crate::edge::NodeId;
// use crate::edge::EdgeId;
// use crate::edge::Weight;
mod graph;
// use crate::graph::build_graph;
// use crate::graph::Edgelist;
mod flow;
// use crate::flow::build_cycles;
// use crate::flow::print_cycles;
// use crate::flow::initialize_weight_of_neighbors_from;
mod cycle;
// use crate::cycle::find_longest_subwalk;
mod uniqueness;
// use crate::uniqueness::is_maximal;
// use crate::uniqueness::unique_sequences;
// use crate::uniqueness::create_parent_structure;
mod safe_paths;
use crate::safe_paths::safe_paths;
// use crate::cycle::longest_subwalk;
// use crate::cycle::get_former_index;
// use crate::cycle::ac_trie;
// use crate::cycle::try_removing;
use log::{info, LevelFilter};
// use simple_logger::SimpleLogger;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
mod test;
// mod tests;
use clap::Parser;
use std::io::Write;
use std::io::BufWriter;
use std::path::PathBuf;




#[derive(Parser, Debug)]
struct Cli {
    /// The input file containing an arc-centric de Bruijn graph.
    /// The file should be an edgelist with the number of nodes on the first row, then one row for each edge containing the starting node, end node, weight and sequence; each separated by one space.
    #[clap(long)]
    input: String,

    /// The k-mer size used to generate the de Bruijn graph.
    #[clap(short)]
    k: usize,

    /// The output file where the arc-centric de Bruijn graph should be written to.
    #[clap(long)]
    output: PathBuf,

    /// The desired log level.
    #[clap(long, default_value = "Info")]
    log_level: LevelFilter,
}

pub fn initialise_logging(log_level: LevelFilter) {
    CombinedLogger::init(vec![TermLogger::new(
        log_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    info!("Logging initialised successfully");
}






fn main() {
    // Choose the file you want to use
    // -------------------------------------------------------------- 
    // let path = "data/short_k13.edgelist";
    // let path = "data/test_k12.edgelist";
    // let path = "data/reference_k15.edgelist";
    // let path = "data/long_k27.edgelist";
    // let path = "data/ecoli_k12.edgelist";
    // let path = "data/fake.edgelist";

    // Test files
    // let path = "data/test_data/short.edgelist";
    // let path = "data/test_data/sufpref.edgelist";
    // let path = "data/test_data/outflow.edgelist";
    // let path = "data/test_data/outflow_k2.edgelist";
    // let path = "data/test_data/longer_k4.edgelist";

    // args
    // let args: Vec<String> = args().collect();
    // let path = &args[1];  // cargo run -- '../data/long_k27.edgelist'
    // let k : usize = args[2].to_string().parse::<usize>().unwrap();
    // -------------------------------------------------------------- 
    let cli = Cli::parse();
    initialise_logging(cli.log_level);
    info!(
        "Loading graph from {:?} with k = {} and writing to {:?}",
        cli.input, cli.k, cli.output
    );
    let mut output = BufWriter::new(File::create(&cli.output).unwrap());

    let safe_paths = safe_paths(&cli.input, cli.k);


    println!("\n++++++++ Then, the safe paths as final unique strings: ++++++++");
    let mut counter = 0;
    for sequence in &safe_paths {
        println!("Path {}:", counter);
        println!("{} ", sequence);
        writeln!(output, "Path {}:", counter).unwrap();
        writeln!(output, "{} ", sequence).unwrap();
        counter += 1;
    }
}




// cargo run -- --input 'data/test_data/maximal_k3.edgelist' -k 3 --output 'output.txt'