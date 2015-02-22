fn function() {
    println!("called function()");
}

mod my {
    pub fn indirect_call() {
        print!("called my::indirect_call(), that\n> ");

        function();

        {
            use cool::function as root_cool_function;

            print!("> ");
            root_cool_function();
        }

        {
            use self::cool::function as my_cool_function;

            print!("> ");
            my_cool_function();
        }

        {
            use super::function as root_function;

            print!("> ");
            root_function();
        }
    }

    fn function() {
        println!("called my::function()");
    }

    mod cool {
        pub fn function() {
            println!("called my::cool::function()");
        }
    }
}

mod cool {
    pub fn function() {
        println!("called cool::function()");
    }
}

fn main() {
    my::indirect_call();
}
