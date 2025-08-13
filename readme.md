# Wraptify

A simple alternative client to the Spotify desktop app, which is basically a wrapper for https://open.spotify.com/. It is primarily developed for Windows, though you should be able to easily compile it for MacOS or Linux in its current state.

## Installation

Download the newest version from the "releases" tab, which is usually somewhere on the right of this page. There should be an .exe somehwere, which you can download and run. Note: it is recommended to put the .exe in an empty folder dedicated to this program, because it will most likely create another folder in the directory it's placed in.

## Features

- A built-in agent switcher, so you don't get ads asking you to download their desktop app
- Tracker blocking, to reduce RAM usage a bit

Below are some comparisons of this wrapper to the official desktop app:

### Pros

- Faster startup and browsing (about 2-3x from my testing)
- Always updated (because it's basically just a website)
- More customizable
- Smarter cache management (uses the one built into your system)
- Fully portable (if there are no permission issues)

### Cons

- No offline usage, so no downloads
- No quality control, though defaults to the highest available: 128kbps for free, 256kbps for premium users (which can sound better than the desktop app's 320kbps due to a more efficient codec (Vorbis vs AAC))
- Smart features like automix are missing (so certain albums' song transitions can sound worse)
- Resource usage is slightly higher (might be resolvable in a future update)
