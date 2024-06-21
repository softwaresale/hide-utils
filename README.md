
# Hider

One of the silliest things I've ever made. It's entire purpose is to hide and
unhide linux files. For example:

```bash
$ ls
file1 file2 file3
$ hider file1
$ ls
file2 file3
$ ls -a
.file1 file2 file3
$ hider .file1
$ ls
file1 file2 file3
```
In short, if a file does not start with a '.', then it will make that file
start with a '.' If the file does start with a '.' then it will be removed.
If you try to hide/unhide a file that is already hidden, it'll find it for you.

Usage:
```
A simple command line utility for hiding or un-hiding linux files.
Provide a file to hide or un-hide and this tool will do just that.
A file is hidden if it starts with a "hide character", which is '.'
by default. A file is un-hidden when it does not start with the
hide-character.

Usage: hider [OPTIONS] <FILE>

Arguments:
  <FILE>  The file to hide/unhide

Options:
  -v, --verbose                If turned on, verbose logging is enabled
  -c, --hide-char <HIDE_CHAR>  Character to append/remove from the front of the filename [default: .]
  -u, --unhide                 force unhide
  -i, --hide                   force hide
  -h, --help                   Print help
  -V, --version                Print version
```
