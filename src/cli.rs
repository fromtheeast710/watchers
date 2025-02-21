use clap::Parser;

const ABOUT: &str = "Create an Awesome list with starred repos";

#[derive(Parser, Debug)]
#[command(version, about = &ABOUT, long_about = None)]
pub struct CLI {
  #[arg(short, long)]
  pub token: String,

  #[arg(short, long, default_value = "README.md")]
  pub file: String,

  #[arg(short, long, default_value_t = false)]
  pub preview: bool,

  #[arg(
    short = 'F',
    long,
    default_value = r#"+ **[{owner}/{name}]({url})** `⭐ {star}`"#
  )]
  pub format: String,

  // #[arg(short = 'w', long, default_value_t = false)]
  // pub following: bool,

  #[arg(short, long, default_value_t = false)]
  pub content_table: bool,
}
