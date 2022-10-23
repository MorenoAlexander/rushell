
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

*/



#[derive(Debug)]
pub struct CommandInput<'a> {
    pub command: &'a str,
    pub args: Vec<&'a str>
}

impl<'a> CommandInput<'a> {
    pub fn new(mut input_split:Vec<&'a str>) -> Self {
        let command_str = input_split.remove(0);

        CommandInput { command: command_str, args: input_split }
    }

}
