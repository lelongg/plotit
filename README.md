# Plotit

## Overview

Plotit allows live visualization of multiple time series read from terminal.

Plotit consumes standard input formatted as CSV and will build a trace for each field.
Plotit open a new tab in your browser in which a [plotly](https://plot.ly/) graph is continuously updated.

This is **experimental** software.
It was made as an experiment to build a self contained client and server communicating through websocket fully in Rust.

The port *9001* is used internally for the websocket and the port *8000* is used for the [rocket](https://rocket.rs/) server.
These ports cannot be changed at this time.

## Quickstart

This repository comes with a Makefile allowing to build and run Plotit easily.  
Type `make` and go take a look at your browser.
A new tab should display live plotting of sine waves.

![screenshot](screenshot.png)

This example is using `noise` program output as an input for plotit.
The `noise` program is built by this repository and continuously output sine waves samples to standard output.

```
0.7173560908995227, 0.6967067093471655
0.7833269096274833, 0.6216099682706645
0.8414709848078964, 0.5403023058681398
0.8912073600614353, 0.4535961214255775
0.9320390859672263, 0.3623577544766736
0.963558185417193, 0.26749882862458735
```

## Usage

```bash
my_program | plotit
```

Go to your browser at [localhost:8000](http://localhost:8000).