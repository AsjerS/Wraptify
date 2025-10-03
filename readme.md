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
- Smarter cache management (uses the manager built into your system)
- Fully portable (if there are no permission issues), so no real installation or admin rights required
- Comparable quality to the desktop app at lower bitrates (so a smaller cache size and less data usage):

|  | Web/Wraptify | Desktop (Lossy) | Desktop (Lossless) |
|---|---|---|---|
| Free | 128 kbps AAC (≈192 kbps MP3) | 160 kbps Vorbis (≈192 kbps MP3) | N/A |
| Premium | 256 kbps AAC (≈384 kbps MP3) | 320 kbps Vorbis (≈384 kbps MP3) | 2000 kbps FLAC |

### Cons

- No offline usage, so no downloads
- No quality control, so with bad internet you might be forced a lower quality stream
- Smart features like automix are missing (so certain albums' song transitions can sound worse)
- Resource usage is slightly higher (might be resolvable in a future update)
