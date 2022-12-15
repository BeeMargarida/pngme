# PNGMe

Simple CLI that lets users hide secret messages in PNGs. Made for learning purposes.

Access to the tutorial [here](https://picklenerd.github.io/pngme_book/introduction.html) and to the [PDF Spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html).

## Usage

#### Encode a message:
`./pngme encode <png path> <chunk type> <message> <output path (optional)>`

Ex: `./pngme encode ./image.png ruSt "psst, this is a secret"`

#### Decode a message:
`./pngme decode <png path> <chunk type>`

Ex: `./pngme decode ./image.png ruSt`

#### Remove a message:
`./pngme remove <png path> <chunk type>`

Ex: `./pngme remove ./image.png ruSt`

#### Print the chunks information:
`./pngme print <png path>`

Ex: `./pngme print ./image.png`
