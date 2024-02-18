1)Primitive Data Types
    a)Scalar
        i)Signed integer (i32: Default type for integers, can use negative integers)
          for i32 range: -((2^31)-1) to (2^31)-1
          i8,i16,i32,i64,i128

       ii)Unsigned integers (Can't use negative integers)
          for u32 range: 0 to (2^32)-1
          u8 to u128

      iii)float:f32,f64 (single and double precision, default:f64 )

       iv)bool:true(1),false(0)

        v)char:''

    b)Compound
        i)tuple: can be mutated with "mut" not with "let".
          can be printed by giving index
          we can't add more elements to tuple

       ii)array: we can't add elements to array or change Types
          we can mutate elements using "mut"
          we can explicitly define type & size or array syntax:- arr:[i32(type of array);5(length of array)]
          explicit array can't be empty array 
