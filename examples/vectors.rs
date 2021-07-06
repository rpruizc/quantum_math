// Sorting a Vector of Structs
// Sorts a Vector of Person structs with properties name and age by its natural order
// Derived traits are used [Eq, PartialEq, Ord, and PartialOrd]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age,
        }
    }
}

fn main() {
    // This example sorts a Vector of integers via vec::sort
    // vec::sort_unstable is faster, but does not preserve the order of equal elements
    let mut vec = vec![1, 5, 10, 2, 20];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 5, 10, 20]);

    // A Vector of Floats can be sorted using vec::sort_by and PartialOrd::partial_cmp
    let mut vec2 = vec![1.1, 3.3, 1.38, 1.123, 2.2];
    vec2.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec2, vec![1.1, 1.123, 1.38, 2.2, 3.3]);

    // Sorting a Vector of Structs
    let mut people = vec![
        Person::new("Bob".to_string(),20),
        Person::new("Joe".to_string(), 25),
        Person::new("Alice".to_string(), 3),
    ];

    // Sort the people vector by name
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Alice".to_string(),3),
            Person::new("Bob".to_string(), 20),
            Person::new("Joe".to_string(), 25),
        ]
    );

    // Sort the people vector by age
    people.sort_by(|a, b| b.age.cmp(&a.age));
    assert_eq!(
        people,
        vec![
            Person::new("Joe".to_string(), 25),
            Person::new("Bob".to_string(), 20),
            Person::new("Alice".to_string(),3),
        ]
    );


}