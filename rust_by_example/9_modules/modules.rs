fn function() {
    println!("called function()");
}

mod my {
    fn function() {
        println!("called my::function()");
    }

    mod nested {
        fn function() {
            println!("called my::nested:function()");
        }
    }
}

fn main() {
    std::old_io::stdio::println("Hello world!");
    function();

    // my::function is private; errors
    //my::function();
}
