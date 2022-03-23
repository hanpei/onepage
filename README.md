# ONEPAGE

### Overview

A simple static site genertor, convert md posts to html.

`cargo run -- build` to build pages
`cargo run -- serve` to build pages

- Read /pages markdown files
- Parse md to html by [pulldown cmark](https://docs.rs/pulldown-cmark/latest/pulldown_cmark/)
- Render into [tera template](https://github.com/Keats/tera)
- styled by [picocss](https://picocss.com/) and [highlightjs](https://highlightjs.org/)

### structure

- `/pages`: markdown source file
  - `index.md` => _index page_
  - `/posts/*.md` => _post page_
  - `/assets` img/css ...
  - `/favicon` favicon files
- `/dist`: static stie pages
- `/src`: rust src

### todo

- [ ] site init
- [x] css style
- [x] add command line
- [x] serve /dist
- [x] watch /pages and rebuild

### reference

- [Build Your Own Static Site Generator](https://blog.hamaluik.ca/posts/build-your-own-static-site-generator/)
- [Building a static site generator in 100 lines of Rust](https://kerkour.com/rust-static-site-generator)
- [mdblog](https://github.com/FuGangqiang/mdblog.rs)
