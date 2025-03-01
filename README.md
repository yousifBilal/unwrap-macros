# unwrap-macros
This tiny library provides unwrapping macros in situation where the typical unwrapping methods for Result and Option in the standard library comes short and the alternative is too verbose. Specifically when you want to have the `unwrap_or_else` logic but need to `continue` or `return`.
