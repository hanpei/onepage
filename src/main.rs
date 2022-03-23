use anyhow::Result;
use clap::Command;
use onepage::{SiteBuilder, SiteServer, PAGE_DIR};

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("onepage")
        .version("0.1.0")
        .author("hanpei")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .about("A simple static site generator")
        .subcommand(Command::new("build").about("Build the site"))
        .subcommand(Command::new("serve").about("Serve the site"))
        .get_matches();

    match matches.subcommand() {
        Some(("build", _)) => {
            let mut site = SiteBuilder::new(PAGE_DIR);
            site.build()?;

            Ok(())
        }
        Some(("serve", _)) => {
            println!("Serve site");
            SiteServer::new("127.0.0.1:8080").run()?;
            Ok(())
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_walkdir() {
        walkdir::WalkDir::new("pages/posts")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().display().to_string().ends_with(".md"))
            .for_each(|e| {
                println!("====> {}", e.path().display());
            });
    }
}
