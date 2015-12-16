fn main() {
    // create immutable greeting array length 13 type char
    // because simple hello world is boring :P
    let greeting: [char; 13] = 
      ['H','e','l','l','o',',','w','o','r','l','d','!','\n'];
    // create iterator over array
    let greeting_iter = greeting.iter();
    // iterate over greeting via for loop
    for i in greeting_iter {
        print!("{}", i);
    }
}
