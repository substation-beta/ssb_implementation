/// Condition to trigger rendering on specific image.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RenderTrigger<'a> {
    Id(&'a str),
    Time(u32)
}