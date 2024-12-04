<<<<<<< HEAD
=======
(function_item
  name: (identifier) @function)

(type_identifier) @type
(primitive_type) @type

(string_literal) @string
(raw_string_literal) @string
(char_literal) @string

(integer_literal) @number
(float_literal) @number

(line_comment) @comment
(block_comment) @comment

[
  "fn"
  "let"
  "const"
  "struct"
  "enum"
  "trait"
  "impl"
  "pub"
  "mut"
  "return"
  "match"
  "if"
  "else"
  "for"
  "while"
  "loop"
  "break"
  "continue"
] @keyword

(parameter
  pattern: (identifier) @parameter)

[
  "+"
  "-"
  "*"
  "/"
  "="
  "=="
  "!="
  ">"
  "<"
  ">="
  "<="
  "&"
  "|"
  "^"
] @operator
>>>>>>> Snippet
; Keywords
[
  "as"
  "break"
  "const"
  "continue"
  "crate"
  "else"
  "enum"
  "extern"
  "fn"
  "for"
  "if"
  "impl"
  "in"
  "let"
  "loop"
  "match"
  "mod"
  "move"
  "mut"
  "pub"
  "ref"
  "return"
  "self"
  "Self"
  "static"
  "struct"
  "super"
  "trait"
  "type"
  "unsafe"
  "use"
  "where"
  "while"
  "async"
  "await"
  "dyn"
] @keyword

; Functions
(function_item name: (identifier) @function)
(call_expression function: (identifier) @function)
(generic_function (identifier) @function)
(macro_invocation macro: (identifier) @function)

; Types
(type_identifier) @type
(primitive_type) @type
(generic_type (type_identifier) @type)

; Strings
(string_literal) @string
(raw_string_literal) @string
(char_literal) @string

; Numbers
(integer_literal) @number
(float_literal) @number

; Comments
(line_comment) @comment
(block_comment) @comment

; Parameters
(parameter pattern: (identifier) @parameter)
(closure_parameters (identifier) @parameter)

; Variables
(identifier) @variable

; Operators
[
  "+"
  "-"
  "*"
  "/"
  "="
  "=="
  "!="
  ">"
  "<"
  ">="
  "<="
  "&"
  "|"
  "^"
  "&&"
  "||"
  "+="
  "-="
  "*="
  "/="
  "%="
  "&="
  "|="
  "^="
  "!"
  "~"
] @operator

