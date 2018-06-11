# font-metrics

## fm-xheight

Calculates the x-height/cap height ratio of a TrueType font.

    USAGE:
        fm-xheight.exe <FILENAME>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    ARGS:
        <FILENAME>    The location of the TrueType font to measure (ex. C:\Windows\Fonts\Tahoma.ttf)

## fm-density

Measures the density of TrueType fonts.
Calculated from the amount inked between the baseline and x-height of lowercase Latin letters.

    USAGE:
        fm-density.exe <FILENAME>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    ARGS:
        <FILENAME>    The location of the TrueType font to measure (ex. C:\Windows\Fonts\Constan.ttf)

## fm-cpp

Measures the characters per pica (cpp) of TrueType fonts on a standard test page.

    USAGE:
        fm-cpp.exe [FLAGS] [OPTIONS] <FILENAME>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
        -v, --verbose    Prints extra debug messages

    OPTIONS:
        -s, --size <size>      Font size in points [default: 12]
        -w, --width <width>    Width of the test page's printable area in picas [default: 32]

    ARGS:
        <FILENAME>    The location of the TrueType font to measure (ex. C:\Windows\Fonts\Arial.ttf)
