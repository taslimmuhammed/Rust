pub trait Draw {
    fn draw(&self);
}

pub struct Screen{
    //dyn'amic dispatch allows us to create objects that implements the draw triat
    pub components: Vec<Box<dyn Draw>>,
}

//using generics will reduse flexiblity
//here we can define a custom objects of diffrent types
impl Screen
{
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
        // code to actually draw a button
        println!("drawing by button")
    }
}