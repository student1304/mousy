# Mousy
Short script to move the mouse over the screen in random movements.
## Building, Installing
``` 
cargo build --release
```
or install to your path with
``` 
cargo install --path .
```

## Usage
From Terminal 
```
mousy <x_max> <y_max> -s <seconds>
```
All arguments are optional. 

x_max defaults to 1200
y_max defaults to 800
seconds default to 3600 (1hr)

stop with CTRL-C if done befor
