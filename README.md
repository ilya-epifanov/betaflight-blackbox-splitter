# Betaflight blackbox log file splitter

## Build status

| Snapcraft | [![Snap Status](https://build.snapcraft.io/badge/ilya-epifanov/betaflight-blackbox-splitter.svg)](https://build.snapcraft.io/user/ilya-epifanov/betaflight-blackbox-splitter)|
-- | --
| Travis | [![Linux and MacOS build status](https://travis-ci.org/ilya-epifanov/betaflight-blackbox-splitter.svg?branch=master)](https://travis-ci.org/ilya-epifanov/betaflight-blackbox-splitter)|
| Appveyor | [![Windows build status](https://ci.appveyor.com/api/projects/status/mtq4w3fd6dqqglcg/branch/master?svg=true)](https://ci.appveyor.com/project/ilya-epifanov/betaflight-blackbox-splitter/branch/master)|

# Installation

## Snap-enabled Linux installations

If you have `snapd` installed (by default on Ubuntu and derivatives), you can install the tool from snapcraft.io global store.

```
snap install bfbb-split
```

## Others (Linux without snap, MacOS, Windows)

Download an archive from [release page](https://github.com/ilya-epifanov/betaflight-blackbox-splitter/releases), place the binary from inside the archive where you see fit.

# Usage

## Command-line

```
$ cd _YOUR_BLACKBOX_DIRECTORY_
$ ls
BTFL_BLACKBOX_LOG_Copty_20180128_192226.BBL
RC_VID_0000.MOV
$ bfbb-split BTFL_BLACKBOX_LOG_Copty_20180128_192226.BBL
$ ls
BTFL_BLACKBOX_LOG_Copty_20180128_192226.000.BBL
BTFL_BLACKBOX_LOG_Copty_20180128_192226.001.BBL
BTFL_BLACKBOX_LOG_Copty_20180128_192226.002.BBL
BTFL_BLACKBOX_LOG_Copty_20180128_192226.003.BBL
BTFL_BLACKBOX_LOG_Copty_20180128_192226.004.BBL
BTFL_BLACKBOX_LOG_Copty_20180128_192226.005.BBL
BTFL_BLACKBOX_LOG_Copty_20180128_192226.006.BBL
BTFL_BLACKBOX_LOG_Copty_20180128_192226.007.BBL
BTFL_BLACKBOX_LOG_Copty_20180128_192226.008.BBL
BTFL_BLACKBOX_LOG_Copty_20180128_192226.009.BBL
BTFL_BLACKBOX_LOG_Copty_20180128_192226.010.BBL
BTFL_BLACKBOX_LOG_Copty_20180128_192226.BBL
RC_VID_0000.MOV
```

## Windows shortcut

Create a shortcut to the program binary.
Drop a `.bbl` file on the shortcut, individual parts will be placed next to the original `.bbl`.
