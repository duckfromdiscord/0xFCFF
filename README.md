# FCFFh (formerly known as `0xFCFF` before GitHub told me I can't use numbers as the first character in the repo name)
FCFFh is a simple "music player"-type interface for BizHawk that runs with a Lua script and a Rust program and shows you the currently playing song in your game.

## How do I set up FCFFh?
Simply pick a port for the Rust "server", and put your server IP and port in the included Lua script. Open up the FCFFh server. \
Setting up BizHawk is a bit more complicated, but not by much. It's recommended to make a batch file to run it with; to both enable the HTTP client and to have the Lua script run automatically.
`EmuHawk --url_get=http://localhost:8000 --url_post=http://localhost:8000`

## How does FCFFh work?
In the `games` folder, there are a bunch of JSON configuration files. These direct the client to the address in the game system's memory that contains some sort of value identifying the song. These configuration files also include a dictionary that maps these values to the name of the song.

## Configuration Files and Conventions
Here's a template for a configuration file:
```json
{
    "system": "NDS",
    "hashes": [
        "UPPERCASE SHA-1 ROM HASH"
    ],
    "names": [
        "UPPERCASE ROM FILENAME EXCLUDING EXTENSION"
    ],
    "default_info": {"artist": "Some Game Developer", "album": "Some Game"},
    "addr": "0x111111",
    "songs": {
        "1": {"title": "Song 0x1 Title"}
    },
    "note": [
        "Any",
        "comments",
        "you",
        "want."
    ]
}
```
\
The `note` section is ignored by the parser, but it is recommended to put a couple things there: **some sort of indicator of ANY missing songs/song IDs**, any alternate memory addresses or addresses of interest, sources for song IDs if there is a list of them already out there, and the author of the config,