use clap::Parser;

mod cli;
mod repo;

fn main() {
  let cli = cli::CLI::parse();
  let repo = repo::Repo::new(&cli.token, cli.format).expect("Error");

  if cli.preview {
    repo.preview(cli.content_table);
  } else {
    println!("Writing to file: {}", cli.file);
    repo.write_file(cli.content_table, cli.file);
  }
}
