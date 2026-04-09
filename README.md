# c311-project-key-value-tranaction-engine

#How To Run
Within the c311-project-key-value-transaction-engine/transaction-engine/src/ folder contain main.rs and its modules and sample text files.
In a terminal type cargo run [files seperated by a space including the extension]
Recomended Example:
cargo run simplefile.txt simplefile2.txt

Each file is discrete in its operations.
As each file is processed sequentially, each will output the results of lexing, parsing, validating, and executing the contents of each file.

Syntax:

Each Statement must end in a SINGLE semicolon.

Each Keys's identiifier must be alphanumeric with the first character as alphabetic.

There are NO numerics in this language.

Values passed into the Key must be strings delimted by "" Quotes. Quotes within quotes aren't supported.

Within the quotes any valid standard character is allowed.

BEGIN - Begins transaction block.

END - Ends transaction block.

COMMIT - Clears rollback and establishes baseline.

ABORT - Rollsback to BEGIN or last COMMIT.

SET - Alters or Creates a Key-Value within Key-Value Space.

    SET \[KEY\] \[VALUE\]
GET- Retrieves Value from a specified 

    GET \[KEY\]

DEL- Removes Key-Value pair from Key-Value Space.

    DEL \[KEY\]

Each transaction block begins with a BEGIN; statement.

Each transaction block ends with an END; statement.

Referencing for DEL or GET non existent keys will throw and error.

    Referencing a key after deletion will do the same
