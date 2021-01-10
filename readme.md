Kicad Automation
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

You get the point: this is not reliable in any way.

I really want to automate these steps, though so I'm willing to leave with
these constraints.

To mitigate most of the issues above, it is recommended to run headless with
a tool such as xvfb (removes influence from the windowing environment) and to
run in a docker container (removes influence from environment variables, other
software running or installed, etc.)

WIP
===

This work is a very early work in progress.

TODO:
* Better error handling
  - In particular, informative error messages for
    - Missing eeschema executable (install kicad)
	- Path provided doesn't appear to be a valid kicad file
    - Gui not responding as we expected
* More configurability, in particular for the timeouts
* Run drc too
* Optionally run headless by running xvfb
* Publish a Dockerfile for inspiration

Prior art
===
This work is heavily inspired by prior work from the Productize, Scott Bezek
and the splitflap contributors.
Specifically, this repository:
https://github.com/productize/kicad-automation-scripts

I wrote my own version in rust to avoid having to use python2 and having
to deal with pip or equivalent to pull in a number of python
dependencies.

