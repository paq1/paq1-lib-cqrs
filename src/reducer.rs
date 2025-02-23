pub struct Reducer<EVENT, STATE> {
    pub apply: fn(Option<STATE>, EVENT) -> Option<STATE>,
}
