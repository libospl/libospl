[![CircleCI](https://circleci.com/gh/libospl/libospl.svg?style=shield)](https://app.circleci.com/pipelines/github/libospl/libospl)
[![Coverage](https://codecov.io/gh/libospl/libospl/branch/main/graph/badge.svg?token=53EckTgSg7)](https://codecov.io/gh/libospl/libospl)
[![GitHub milestone](https://img.shields.io/github/milestones/progress-percent/libospl/libospl/1)](https://github.com/libospl/libospl/milestones)
[![dependency status](https://deps.rs/repo/github/libospl/libospl/status.svg)](https://deps.rs/repo/github/libospl/libospl)
# libospl - Open Source Photo Library
----------------------------------------
### OSP what?
OSPL is a opensource and multiplateform photo library management that can be used to store and sort all your photos.

### Where is the GUI?
This project is split in two main parts:

* the library and CLI (this repo) who manages all the library, imports, exports the photos and do all the background things. All the info is stored in the database.db file at the root of the created library. This part is cross-plateform.
* The user interface, communicating with this library (or CLI) this permits flexible integration. The developement of this side of the project hasn't started yet. This part don't have to be cross-plateform so anyone who wants to make his own interface can do it, using libospl.

### Why?
Because I want to make a powerful photo library, an open source photo library and a working photo library.

### Features
OSPL is meant to be compatible with all photos types and metadata types, such as Apple Live Photos, slow motion videos and all other cool stuff you can do with your phone and photo cameras.
To see what's planned, you can look at [IDEAS.md](IDEAS.md)


### Contributing
Any help is welcome, see [CONTRIBUTING.md](CONTRIBUTING.md) to know how to contribute.


### Build
just type `cargo build`

