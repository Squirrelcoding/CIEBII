# CIEBII
- Checks
- If
- Every
- Byte
- Is
- Intact

CIEBII is an image file format that checks if *every single byte* is intact. What does it do if it finds that a byte is out of place? Nothing! Because thats not its goal!

CIEBII is a significant upgrade from our previous project [temage](https://github.com/Squirrelcoding/temage)!

<center><img src="https://i.imgur.com/H36poDV.png" width="960px" height="540px" alt="comparision"/><br/><i>CIEBII rendering space photography</i></center>

Since it has to check if *every byte* is intact with a checksum per pixel, here is the space complexity shown:

<center>
    <img src="https://i.imgur.com/RPqX96V.png" width="960px" height="540px" alt="space comparision"/>
</center>

The image was this, in case you were wondering:

<center>
    <img src="https://i.imgur.com/VqzdOhc.png" width="457px" height="684px"/>
</center>


## CLI usage
Unlike our other projects, this time its cross-platform for Windows *and* Linux!

## `cib convert <file>`
Converts a PNG/JPG file into a `.cib` file. For example, if you do `cib convert my_image.png` it will spit out `my_image.cib`.

## `cib render <file.cib>`
Attempts to render a `.cib` file.