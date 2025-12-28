pub trait Reducer<EVT, STATE>: Send + Sync {
    fn reduce(&self, from: Option<&STATE>, evt: &EVT) -> Option<STATE>;
}
