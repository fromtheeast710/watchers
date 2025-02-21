use octocrab::{models::Repository, Octocrab, Result};
use std::{
  collections::{BTreeMap, HashMap},
  fs::OpenOptions,
  io::{BufWriter, Write},
};
use strfmt::strfmt;

const WRITE_ERR: &str = "Unable to write data!";
const OPEN_ERR: &str = "Unable to open file!";
const DATA_ERR: &str = "Unable to parse data!";

pub struct Repo {
  repos: Vec<Repository>,
  format: String,
}

impl Repo {
  #[tokio::main(flavor = "multi_thread", worker_threads = 12)]
  pub async fn new(token: &str, format: String) -> Result<Repo> {
    let ocrab = Octocrab::builder().personal_token(token).build()?;
    let pages = ocrab
      .current()
      .list_repos_starred_by_authenticated_user()
      .sort("created")
      .per_page(100)
      .send()
      .await?;
    let repos = ocrab.all_pages(pages).await?;

    Ok(Self { repos, format })
  }

  pub fn iter_repo(&self) -> BTreeMap<String, Vec<String>> {
    let mut vars: HashMap<String, String> = HashMap::new();
    let mut col: HashMap<String, Vec<String>> = HashMap::new();

    for repo in &self.repos {
      let lang = repo
        .language
        .as_ref()
        .map_or("N/A".to_string(), |l| l.to_string())
        .replace('\"', "")
        .replace(' ', "");

      // TODO: support release, pulls, description
      vars.insert(
        "owner".to_string(),
        repo.owner.as_ref().expect(&DATA_ERR).login.clone(),
      );
      vars.insert("name".to_string(), repo.name.clone());
      vars.insert(
        "star".to_string(),
        repo.stargazers_count.expect(&DATA_ERR).to_string(),
      );
      vars.insert("url".to_string(), repo.url.as_ref().to_string());
      vars.insert(
        "issue".to_string(),
        repo.open_issues_count.expect(&DATA_ERR).to_string(),
      );
      vars.insert(
        "release".to_string(),
        repo.releases_url.as_ref().expect(&DATA_ERR).to_string(),
      );
      // vars.insert("topic".to_string(), repo.topics.expect(&DATA_ERR));
      // vars.insert(
      //   "about".to_string(),
      //   repo.description.as_ref().expect(&DATA_ERR).to_string(),
      // );

      let form = strfmt(&self.format.to_string(), &vars).unwrap();

      // TODO: move N/A to the end
      col.entry(lang).or_insert_with(Vec::new).push(form);
    }

    col.into_iter().collect()
  }

  pub fn format_table(&self) -> String {
    let langs: Vec<String> = self.iter_repo().keys().cloned().collect();

    let mut table = String::new();

    table.push_str("## Table of Contents\n");

    if langs.iter().count() > 9 {
      for (i, lang) in langs.iter().enumerate() {
        match i {
          4 => table.push_str("|\n|-|-|-|-|\n"),
          _ if (i % 4 == 0 && i != 0 && i != 4) => table.push_str("|\n"),
          _ => {}
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
