# showargs - Output each argument, bypassing redirection

This is a simple diagnostic tool that prints numbered command-line arguments to the console or terminal device.

This is not equivalent to using stdout or stderr. Even if all standard streams are redirected, this will still attempt to write to the console.

Windows and Unix-like systems are supported. On Unix-like systems `/dev/tty` is assumed available and used. On Windows, the console device is used and accessed via the device path `\\.\CON`. On either system, if there is not associated terminal or console, this will of course not work.

## What's it for?

Usually you want to print things to a standard stream, so the the good uses of this utility limited, but I use it to "mock out" external commands to observe how they are called even when the caller redirects or otherwise suppresses standard output and standard error. An alternative, in some situations, is of course better logging.

Note that in most cases on a Unix-like system, a script marked executable will do fine even if the usual executable is a binary, and that may be simpler than compiling something. On Windows, there are more situations where the treatment of binary executables and scripts (even batch files) differ.

## License

[0BSD](LICENSE)
