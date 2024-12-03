use octocrab::{models::Repository, Octocrab, Result};
use std::{
  collections::{BTreeMap, HashMap},
  fs::OpenOptions,
  io::{BufWriter, Write},
};

const WRITE_ERR: &str = "Unable to write data!";
const OPEN_ERR: &str = "Unable to open file!";
const DATA_ERR: &str = "Unable to parse data!";

pub struct Repo {
  repos: Vec<Repository>,
  // repo_lang: String,
  // repo_name: String,
  // repo_owner: String,
}

impl Repo {
  #[tokio::main(flavor = "multi_thread", worker_threads = 12)]
  pub async fn new(token: &str) -> Result<Repo> {
    let ocrab = Octocrab::builder().personal_token(token).build()?;
    let pages = ocrab
      .current()
      .list_repos_starred_by_authenticated_user()
      .sort("created")
      .per_page(100)
      .send()
      .await?;
    let repos = ocrab.all_pages(pages).await?;
    // let repo = repos.clone().into_iter().next().expect(&DATA_ERR);

    Ok(Self { repos })
  }

  pub fn iter_repo(&self) -> BTreeMap<String, Vec<String>> {
    let mut col: HashMap<String, Vec<String>> = HashMap::new();

    for repo in self.repos.clone() {
      let lang = repo
        .language
        .as_ref()
        .map_or("N/A".to_string(), |l| l.to_string())
        .replace('\"', "")
        .replace(" ", "");
      let owner = &repo.owner.as_ref().expect(&DATA_ERR).login;
      let name = &repo.name;
      let star = &repo.stargazers_count.expect(&DATA_ERR);
      let url = repo.html_url.as_ref().expect(&DATA_ERR);

      // TODO: custom formatting
      let form = format!(r#"+ **[{owner}/{name}]({url})** `{star}`"#);

      // TODO: move N/A to the end
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

    if langs.iter().count() > 9 {
      for (i, lang) in langs.iter().enumerate() {
        if i == 4 {
          table.push_str("|\n|-|-|-|-|\n")
        }

        if i % 4 == 0 && i != 0 && i != 4 {
          table.push_str("|\n");
        }

        table.push_str(&format!("|**{}. [{lang}](#{lang})**", i + 1));
      }
      table.push_str("\n");
    } else {
      for (i, lang) in langs.iter().enumerate() {
        table.push_str(&format!("**{}. [{lang}](#{lang})**\n", i + 1))
      }
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
