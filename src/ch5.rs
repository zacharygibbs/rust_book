
// example struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}


//Tuple Structs - no named fields..
struct Color(i32, i32, i32);
struct Point {x: i32, y: i32}

// unit-like structs - don't have any fields!
struct AlwaysEqual;

fn print_point(p: &Point) {
    println!("{}, {}", p.x, p.y);
}


pub fn main() {

    // Defining and Instantiating Structs  https://rust-book.cs.brown.edu/ch05-01-defining-structs.html
    let mut user1 = build_user(
        "blahblah@gmail.com".to_string(),
        "nananana".to_string()
    );

    //println!("{:?}", user1);
    println!("e-mail: {}", user1.email);

    user1.email = "newemail_socool@protonmail.com".to_string();
    println!("Email updated");
    println!("e-mail: {}", user1.email);

    let user2 = User {
        email: "NotherNewEmail@gottagetsome.com".to_string(),
        ..user1 // copies all other fields from user1..
    };


    println!("user2 e-mail: {}", user2.email);
    //Actually cannot print user1 username
    // anymore bc it has been deallocated / moved into user2..
    // active and sign_in_count can get copied - (implement Copy trait)
    // but username (String) cannot
    //println!("e-mail: {}", user1.username);

    // Tuple structs
    // Similar to structs but args arent' named.

    let black = Color(0, 0, 0);

    println!("{}", black.0); // basically accessed the same way as a tuple
    // but probably get more for having it be a struc.


    // Unit-like structs - don't have any fields..
    // Behave similar to (); empty tuple..
    // Can implement trait on a type, but don't actually have
    // any data associated that needs stored.
    let _subject = AlwaysEqual;

    // Ownership of structs -
    // in User struct - used "String" type on purpose
    // because we want the struct to own its own data
    // and for that data to be valid as long as the struct..

    // It IS possible to store references to data owned by something else
    // BUT that requires the use of Lifetimes (ch 10) which ensure
    // that the data referenced by the struct is valid as long
    // as the struct is..

    // References can be done on each field of the struc.
    let mut p = Point { x: 0, y: 0 };
    print_point(&p);
    let x = &mut p.x;
    *x +=1;
    *x +=1;
    *x +=1;
    print_point(&p);

    let scale = 2;
    // rectangle tuple
    let rectangle1 = Rectangle {width: dbg!(30*scale), height: 25};
    dbg!(&rectangle1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("{}", rectangle1.can_hold(&rect2));
    //println!("{:#?}", rectangle1);

    let area1 = rectangle1.area();
    println!("The area of the rectangle is {} pixels^2", area1);

    let square1 = Rectangle::square(4);
    println!("{}", square1.area());

    let mut r = Rectangle {
        width: 1,
        height: 2
    };
    let area1 = r.area();
    let area2 = Rectangle::area(&r); // this is equivalent..
    assert_eq!(area1, area2);
    dbg!(area1);
    dbg!(area2);
    r.set_width(2);
    Rectangle::set_width(&mut r, 2);
    dbg!(r.area());

    // methods (with .) will de reference as much as needed
    let r = &mut Box::new(Rectangle {
        width: 1,
        height: 2
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    // two *'s , 1 to get rid of the reference&, the next to go from r -> Box on heap
    assert_eq!(area1, area2);
    println!("{} {}", area1, area2);
    &r.set_width(5);
    println!("{}", r.area());
    //mutable reference is downgraded into a shared ref
    // if you did this, you couldn't call set_width..
    let max_rect = rectangle1.max(rect2);
    dbg!(max_rect);

    // However, after using "max" the original rectangle is lost..
    //println!("{}", rectangle1.width); // this throws an error, can't use rectangle1 after it's moved
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn can_hold(&self, rectangle2: &Rectangle) -> bool {
        (
            (self.width > rectangle2.width)
            | (self.height > rectangle2.width)
        ) & (
                (self.width > rectangle2.height)
                    | (self.height > rectangle2.height)
        )
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, rectangle2: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(rectangle2.width),
            height: self.height.max(rectangle2.height),
        }
    }

    fn set_to_max(&mut self, rectangle2: Rectangle) {
        // we can do this because Rectangle does not own any heap data..
        // BUT must add the Copy, Clone derive to the Rectangle struct
        let max = self.max(rectangle2);
        *self = max;
    }


}

