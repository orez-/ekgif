# ekgif

![example.gif](media/result.gif)

Tool for creating animated gifs in the style of an EKG readout.

## Why
I needed exactly this, maybe you do too?

## Usage
```shell
ekgif media/bg.gif media/fg.gif > media/result.gif
```

| `media/bg.gif` | `media/fg.gif` |
| --- | --- |
| ![background_gif](media/bg.gif) | ![foreground_gif](media/fg.gif) |

Currently only accepts non-animated gifs of the same size.
The result of any other input is undefined.

Result gifs are not optimized.
It is recommended you optimize them yourself.

All other parameters are currently hardcoded.
To edit them you will need to compile from source.
