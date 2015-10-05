extern crate docopt;
//extern crate markdown;
#[macro_use] extern crate nickel;
extern crate pulldown_cmark;
extern crate rustc_serialize;
extern crate time;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, Read};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use docopt::Docopt;
use pulldown_cmark::{Parser, html, Options};
use nickel::{Nickel, HttpRouter, Mount, StaticFilesHandler};
use nickel::status::StatusCode;
use time::Tm;


fn render_markdown_as_html(text: &str) -> String {
    let opts = Options::all();
    let mut s = String::with_capacity(text.len() * 3 / 2);
    let p = Parser::new_ext(&text, opts);
    html::push_html(&mut s, p);
    s
}

const USAGE: &'static str = "
Simple Blog Engine

Usage:
  robo [options] <root>

Options:
  -h --help     Show this screen.
  --port=PORT          Port to listen on [default: 8000].
  --interface=IFACE    Interface to listen on [default: 127.0.0.1].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_root: String,
    flag_port: u16,
    flag_interface: String,
}

struct Entry {
    entry_dir: String,
}

impl Entry {
    pub fn from_parts(year: &str, month: &str, day: &str, title: &str) -> Entry {
        Entry {
            entry_dir: format!("{}-{}-{}-{}", year, month, day, title),
        }
    }

    fn parts(&self) -> (Tm, String) {
        let mut segs = self.entry_dir.splitn(4, "-");

        let mut tm = time::empty_tm();

        tm.tm_year = FromStr::from_str(segs.next().unwrap()).unwrap();
        tm.tm_mon = FromStr::from_str(segs.next().unwrap()).unwrap();
        tm.tm_mon -= 1;
        tm.tm_mday = FromStr::from_str(segs.next().unwrap()).unwrap();
        tm = time::at_utc(tm.to_timespec());
        let name = segs.next().unwrap().to_owned();
        (tm, name)
    }
    pub fn link(&self) -> String {
        let (tm, name) = self.parts();
        format!("{}/{:02}/{:02}/{}/", tm.tm_year, tm.tm_mon + 1, tm.tm_mday, name)
    }
    pub fn name(&self) -> String {
        let (_, name) = self.parts();
        name
    }
    pub fn date(&self) -> String {
        let (tm, _) = self.parts();
        time::strftime("%A, %B %d '%y", &tm).unwrap()
    }
    pub fn is_draft(&self) -> bool {
        let (_, name) = self.parts();
        name.starts_with("_")
    }
    pub fn post_info(&self, post_root: &Path) -> io::Result<HashMap<String, String>> {
        let post_path = post_root.join(&self.entry_dir).join("post.md");
        let f = try!(File::open(&post_path));
        let mut reader = io::BufReader::new(f);
        // Parse out the header
        let mut post_info = HashMap::new();
        let mut line = String::with_capacity(200);
        loop {
            line.truncate(0);
            reader.read_line(&mut line).unwrap();
            if line.trim() == "" {
                break;
            }
            let mut parts = line.splitn(2, ":");
            let key = parts.next().unwrap().trim();
            let value = parts.next().unwrap().trim();
            post_info.insert(key.to_owned(), value.to_owned());
        }

        let mut remaining = String::new();
        reader.read_to_string(&mut remaining).unwrap();
        post_info.insert("date".to_owned(), self.date());
        if post_info.contains_key("title") {
            post_info.insert("title_delim".to_owned(), " | ".to_owned());
        }
        let body = render_markdown_as_html(&remaining);
        post_info.insert("body".to_owned(), body);
        Ok(post_info)
    }

    pub fn entry_root(&self, post_root: &Path) -> PathBuf {
        post_root.join(&self.entry_dir)
    }
}

fn get_entries(path: &Path) -> Vec<Entry> {
    let wd = fs::read_dir(path).ok().expect("Couldn't list post directory");

    let mut entries = vec![];
    for reader in wd {
        let entry = reader.ok().expect("Error enumerating post directory contents");
        let path = entry.path();
        let name = path.file_name().unwrap();
        let sname = name.to_str().unwrap().to_owned();
        entries.push(Entry{ entry_dir: sname });
    }
    entries.sort_by(|a, b| b.link().cmp(&a.link()));
    entries
}

fn get_entry_template_maps(path: &Path) -> Vec<HashMap<String, String>> {
    let entries = get_entries(path);
    let mut maps = Vec::with_capacity(entries.len());
    for e in entries {
        if e.is_draft() {
            continue
        }
        let mut entry = HashMap::new();
        entry.insert("url".to_owned(), e.link());
        entry.insert("name".to_owned(), e.name());
        entry.insert("date".to_owned(), e.date());
        maps.push(entry);
    }
    maps
}

fn main() {
    let mut server = Nickel::new();

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let root_template_path = Path::new(&args.arg_root).join("templates/home.html");
    let post_template_path = Path::new(&args.arg_root).join("templates/post.html");
    let post_path_home = Path::new(&args.arg_root).join("posts");
    let post_path_post = post_path_home.clone();
    let post_path_post_resource = post_path_home.clone();
    let static_path = Path::new(&args.arg_root).join("static");

    server.utilize(Mount::new("/static/",
        StaticFilesHandler::new(static_path.to_str().unwrap())));

    server.get("/", middleware! { |_, response|
        let mut data = HashMap::new();
        data.insert("posts", get_entry_template_maps(&post_path_home));
        return response.render(root_template_path.to_str().unwrap(), &data);
    });

    server.get("/:year/:month/:day/:title/", middleware! { |request, response|
        let entry = Entry::from_parts(
            request.param("year").unwrap(),
            request.param("month").unwrap(),
            request.param("day").unwrap(),
            request.param("title").unwrap(),
        );

        return match entry.post_info(&post_path_post) {
            Ok(data) => {
                response.render(post_template_path.to_str().unwrap(), &data)
            },
            Err(_) => {
                response.error(StatusCode::NotFound, "Post not found")
            }
        };
    });

    server.get("/:year/:month/:day/:title/:resource", middleware! { |request, response| 
        let entry = Entry::from_parts(
            request.param("year").unwrap(),
            request.param("month").unwrap(),
            request.param("day").unwrap(),
            request.param("title").unwrap(),
        );
        let mut url_parts = request.path_without_query().unwrap().rsplitn(2,
            request.param("title").unwrap());
        let file_name = url_parts.next().unwrap().to_owned();
        let ep = entry.entry_root(&post_path_post_resource);
        let resource_path = ep.join(&file_name[1..]);
        return response.send_file(resource_path);
    });

    let listen_location = format!("{}:{}", args.flag_interface, args.flag_port);
    server.listen(&listen_location[..]);
}
