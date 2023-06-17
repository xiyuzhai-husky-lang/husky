pub trait Has<Object>: Copy {
    fn get(self) -> Object;
}
