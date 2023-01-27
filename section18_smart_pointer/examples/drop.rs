struct Custom {
    data: String,
}

impl Drop for Custom {
    fn drop(&mut self) {
        println!("Drop with {}", self.data);
    }
}

fn main() {
    let c = Custom {
        data: String::from("aaa"),
    };
    let d = Custom {
        data: String::from("bbb"),
    };
    println!("Custom Created");
}

// variables are dropped in the reverse of the order in which they were created.
