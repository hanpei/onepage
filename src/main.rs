use anyhow::Result;
use clap::{crate_version, Arg, Command};
use onepage::{init, SiteBuilder, SiteServer, STARTER_TEMPLATE_URL};

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("onepage")
        .author("hanpei")
        .version(crate_version!())
        .arg_required_else_help(true)
        .subcommand_required(true)
        .about("A simple static site generator")
        .subcommand(
            Command::new("init").about("Initialize the site").arg(
                Arg::new("dir")
                    .value_name("DIR")
                    .default_value("./onepage")
                    .help("The directory to initilize the site"),
            ),
        )
        .subcommand(Command::new("build").about("Build the site"))
        .subcommand(Command::new("serve").about("Serve the site"))
        .subcommand(
            Command::new("new").about("Create a new post page").arg(
                Arg::new("name")
                    .value_name("NAME")
                    .required(true)
                    .help("The name of the post page"),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", arg_matches)) => {
            let dir = arg_matches.value_of("dir").unwrap();
            init(dir, STARTER_TEMPLATE_URL)?;
            Ok(())
        }
        Some(("build", _)) => {
            let mut site = SiteBuilder::new();
            site.build()?;

            Ok(())
        }
        Some(("serve", _)) => {
            println!("Serve site");
            SiteServer::new("127.0.0.1:8080").run()?;
            Ok(())
        }
        Some(("new", arg_matches)) => {
            let mut name = arg_matches.value_of("name").unwrap().to_string();
            if !name.ends_with(".md") {
                name.push_str(".md");
            }
            let mut site = SiteBuilder::new();

            site.create_page(&name)?;

            Ok(())
        }
        _ => unreachable!(),
    }
}
