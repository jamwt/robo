title: Sample Post
author: Posty

I blog about *really* important things here.

What kinds of things?
---------------------

Things like this:

 * This is a thing that is important
 * Another thing that is important is going right here, and
   it's wrapping around the page
 * This is a final thing that is important

This is a longer paragraph that talks about things.  Lorem
sorts of things.  Lorem ipsum dolor sit amet, ad vide torquatos
expetendis vim. Vocibus ancillae definiebas sed et, quod oratio
appetere ius id. Mel legimus definitionem ne. Sit an utinam accumsan,
sumo hinc quot quo ut. Legimus corpora vim ea, vim eu decore expetenda.
Laudem dignissim ex eos.

Here's some fake code:

    #!python
    class MyCollection(dict):
        def __init__(self):
            self.foo = "blah"

    def foo():
        print "woo!  Python things!"

Let's try a picture.  Here's a duck:

![](duck.jpg)

Here's some other code:

    #!rust
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
    }

That's all, **folks!!**
