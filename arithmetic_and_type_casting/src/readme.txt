u8 range 0-255 storing anything beyond this range will give overflow error also we cannot apply unary operator `-` to type `u8`.
float values require decimal values
we cannot perform operations on two different types together.

while explicit conversions in arithmatic operations type conversion to lesser type can lead to overflow which is not caught by the compiler.
string.trim() is used to remove next line character in order to avoid error while conversion.
type conversion string to integer type will give error when converting non-numeric value to integer.
Also it will give error if input is not specific type eg: defining integer and giving float value.