# pasta
Displays the pastafarian holiday for the current date. Written in rust and intended for use in a terminal greeting message.

### Usage
Simply run the binary and it will print the current holiday to standard out. For example, this may be the output if run on the 9th of June.

```bash
> pasta
Bathe in Marinara Day
```

You can add or change any holidays in the `holidays.csv` file to customise the output :thumbsup:.

### Building

Building `pasta` requires a copy of the rust compiler. You can install it using rustup which is available [here](https://www.rust-lang.org/tools/install).

Clone locally
```
git clone https://github.com/giraugh/pasta
```

Then build it
```
cd pasta
cargo build
```

Then, optionally, you can install it to your PATH.
```
cargo install --path .
```

After doing so you can use it in your terminal greeting scripts. Here is an example for fish shell.
```fish
function fish_greeting
  echo ""
  echo "Hello!"
  echo "Today is "(set_color blue)(pasta)(set_color normal)
  echo ""
end
```

### Contributing
If you encounter any issues or bugs, please leave an issue on Github. Additionally, all and any PRs are welcome :)


### Copyright
*Ewan Breakey - MIT 2021*
