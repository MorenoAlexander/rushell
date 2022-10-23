

/*
Need to handle stdin with various command splitters

| - splits the command and pipes the output of the left operand to the right

echo "c1" | echo

\ - does not split the command but simply outputs the stdin and creates a new line for the user to continue writing it out

echo "c1" \ "c2"

&& - runs the second command IIF the first command executes successfully

echo "c1" && echo "c2"

; - separates both commands independantly of each other in order

echo "c1" ; echo "c2"


> overwrites output of command into file

echo "This is a test!" > output.txt

```
This is a test!
```

>> appends stdout to file along with any additional text

echo "This is a test!" >> output.txt

```
This is a test!This is a test!
```


*/
use std::{process::{Command, Output}};



pub enum CommandType<'a> {
    None(CommandInput<'a>),
    Pipe(CommandInput<'a>),
    Ampersand(CommandInput<'a>),
    Semi(CommandInput<'a>)
}


#[derive(Debug)]
pub struct CommandInput<'a> {
    pub command: &'a str,
    pub args: Vec<&'a str>
}

impl<'a> CommandInput<'a> {
    pub fn new(mut input_split:Vec<&'a str>) -> Self {
        let command = input_split.remove(0);
        CommandInput { command, args: input_split }
    }

    pub fn execute(&self) -> Result<Output, std::io::Error> {
        Command::new(self.command).args(&self.args).output()

    }

}
