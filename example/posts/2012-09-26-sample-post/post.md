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

    #!haskell
    import           Data.Time.Calendar (fromGregorian, Day)
    import           System.Locale (defaultTimeLocale)
    import           Data.Maybe (catMaybes)

    markdownText a = TLE.decodeUtf8 $ LB.fromChunks [markdown a]

    data TemplateSet = TemplateSet {
          postTemplate :: Template
        , homeTemplate :: Template
        }

    type Template = (MuContext IO -> IO LB.ByteString)

    data Config = Config {
          base :: String
        , port :: Int
        }

    getConfig (base:port:[]) = Config base (read port)
    getConfig _ = error "wrong number of arguments to hobo; use `hobo BASE_DIR PORT`"

    templateDir = "templates"

    readTemplatesOrError base = undefined

    i = 3 + 0

That's all, **folks!!**
