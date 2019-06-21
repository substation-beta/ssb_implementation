#[derive(Debug, PartialEq, Clone)]
pub enum RenderTrigger<'a> {
    Id(&'a str),
    Time(u32)
}