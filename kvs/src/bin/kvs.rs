
use clap::{Arg, Command, Parser, };

pub struct KvsArgs{
    pub method: String,
    
    pub arg: String,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
fn main() {
    let matches = Command::new("kvs")
    .author("Rajaji")
    .version(env!("CARGO_PKG_VERSION"))
    .about("This is a key value store")
    .subcommand(Command::new("set").arg(Arg::new("KEY").required(true)).arg(Arg::new("VALUE").required(true)))
    .subcommand(Command::new("get").arg(Arg::new("KEY").required(true)))
    .subcommand(Command::new("remove").alias("rm").arg(Arg::new("KEY").required(true)))
    .get_matches();

    match matches.subcommand() {
        Some(("set",sub_command))=>{
            panic!("unimplemented");
        }
        Some(("get",sub_command))=>{
            panic!("unimplemented");
        }
        Some(("remove",sub_command))=>{
            panic!("unimplemented");
        }
        _ => unreachable!(),
    }
}