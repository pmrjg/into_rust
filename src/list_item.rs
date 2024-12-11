#[derive(Clone)]
pub struct ListItem<T>
where T : Clone,
{
    pub data: Box<T>,
    pub next: Option<Box<ListItem<T>>>,
}

enum Recursive<T>{
    Next(Box<Recursive<T>>),
    Boxed(Box<T>),
    Optional(Option<T>),
}