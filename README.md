# EveryGarf Downloader

A Rust program to download every Garfield comic to date.

# Stats

- Download size: 4.8GB
- Download time: ~50m, from one test
- Images: 16,400

# TODO

- download todays date (as opposed to yesterdays)
    - must check for time, lest same image is downloaded with different name (different apparent date)
    - check if current time is after a certain time (utc)
    - if yes: do todays as well
    - if no: only do up to yesterday

- add tests :(

- Create folder if not exist?
- Ignore 'clean' if folder not exist?

