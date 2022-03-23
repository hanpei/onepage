use anyhow::Result;
use clap::{Arg, Command};
use onepage::{init, SiteBuilder, SiteServer, PAGE_DIR};

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("onepage")
        .author("hanpei")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .about("A simple static site generator")
        .subcommand(
            Command::new("init").about("Initialize the site").arg(
                Arg::new("dir")
                    .short('d')
                    .long("dir")
                    .value_name("DIR")
                    .default_value("./onepage")
                    .help("The directory to initilize the site"),
            ),
        )
        .subcommand(Command::new("build").about("Build the site"))
        .subcommand(Command::new("serve").about("Serve the site"))
        .get_matches();

    match matches.subcommand() {
        Some(("init", arg_matches)) => {
            let dir = arg_matches.value_of("dir").unwrap();
            init(dir)?;
            Ok(())
        }
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
