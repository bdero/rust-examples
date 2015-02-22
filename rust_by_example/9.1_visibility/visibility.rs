fn function() {
    println!("called function()");
}

mod my {
    pub fn function() {
        println!("called my::function()");
    }

    fn private_function() {
        println!("called my::private::function()");
    }

    pub fn indirect_access() {
        print!("called my::indirect_access(), that\n>");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called my::nested::function()");
        }

        fn private_function() {
            println!("called my::nested::private_function()");
        }
    }
}

fn main() {
    my::function();

    function();

    my::nested::function();

    my::indirect_access();
}
