
//Rust function to convert a number to binary using recursion

fn dec2bin(num:i32)->i32
{
    if num == 0
    {
        return 0;
    }
    else
    {
        return num % 2 + 10 * dec2bin(num / 2);
    }
}