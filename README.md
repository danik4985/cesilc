<p style="text-align: center">
	<img src="https://miro.medium.com/max/1400/0*AJK4a0G1gaknToSP.jpg" alt="cesil banner" style="height: 250px; width: auto">
</p>

# CesilC

> CESIL, or Computer Education in Schools Instruction Language, is a programming language designed to introduce pupils in British secondary schools to elementary computer programming. It is a simple language containing a total of fourteen instructions. 

[![license: GPLv3](https://img.shields.io/badge/license-GPLv3-blue.svg)](COPYING) [![version: 1.0.0](https://img.shields.io/badge/version-1.0.0-green.svg)](README.md) [![build: linux and shitdows only](https://img.shields.io/badge/build-linux%20and%20shitdows%20only-yellow.svg)](README.md) [![amogus: sus](https://img.shields.io/badge/amogus-sus-black.svg)](README.md)

Ladies and gentlemen, [CESIL](https://en.wikipedia.org/wiki/CESIL) time. That's right, I did it again! And this time it works (maybe).

47 years later, introducing a brand new, fast, epic cesil compiler.

## Installation
> There is currently no binary build for macos. That is because linux -> mac cross-compilation fails with some long and chaotic error message and It would be super complicated to fix and I don't want to have to read the Xcode license and I'm sure it would make me give all my copyright rights to Apple [...]

There are binaries and installation scripts for Linux and Shitdows in releases. On shitdows running the installation script should be enough. On linux, you need to run it with `sudo`!

## Syntax
I was not able to find any official specifictions, so I had to go mainly by the wikipedia article. When it was first created, cesil was punched onto punch cards. This means that the syntax written in a text file on a computer must be modified a little. Here is what I did:
- You are not forced to use columns
- If a line starts with `(`, it is a comment
- Empty lines and comments are ingored
- If a line starts with an identifier, then the first word is considered a label, word 2 is condidered the command and word 3 (if it exists) is considered the argument
- If the line starts with a whitespace, the first word is considered the command and the second one the argument

This means that the following program:
```
        LOAD    +0
LOOP    STORE   TOTAL
        UIN
        JINEG   DONE
        ADD     TOTAL
        JUMP    LOOP

DONE    PRINT   "The total is: "
        LOAD    TOTAL
        OUT
        LINE
        HALT
```

is just as valid as
```
 LOAD +0
LOOP STORE TOTAL
 UIN
 JINEG DONE
 ADD TOTAL
 JUMP LOOP

DONE PRINT "The total is: "
 LOAD TOTAL
 OUT
 LINE
 HALT

```

Data section starts with `%` and ends with `*`

## Compilation targets

The compiler will detect the target you want from the output file extension.

### Current
- [x] C (.c)
- [x] native binaries (No extension [linux]/.app [MacOS]/.exe [Shitdows])
- [x] Bash (.sh)

### Planned
- [ ] Java
- [ ] JavaScript
- [ ] APKs (using java)
- [ ] Batch files (MS-DOS compatible)

Dm me on discord if you want to request a compilation target!

## API coverage
- This implementation covers 100% of cesil api.
- A new instruction is added: `UIN`, this reads a number from stdin and saves it to the accumulator. This is for interactivness of your binaries and won't compile unless you use the *interactive* flag.

## CLI usage

The CLI usage goes as follows
```
cesilc <INPUT> -o <OUTPUT> [Options]

-o, --output      : The output file
-i, --interactive : If this flag is used, you can use the `UIN` instruction
-O, --optimize    : Optimize the output (WIP)
```

## Roadmap
- [ ] Catch some errors at compilation time
- [ ] Error handeling so the compiler doesnt ever crash
- [ ] More compilation targets
- [ ] ...

## VsCode cesil highlighting?
Coming whenever someone makes one or I learn how to make vscode language extensions.
Btw, I'm thinking I will make my own CESIL IDE, since studio for visual programming by micronesian software isn't ideal for this.

## Additional questions
If you have any questions or want to contact me for any reason, add me on discord: `danik#4985`

## Development
You can make your own compilation targets, just have a look at the source.
You can use `make_release.sh` to generate binaries for all platforms.

## License
This program uses the GPLv3 license. See its full text in the [COPYING](COPYING) file!

```
Cesil compiler/transpiler
Copyright (C) 2021 danik4985

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

## Notes
- Compilation to mac and shitdows binaries isn't tested. In case it doesnt work, compile to c source and use your compiler to compile that. (And report it to issues).
- To compile to binaries, you need to have `gcc` installed.
- If you encounter any problems, please report them in the *issues* tab.