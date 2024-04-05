pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>
}

impl<T> Screen<T> where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button with label: {}", self.label);
        // code to actually draw a button
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a select box with options: {:?}", self.options);
        // code to actually draw a select box
    }
}


impl Draw for Box<dyn Draw> {
    fn draw(&self) {
        self.as_ref().draw();
    }
}

fn main() {
    // Создаем экземпляр объекта Button и SelectBox
    let button = Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
    };
    let select_box = SelectBox {
        width: 75,
        height: 10,
        options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
        ],
    };

    // Создаем вектор компонентов для Screen
    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(button) as Box<dyn Draw>,
        Box::new(select_box) as Box<dyn Draw>,
    ];

    let screen = Screen { components };

    // Вызываем метод run() для отображения всех компонентов на экране
    screen.run();
}
