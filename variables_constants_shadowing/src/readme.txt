1) by default all variables are immutable.
2) use "mut" to make variables mutable.
    a)we get warnings if no need to use mut.
    b)we get warning when value assigned is never needed.
3)we can re-cdeclare variables to mutate values without using "mut".
4)Shadowing: variablrs in a scope can use outer variable values but its not vice-versa.
5)when using let we can re-declare & redefine data type. Using "mut" does not allow us to redefine data type.
6)constants cannot be re-declared or mutated.