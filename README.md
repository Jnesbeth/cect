Cloud Engineer Coding Test

# Problem Description

The objective is to simulate sending a large number of SMS alerts, like for an emergency alert service. The simulation consists of three parts:

1. A producer that that generates a configurable number of messages (default 1000) to random phone number. Each message contains up to 100 random characters.

2. A sender, who pickups up messages from the producer and simulates sending messages by waiting a random period time distributed around a configurable mean. The sender also has a configurable failure rate.

3. A progress monitor that displays the following and updates it every N seconds (configurable):

a. Number of messages sent so far

b. Number of messages failed so far

c. Average time per message so far

One instance each of producer and progress monitor will be started while a variable number of senders can be started with different mean processing time and error rate settings.

You are free in the programming language you choose, but your code should come with reasonable unit testing.

Please submit the code test at least two business days before the interview, so we have time to review it.

# Dependencies

The program is written in Rust and requires a Rust toolchain. You can install Rust at https://rustup.rs/.

# Installation

Clone the project and cd to its root directory. Then, use Cargo to build it.

```bash
$ cargo build
```

# Usage

You can execute the simulation by running the executable within the `target` directory after running `cargo build`, or with `cargo run` to build and run.

```bash
$ target/debug/cect
$ cargo run
```

There are defaults for all simulation parameters. To use other values, create a YAML configuration file and provide its path on the command line:

```bash
$ target/debug/cect config.yaml
```

```bash
$ cargo run -- config.yml
```

Output is to stderr and verbosity is controlled by the `RUST_LOG` environment variable. Upon execution, the current directory and its parents are searched for a `.env` file. There is a `.env` file that sets this to `RUST_LOG=Info` in the root of the tree.

# Configuration File

The configuration file is in YAML. Here is an example that demonstrates all options:

```yaml
num_messages: 1000
report_period: 2.0
senders:
  - { mean_time: 0.05, failure_rate: 0.50 }
  - { mean_time: 0.10, failure_rate: 0.45 }
  - { mean_time: 0.15, failure_rate: 0.40 }
  - { mean_time: 0.20, failure_rate: 0.35 }
  - { mean_time: 0.1, failure_rate: 0.3 }
```

# Theory of Operation

The simulation executes as a set of asynchronous tasks.

A single producer task sends `num_messages` random messages into a message channel.

A task is spawned for each configured sender. They consume from the channel that the producer sends to, each taking the next message when ready to send. After receiving a message to send, and until it is successfully sent, a sender implements a random delay drawn with a gamma distribution configured so that the expected mean delay is as configured, and with a shape chosen to provide a "nice looking", "bell curve-like" distribution. After the delay, the sender considers its configured failure rate and decides if the message transmission succeeds or fails. In either case, it transmits the result to the monitor through a channel. If transmission failed, then the sender loops back, chooses a new delay, and repeats the process from there until transmission succeeds. Once transmission succeeds, the sender loops back to pick up the next message to transmit.

A single monitor task consumes transmission success and failure results from the senders, keeping totals of them. Periodically, as configured, it displays a report of the quantities and the rate of successful transmission.

# Unit Tests

To run the unit tests, use Cargo:

```bash
$ cargo test
```

Cargo by default hides test output. It also runs tests on multiple threads. To execute tests on one thread and see output (if any):

```bash
$ RUST_TEST_THREADS=1 cargo test -- --nocapture
```
