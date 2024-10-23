pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

pub struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("button label: {}\nbutton area: {}", 
            self.label, self.width * self.height);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("select box area: {}", self.height * self.width);
        println!("select box options:");
        for opt in self.options.iter() {
            println!("{opt}");
        }
    }
}


fn main() {
    let b = Button {
        width: 10,
        height: 10,
        label: String::from("close"),
    };

    let sb = SelectBox {
        width: 20,
        height: 50,
        options: vec![
            String::from("yes"),
            String::from("no"),
            String::from("maybe"),
        ],
    };

    let screen = Screen {
        components: vec![
            Box::new(b),
            Box::new(sb),
        ],
    };

    screen.run();
}
