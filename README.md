<h1 align="center" class="header">Randy</h1>
<p align="center" class="desc">
  Rust web app with a bunch of random (but probably useful) tools
</p>

## Table of Content
- [Introduction](#introduction)
- [Features](#features)
- [Contributions](#contributions)
- [Installation](#installation)
- [FAQ](#faq)
- [Credits](#credits)

## Introduction

[![forthebadge](https://forthebadge.com/images/badges/built-with-love.svg)](https://forthebadge.com)

![GitHub Repo stars](https://img.shields.io/github/stars/HoangTuan110/randy) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat)](http://makeapullrequest.com)

Randy is Rust web app with a bunch of random tools.

## Features

* Terminal-friendly (Can be used easily using `curl`)
* Built with Rust
* Self-hostable
* Tools include:
  * Random number generator (`/rand/`)
  * Display current time using (`/now/`)
  * Unique ID generator using (`/id/`)
  * Owoify text (`/owo/`)
  * Display IP address (from `https://httpbin.org/ip`) (`/ip/`)

## Contributions

Contributions are welcome. If you found any bugs or want to requrest a feature, please open an issue.

## Installation

* First, install the Rust compiler and its package manager, `cargo`, at [here](https://www.rust-lang.org/tools/install)
* Next, clone this repo and `cd` to it:
  ```sh
  git clone https://github.com/HoangTuan110/randy
  ```
* Then, run:
  ```sh
  cargo run
  ```
  This will install all the dependencies required to run Randy and start the server.
* Finally, you should see this message:
  ```
  Choo Choo! Listening at 0.0.0.0:8000!
  ```
  Then you can start `curl`ing Randy:
  ```sh
  curl 0.0.0.0:8000/
  ```

## FAQ

## Credits

This app uses the following libraries:

* [warp](https://github.com/seanmonstar/warp) as the web framework
* [rand](https://github.com/rust-random/rand) to generate random numbers
* [chrono](https://github.com/chronotope/chrono) as the date time library
* [nanoid](https://github.com/nikolay-govorov/nanoid) to generate unique ID
* [owoify](https://github.com/AgathaSorceress/owoify) to owoify text
* [ureq](https://github.com/algesten/ureq) as the HTTP client
* [serde_json](https://github.com/serde-rs/json) to convert raw JS data into Rust objects

Thanks [README Templates](https://www.readme-templates.com) and [GitPoint's README](https://github.com/gitpoint/git-point#readme) for the template. This project uses GitPoint's README template.

Thanks [this TOC generator](https://ecotrust-canada.github.io/markdown-toc/) for the TOC (Table Of Content).
