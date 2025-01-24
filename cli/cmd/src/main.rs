use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Add { description: String },
    Done { id: u64 },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.cmd {
        Some(Command::Add { description }) => {
            println!("子命令:添加 -> 参数值: {description}");
        }
        Some(Command::Done { id }) => {
            println!("子命令:完成 -> 参数值: {id}");
        }
        None => {
            println!("不存在的子命令");
        }
    }

    Ok(())
}
