# Xmobet

Slot machine simulator designed for use in menubars. Originally intended for
`xmobar`, hence the name.

## Installation

```sh
cargo install xmobet
```

Or like everything else with git:

```sh
git clone https://github.com/impossibletea/xmobet.git
cd xmobet
cargo install --path .
```

## Usage

### Getting your bar to recognize it

Xmobar:

```haskell
Config
  { template = "}<action=`killall -SIGCONT xmobet` button=1><action=`killall -SIGUSR1 xmobet` button=4><action=`killall -SIGUSR2 xmobet` button=5><action=`killall -SIGINT xmobet` button=3>%xmobet%</action></action></action></action>{"
  , commands = 
    [ Run CommandReader "~/.cargo/bin/xmobet" "xmobet"
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

Default config on linux is `~/.config/xmobet/config.toml`. Structure (with
default values) is as follows:

```toml
[account]
init_balance = 100
init_bet = 5
bet_inc = 1

[slots]
drums = 5
symbols = '7JQKA'
```

