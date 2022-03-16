use anyhow::Result;
use serde::Serialize;
use tera::Tera;

lazy_static::lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        // tera.register_filter("do_nothing", do_nothing_filter);
        println!("  - Tera loaded {} templates", tera.templates.len());
        tera
    };
}

pub fn render_template(template_name: &str, data: &impl Serialize) -> Result<String> {
    let ctx = tera::Context::from_serialize(data)?;
    Ok(TEMPLATES.render(template_name, &ctx)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_template() {
        let mut ctx = tera::Context::new();
        ctx.insert("title", "test title");
        ctx.insert("content", "hello world");
        let rendered = TEMPLATES.render("post.html", &ctx).unwrap();

        println!("{}", rendered);
    }
}
