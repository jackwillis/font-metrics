# font-metrics

This package provides the binaries
`fm-cpp`, `fm-density`, and `fm-xheight`.

All binaries support the ``--help`` and ``--version`` options.

## Installation

Depends on Rust/Cargo and XeLaTeX.

    cargo install font_metrics

## Programs

### fm-xheight

Calculates the x-height/cap height ratio of a TrueType font.

    USAGE:
        fm-xheight.exe <FILENAME>

    ARGS:
        <FILENAME>    The location of the TrueType font to measure (ex. C:\Windows\Fonts\Tahoma.ttf)

    OUTPUT:
        x-height ratio: 1117/1489 (~0.750)

### fm-density

Measures the density of TrueType fonts.
Calculated from the amount inked between the baseline and x-height of lowercase Latin letters.

    USAGE:
        fm-density.exe <FILENAME>

    ARGS:
        <FILENAME>    The location of the TrueType font to measure (ex. C:\Windows\Fonts\Constan.ttf)

    OUTPUT:
        density: 0.397

### fm-cpp

Measures the characters per pica (cpp) of TrueType fonts on a standard test page.

    USAGE:
        fm-cpp.exe [FLAGS] [OPTIONS] <FILENAME>

    FLAGS:
        -v, --verbose    Prints extra debug messages

    OPTIONS:
        -s, --size <size>      Font size in points [default: 12]
        -w, --width <width>    Width of the test page's printable area in picas [default: 32]

    ARGS:
        <FILENAME>    The location of the TrueType font to measure (ex. C:\Windows\Fonts\Arial.ttf)

    OUTPUT:
        characters per pica: Ratio { numer: 727, denom: 352 } (~2.07)