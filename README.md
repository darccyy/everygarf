# EveryGarf Comic Downloader

A Rust program to download every Garfield comic to date.

## Stats

- Download size: 4.8GB
- Download time: ~50m, from one test
- Images: >16,400

# Installation

Install from source with `cargo` 

```bash
git clone https://github.com/darccyy/everygarf
cd everygarf
./install
```

# Usage

```bash
# Help information
everygarf --help

# Download to default folder ('garfield' in user pictures directory)
everygarf

# Change some options
everygarf ~/Pictures/garfield -cq --attempts 20 --timeout 30
```

# About

## API

Since an official Garfield comic API could not be found, this program scrapes [gocomics.com](https://www.gocomics.com/garfield/1978/6/19), and finds the [assets.amuniversal.com](https://assets.amuniversal.com/aead3a905f69012ee3c100163e41dd5b) link. This requires 2 HTTP requests per comic. The files hosted at [picayune.uclick.com](https://picayune.uclick.com/comics/ga/1978/ga780619.gif), while only requiring 1 request each, have been found to be very inconsistent and unstable, therefore are not used.

## Speed

As mentioned above, since each image requires 2 HTTP requests, the program's speed is almost entirely dependent on internet speed. This program attempts to utilize as many CPU threads as possible. The only forseeable optimization to this program would be advanced parallelism, or using a different web API.

