Lexi Velas Programming Assignment 1
I used Rust for this assignment

To compile from the makefile, run:

make

From there, the program is run like this on the command line, with brackets denoting parts to be replaced by the user:

./happyQR [options] "[Text to encode, spaces allowed as long as in quotes]"

There are two options, --dark or --light, with --dark pertaining to the black/dark pixels of the qr code, and light to the white/light ones.

--[dark|light] col [percent] [0-255] [0-255] [0-255]\n
--[dark|light] img [percent] [pathToFile]\n
--[dark|light] pat [percent] [jgraphPatternString]\n

percent: Percentage
    the amount of the dark or light pixels you would like to be your specified color, image, or pattern. The percentages specified don't need to add up to 100, as they just subtract from the existing pixel's percentage, but as such the total percentages given as options must not exceed 100. Dark and light both have separate percentages

col: Color
    The three parts after percent are the amounts of red, green, and blue respectively, each denoting 0-255 of each color, with white being 255 255 255 and black being 0 0 0

img: Image
    Pixels can be replaced with images. pathToFile is the path to the image you would like to insert as pixels. Please ensure these are png, jpeg, svg, or anything else able to be converted by the convert command to eps file format.

pat: Pattern
    A jgraph pattern can be used in place of flat colors. the part you supply is what's in brackets in this example pattern , the start and pts at the end are provided:
        newline poly [\"linethickness 5 color 1 1 0 pcfill 1 0 1 ppattern stripe 60\"] pts  4 4


For patterns, the part you supply is what's in brackets in this example pattern , the start and pts at the end are provided:
    newline poly "[linethickness 5 color 1 1 0 pcfill 1 0 1 ppattern stripe 60]" pts  4 4  4 6  6 6  6 4


An example of a valid command, as long as temp.png is a file in the current directory:

./happyQR --light col 50 200 100 200 --light col 50 250 250 0 --dark img 50 temp.png --dark pat 50 "linethickness 1 color 1 1 0 pcfill 1 0 1 ppattern stripe 60" "This Is A Valid Command"

This, however, is an example of a qr code that does not work, as it's pixels have been changed too far to be able to be recognized by a scanner. Below is one with a few color changes that still functions. To work properly, lights need to be close to white, and darks need to be close to black.

./happyQR --light col 100 250 240 250 --dark col 50 50 0 100 "question mark?"