Kicad CLI
===

Automate common tasks that Kicad doesn't expose through better means by running
their GUI tools, performing a sequence of actions and hoping we get the
intended result.

This method is very finicky and unreliable:
* It depends on hard-coded timeout values that may be way off.
* It relies on certain behaviours of the windowing environment, such as keeping
  the focus on a certain window.
* It may rely on some prior configuration of Kicad so certain unexpected
  dialogs don't interfere with our logic.
* It relies on a specific version of Kicad having the UI configured in a
  specific way.
* It relies on another version of Kicad's GUI tool not running at the time of
  running this script.
* It relies on the locale and possibly other environment variables to be set
  just right.
* It relies on no other instance of eeschema (for erc) or pcbnew (for drc)
  running at the same time
* It relies on the absence of user input (mouse or keyboard) while the task is
  running

You get the point: this is not reliable in any way.

To mitigate all of the above issues, there is a `--headless` flag that allows
to run without an X Display server.
It is highly recommended to run from a docker environment, where there won't be
interference from the outside world with the `--headless` flag to run within an
xvfb fake display server.

Dependencies
===

This program uses existing software for most of the functionality.
These executables will need to be installed and in the path prior to using
kicad_cli:
* for `run-erc`
  * `eeschema` (from `kicad` package)
  * `xvfb` if running headless
* for `run-drc`
  * `pcbnew` (from `kicad` package)
  * `xdotool`
  * `xvfb` if running headless

Usage
===

Basic usage:
---

See `kicad_cli --help`, `kicad_cli run-erc --help` or
`kicad_cli run-drc --help` for details.

The basic usage is as follows:
```
kicad_cli run-erc path/to/schematic_file.sch --headless
```
to run the Electrical Rule Checker in a headless environment.
```
kicad_cli run-drc path/to/board_file.kicad_pcb --headless
```
To run the Design Rule Checker in a headless environment.

Not passing the `--headless` flag will mean you will be able to see what's
happening in your windows environement. It also means it's more likely to fail
for some arbitrary reason, such as your breathing too loud or moving your mouse
around and stealing the focus from where the script expects it. You've been
warned ;)

With docker (recommended):
---

From this repo:
```
docker build -t kicad_cli .
```
Then run kicad_cli in docker:
```
docker run -v /absolute/path/to/your/pcb/project:/workdir -t kicad_cli kicad_cli run-erc path/to/board.sch --headless
```
or
```
docker run -v /absolute/path/to/your/pcb/project:/workdir -t kicad_cli kicad_cli run-drc path/to/board.kicad_pcb --headless
```

Troubleshooting:
---

If the script is not behaving as expected. You may increase the information
being logged by setting the `RUST_LOG` environment varible to one of the
following values: `debug`, `error`, `info`, `warn` or `trace`.
For instance, run this command:
```
docker run -v /absolute/path/to/your/pcb/project:/workdir -t kicad_cli bash -c "RUST_LOG=trace kicad_cli run-erc path/to/board.sch --headless"
```

Another useful technique is to run headless on docker, but record the screen to
be able to see what's going on with the UX.
It can be done as follows:
```
docker run -v /absolute/path/to/your/pcb/project:/workdir -ti kicad_cli bash
pacman -Syy
pacman -S recordmydesktop
Xvfb :99 -ac -nolisten tcp&
export DISPLAY=:99
recordmydesktop --no-sound --no-frame --on-the-fly-encoding -o recording&
kicad_cli run-erc path/to/board.sch
killall recordmydesktop
killall Xvfb
```

You should then find the file: recording.ogv in your pcb project's directory.
Open it with `mpv`, `vlc` or your favourite video player.

Prior art
===
This work is heavily inspired by prior work from the Productize, Scott Bezek
and the splitflap contributors.
Specifically, this repository:
https://github.com/productize/kicad-automation-scripts
It is basically a rewrite of a subset of these scripts' functionality in rust.

I wrote my own version in rust to avoid having to use python2 and having
to deal with pip or equivalent to pull in a number of python2 dependencies.
I also had some issues with non-reproducibility with that solution and one
goal here is to make the code work reliably in a specific docker environment
so it can be used in CI without having to deal with flakyness issues.
