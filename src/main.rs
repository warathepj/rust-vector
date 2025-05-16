fn main() {
    // 1. Creating a new, empty vector
    let mut v: Vec<i32> = Vec::new();

    // 2. Adding elements to a mutable vector using push()
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Print the vector
    println!("After pushing elements: {:?}", v);

    // 3. Creating a vector with initial values using the vec! macro
    let mut v2 = vec![1, 2, 3, 4, 5];

    // Print the second vector
    println!("Vector created with vec! macro: {:?}", v2);

    // 4. Accessing elements using indexing (returns a reference)
    let third_element = &v2[2];
    println!("The third element of v2 is: {}", third_element);

    // 5. Accessing elements using the get() method (returns an Option<&T>)
    match v2.get(4) {
        Some(fifth_element) => println!("The fifth element of v2 is: {}", fifth_element),
        None => println!("There is no fifth element in v2."),
    }

    // Accessing an element out of bounds using get()
    match v2.get(10) {
        Some(element) => println!("Element at index 10: {}", element),
        None => println!("Element at index 10 does not exist."),
    }

    // **Caution:** Accessing an element out of bounds using [] will cause a panic at runtime.
    // Uncommenting the line below would cause the program to crash:
    // let non_existent_element = &v2[10];

    // 6. Modifying elements in a mutable vector using indexing
    v2[0] = 10;
    println!("Modified v2: {:?}", v2);

    // 7. Iterating over elements in a vector (immutable iteration)
    println!("Iterating over elements in v:");
    for element in &v {
        println!("{}", element);
    }

    // 8. Iterating over mutable references to elements (mutable iteration)
    println!("Iterating over mutable references in v2 and modifying elements:");
    for element in &mut v2 {
        *element += 1; // Dereference the mutable reference to modify the value
    }
    println!("v2 after mutable iteration: {:?}", v2);

    // 9. Getting the length of a vector
    println!("Length of v: {}", v.len());
    println!("Length of v2: {}", v2.len());

    // 10. Removing the last element using pop() (returns an Option<T>)
    let last_element = v2.pop();
    match last_element {
        Some(element) => println!("Popped element from v2: {}", element),
        None => println!("v2 was empty, nothing to pop."),
    }
    println!("v2 after popping: {:?}", v2);
}