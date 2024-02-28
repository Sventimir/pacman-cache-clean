pacman-cache-clean
==================

The name is probably quite self-explanatory. As
[pacman](https://wiki.archlinux.org/title/pacman) unfortunately does
not ship with an easy method of intelligent cleaning the package
cache, this program tries to compensate for it. Package cache comes in
very handy if a system upgrade breaks something and there comes a need
to revert some or all of the upgraded packages to their previous
versions. On the other hand, just left as is, the package cache grows
over time and consumes more and more disk space.

This automatic cleaner takes the simplest approach except, perhaps, for
mindlessly deleting the whole cache. It lists the package cache directory
(`/var/cache/pacman/pkg`) for all package files, groups them by package
they belong to and for each package keeps the most recent version and
deletes the rest.

Determining the version from the file name is quite tricky, because not
all packages are versioned by semver; in some cases hashes are used, which
cannot be reasonably compared. Therefore, rather than parse versions from
filenames, this program simply looks at files' creation time, assuming that
the most recently created file is also the most recent version (which should
always be true unless user creates files in the cache directory manually).

Building
--------

    $ cargo build --release
    
Installation
------------

    $ sudo cp target/release/pacman-cache-clean /usr/local/bin
    
Of course any other location in `$PATH` will do just as well.

Usage
-----

As of now the program does not take any arguments or configuration.
Simply run the binary. Don't forget to give it permission to write
to the package cache directory (which usually boils down to running
the program as root).

The program will print out each file name it deletes. If the output
is clean, no packages were deleted at all.
