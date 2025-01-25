

pub fn main() -> () {

    //https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html


    //Memory References - passing variables, then returning to prevent
    // from consuming
    let m1 = String::from("Hello"); // m1 m2 in the stack
    let m2 = String::from("world"); // Hello and world in the heap
    let (m1_again, m2_again) = greet_return(m1, m2);
    println!("{} {}", m1_again, m2_again);
    // no error since we returned m1/m2 and then resaved them;
    // trying to print m1 or m2 after this would cause error though

    // Introduce References

    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet_references(&m1, &m2); // note the ampersands -- Borrow m1 pointer on the heap (reference) and pass it
    // &m1 &m2 - m1 and m2 still own their data on the Heap; we're just borrowing them in greet refernces
    println!("{} {}", m1, m2);


    // DEreferecing
    // x is on the stack (current frame) - *x is pointer to the heap (actually in memory object)

    // The way that i see this working is that it allows you to point directly to the heap memory
    // versus pointing to the stack memory object itself (a is not pointed to x; it's pointed to
    // the box item that x is pointed to..)

    // a lives on the stack..

    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value,
    //     so x points to the value 2
    // *a += 1; - this doesn't work since a is an i32 (doesn't live on the heap)


    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value

    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;    // so only one dereference is needed to read it

    println!("x={x}; a={a}; r1={r1}, b={b}, r2={r2}, c={c}");

    //IMPLICIT Dereferences
    // So you don't have to do it yourself - the "." methods e.g. ".abs", ".len"
    // will do the dereferncing for you!

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs();      // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs();       // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len();      // implicit reference
    assert_eq!(s_len1, s_len2);

    // Vectors -
    let mut v: Vec<i32> = vec![1, 2, 3];// 1,2,3 -> pointers to the heap
    v.push(4);  // Note - when pushing new var - invalidates 1,2,3 heap above and creates new ones of diff size.
    println!("{:?}", v);

    // Example on how to deal w/ ownership errors (https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html)
    let nm: Vec<String> = vec!["Ferris".to_string(), "Jr.".to_string()];
    println!("{:?}", nm);
    let nm_full = stringify_name_with_title(&nm);
    println!("{}", nm_full);
    println!("{:?}", nm);

    // example 2
    let mut dst_strs = vec!["string2123".to_string()];//, "string3_lkj;klafsdkl;jadsfkl;98383".to_string()];
    let tst_str1 = vec![String::from("this is a long string"), String::from("tiny"), String::from("another long one")];
    //let tst_str2 = String::from("or not..");

    println!("{:?}", dst_strs);
    add_big_strings(&mut dst_strs, &tst_str1);
    println!("{:?}", dst_strs);

    // ANother exercise to fix..

    // This case works w/ Ints, but not Strings..
    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];
    let n: i32 = *n_ref;

    println!("v={:?}, n_ref={}, n={}", v, n_ref, n);

    // String case..
    let v: Vec<String> =
        vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    let s: &String = &s_ref; // *s_ref - won't work because we "cannot move, behind shared reference
    // and String doesn't implement the Copy trait"
    // s_ref tries to take ownership through v when using *s_ref, but since references (&v[0])
    // is a non owning pointer, cannot take ownership
    // through it.
    println!("v={:?}, s_ref={}, s={}", v, s_ref, s);

    // Rust can still get confused..
    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean")
    );
    name.1.push_str(", Esq."); // error because &name as a whole was borrowed and later used..
    let first = get_first(&name); // push first instead of after extracting last and we're good to go.
    println!("{first} {}", name.1);

    // SLICES - https://rust-book.cs.brown.edu/ch04-04-slices.html

    //To motivate why slices are useful, let’s work through a small programming problem:
    // write a function that takes a string of words separated by spaces and returns
    // the first word it finds in that string. If the function doesn’t find a space in the string,
    // the whole string must be one word, so the entire string should be returned. Without slices,
    // we might write the signature of the function like this:

    let test_str = "The quick red fox jumped over the lazy brown dog.".to_string();

    let mut out_vec = vec![];
    let mut out_str = "".to_string();
    for (_ix, chr) in test_str.chars().enumerate() {
        //if !chr.is_whitespace() {
        if (chr != ' ') & ( _ix != test_str.len() - 1 )  { // single quotes is a char literal..
            out_str.push_str(&chr.to_string());
        } else {
            if _ix == test_str.len() - 1 {
                out_str.push_str(&chr.to_string());
                //make sure we don't miss the last character..
            }
            out_vec.push(out_str.clone());
            out_str = "".to_string();
        }

    }

    println!("{:?}", out_vec);


    //Rust Slices & Ranges https://rust-book.cs.brown.edu/ch04-04-slices.html

    //let s: String = "Zimbabwe Ole!".to_string();
    let s = "Zimbabwe Ole!";

    println!("{}", &s[..8]); // don't need to explicity say 0; or the last val..
    println!("{}", &s[9..]);
    let frst_word = first_word(&s);

    println!("{}, {}", frst_word, frst_word.len());

    // good enough!

    // Array Slices

    let ar = [5.4, 3.8, 12.3, 11.2, 15.0, 12.0, 1.0];

    println!("{:?}", &ar[2..5]);

    // Wrapping up - Ownership https://rust-book.cs.brown.edu/ch04-05-ownership-recap.html

    type Document = Vec<String>;

    fn new_document(words: Vec<String>) -> Document {
        words
    }

    fn add_word(this: &mut Document, word: String) {
        this.push(word);
    }

    fn get_words(this: &Document) -> &[String] {
        this.as_slice()
    }

    let mut doc = new_document(vec!["hello".to_string()]);
    //add_word(&mut doc, "world".to_string());

    println!("{:?}", get_words(&doc));

    // .to_vec() converts &[String] to Vec<String> by cloning each string
    let words_copy = get_words(&doc).to_vec();
    let mut doc2 = new_document(words_copy);
    add_word(&mut doc2, "world".to_string());
    println!("{:?}", get_words(&doc2));

    // The modification to `d2` does not affect `d`
    assert!(!get_words(&doc).contains(&"world".into()));

}

fn greet_return(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}
fn greet_references(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    // prints the stringified w/out modifying the input ---
    //let mut nm1 = name.clone(); // not the best since nm1 copies all elements..
    let mut nm1 = name.join(" "); // instead join the string, then push to it..
    nm1.push_str("Esq.");
    nm1
}

// ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len = find_longest_length(dst);
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

fn find_longest_length(dst: &Vec<String>) -> usize {
    dst.iter().max_by_key(|s| s.len()).unwrap().len()
}


fn get_first(name: &(String, String)) -> &String {
    &name.0
}

fn first_word(s: &str) -> &str {
    for (ix, chr) in s.chars().enumerate(){
        if chr == ' ' {
            return &s[..ix];
        }
    }
    &s[..]
}