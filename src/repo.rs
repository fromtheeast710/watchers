use octocrab::{models::Repository, Octocrab, Page, Result};
use std::{
  collections::{BTreeMap, HashMap},
  fs::OpenOptions,
  io::{BufWriter, Write},
};

const WRITE_ERR: &str = "Unable to write data!";
const OPEN_ERR: &str = "Unable to open file!";
const DATA_ERR: &str = "Unable to parse data!";

pub struct Repo {
  repos: Page<Repository>,
  // repo_lang: String,
  // repo_name: String,
  // repo_owner: String,
}

impl Repo {
  #[tokio::main(flavor = "multi_thread", worker_threads = 12)]
  pub async fn new(token: &str) -> Result<Repo> {
    let ocrab = Octocrab::builder().personal_token(token).build()?;
    // TODO: collect from all pages
    let repos = ocrab
      .current()
      .list_repos_starred_by_authenticated_user()
      .sort("created")
      .per_page(100)
      .send()
      .await?;

    Ok(Self { repos })
  }

  pub fn iter_repo(&self) -> BTreeMap<String, Vec<String>> {
    let mut col: HashMap<String, Vec<String>> = HashMap::new();

    for repo in self.repos.clone() {
      let lang = repo
        .language
        .as_ref()
        .map_or("N/A".to_string(), |l| l.to_string())
        .replace('\"', "");
      // TODO: custom formatting
      let form = format!(
        r#"+ **[{}/{}]({})** `:star: {}`"#,
        repo.owner.as_ref().expect(&DATA_ERR).login,
        repo.name,
        repo.html_url.as_ref().expect(&DATA_ERR),
        repo.stargazers_count.expect(&DATA_ERR),
      );

      col
        .entry(lang)
        .and_modify(|c| c.push(form.clone()))
        .or_insert_with(|| vec![form]);
    }

    col.into_iter().collect()
  }

  pub fn format_table(&self) -> String {
    let langs: Vec<String> = self.iter_repo().keys().cloned().collect();

    let mut table = String::new();

    table.push_str("## Table of Contents\n");

    for lang in langs {
      table.push_str(&format!("  + [{lang}](#{lang})\n"));
    }

    table
  }

  pub fn format_repo(&self) -> String {
    let data = self.iter_repo();

    let mut form = String::new();

    for (lang, repo) in data.iter() {
      form.push_str(&format!("## {lang}\n"));

      for rep in repo {
        form.push_str(&format!("  {rep}\n"));
      }
    }

    form
  }

  pub fn preview(&self, show_table: bool) {
    if show_table {
      println!("{}", self.format_table() + &self.format_repo());
    } else {
      println!("{}", self.format_repo())
    }
  }

  pub fn write_file(&self, show_table: bool, filename: String) {
    let content = if show_table {
      self.format_table() + &self.format_repo()
    } else {
      self.format_repo()
    };
    let file = OpenOptions::new()
      .write(true)
      .create(true)
      .open(format!("./{filename}"))
      .expect(&OPEN_ERR);

    let mut writer = BufWriter::new(&file);

    writer.write_all(content.as_bytes()).expect(&WRITE_ERR)
  }
}
