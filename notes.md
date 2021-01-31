**String vs &str in structs**
from [this reddit comment](https://www.reddit.com/r/rust/comments/5t5zq1/when_to_use_str_over_string_in_a_struct/ddkf5t1/)
Do you want the string to be owned by or to be borrowed by BaseAtom? That is, do you want it to live inside your struct, or to live somewhere else and only be referred to by your struct? In the former case, you'd use String.

Alternatively, if it will only ever contain string literals that are known at compile time, then you can use &'static str. That's a special lifetime that says it will live forever, and so lets you simplify your code a bit.