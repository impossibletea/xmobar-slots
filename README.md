# Xmobet

A small tool intended to use with menubars.

## Installation

Like everything else with git:

```sh
git clone https://github.com/impossibletea/xmobar-slots.git
cd xmobar-slots
cargo install --path .
```

… or, when I finally publish it like a grown up boy:

```sh
cargo install xmobar-slots
```

## Usage

### Getting your bar to recognize it

Xmobar:

```haskell
Config
  { template = "}<action=`killall -SIGCONT xmobet` button=1><action=`killall -SIGUSR1 xmobet` button=4><action=`killall -SIGUSR2 xmobet` button=5><action=`killall -SIGINT xmobet` button=3>%xmobet%</action></action></action></action>{"
  , commands = 
    [ CommandReader "~/.cargo/bin/xmobet" xmobet
    ]
  }
```

For other bars you'll have to figure it out yourself.

### Actual usage

Since it is made for bars rather than stdin, it makes use of signals.

`SIGCONT`: Roll the slots, get money for combinations.

`SIGUSR{1,2}`: Increase or decrease bet.

`SIGINT`: Show current balance.

## Configuration

TBA

