use crate::utils::pattern::*;


pub struct EscapedText {
    text: String,
    tag_starts_ends: Vec<(usize,char)>
}
impl EscapedText {
    pub fn new(source: &str) -> Self {
        let text = source.replace("\\\\", "\x1B").replace(&("\\".to_owned() + TAG_START), "\x02").replace(&("\\".to_owned() + TAG_END), "\x03").replace("\\n", "\n").replace('\x1B', "\\");
        Self {
            text: text.replace('\x02', TAG_START).replace('\x03', TAG_END),
            tag_starts_ends: text.char_indices().filter(|(_,c)| *c == TAG_START_CHAR || *c == TAG_END_CHAR).collect()
        }
    }
    pub fn iter(&self) -> TagGeometryIterator {
        TagGeometryIterator {
            source: self,
            pos: 0
        }
    }
}
pub struct TagGeometryIterator<'src> {
    source: &'src EscapedText,
    pos: usize
}
impl<'src> Iterator for TagGeometryIterator<'src> {
    type Item = (bool, &'src str);
    fn next(&mut self) -> Option<Self::Item> {
        // End of source reached?
        if self.pos == self.source.text.len() {
            return None;
        }
        // Find next tag start
        let tag_start = self.source.tag_starts_ends.iter().find(|(pos,tag)| *pos >= self.pos && *tag == TAG_START_CHAR).map(|(pos,_)| *pos);
        // Match tag or geometry
        let is_tag;
        let text_chunk;
        if tag_start.filter(|pos| *pos == self.pos).is_some() {
            is_tag = true;
            // Till tag end (considers nested tags)
            let mut tag_open_count = 0usize;
            if let Some(end_pos) = self.source.tag_starts_ends.iter().find(|(pos,tag)| match *tag {
                _ if *pos < self.pos + TAG_START.len() => false,
                TAG_START_CHAR => {tag_open_count+=1; false}
                TAG_END_CHAR => if tag_open_count == 0 {true} else {tag_open_count-=1; false}
                _ => false
            }).map(|(pos,_)| *pos) {
                text_chunk = &self.source.text[self.pos + TAG_START.len()..end_pos];
                self.pos = end_pos + TAG_END.len();
            // Till end
            } else {
                text_chunk = &self.source.text[self.pos + TAG_START.len()..];
                self.pos = self.source.text.len();
            }
        } else {
            is_tag = false;
            // Till tag start
            if let Some(tag_start) = tag_start {
                text_chunk = &self.source.text[self.pos..tag_start];
                self.pos = tag_start;
            // Till end
            } else {
                text_chunk = &self.source.text[self.pos..];
                self.pos = self.source.text.len();
            }
        }
        // Return tag or geometry
        Some((is_tag, text_chunk))
    }
}

pub struct TagsIterator<'src> {
    text: &'src str,
    pos: usize
}
impl<'src> TagsIterator<'src> {
    pub fn new(text: &'src str) -> Self {
        Self {
            text,
            pos: 0
        }
    }
}
impl<'src> Iterator for TagsIterator<'src> {
    type Item = (&'src str, Option<&'src str>);
    fn next(&mut self) -> Option<Self::Item> {
        // End of source reached?
        if self.pos == self.text.len() {
            return None;
        }
        // Find next tag separator (considers nested tags)
        let mut tag_open_count = 0usize;
        let tag_sep = self.text.char_indices().skip(self.pos).find(|(_,c)| match *c {
            TAG_START_CHAR => {tag_open_count+=1; false}
            TAG_END_CHAR => {if tag_open_count > 0 {tag_open_count-=1} false}
            TAG_SEPARATOR if tag_open_count == 0 => true,
            _ => false
        }).map(|(index,_)| index);
        // Match till separator or end
        let tag_token;
        if let Some(tag_sep) = tag_sep {
            tag_token = &self.text[self.pos..tag_sep];
            self.pos = tag_sep + 1 /* TAG_SEPARATOR */;
        } else {
            tag_token = &self.text[self.pos..];
            self.pos = self.text.len();
        }
        // Split into name+value and return
        if let Some(tag_assign) = tag_token.find(TAG_ASSIGN) {
            Some((&tag_token[..tag_assign], Some(&tag_token[tag_assign + 1 /* TAG_ASSIGN */..])))
        } else {
            Some((tag_token, None))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{EscapedText,TagsIterator};

    #[test]
    fn tag_geometry_iter() {
        let text = EscapedText::new("[tag1][tag2=[inner_tag]]geometry1\\[geometry1_continue\\\\[tag3]geometry2\\n[tag4");
        let mut iter = text.iter();
        assert_eq!(iter.next(), Some((true, "tag1")));
        assert_eq!(iter.next(), Some((true, "tag2=[inner_tag]")));
        assert_eq!(iter.next(), Some((false, "geometry1[geometry1_continue\\")));
        assert_eq!(iter.next(), Some((true, "tag3")));
        assert_eq!(iter.next(), Some((false, "geometry2\n")));
        assert_eq!(iter.next(), Some((true, "tag4")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn tags_iter() {
        let mut iter = TagsIterator::new("mode=points;reset;animate=0,-500,[position=200,100.5];color=ff00ff;mask-clear");
        assert_eq!(iter.next(), Some(("mode", Some("points"))));
        assert_eq!(iter.next(), Some(("reset", None)));
        assert_eq!(iter.next(), Some(("animate", Some("0,-500,[position=200,100.5]"))));
        assert_eq!(iter.next(), Some(("color", Some("ff00ff"))));
        assert_eq!(iter.next(), Some(("mask-clear", None)));
        assert_eq!(iter.next(), None);
    }
}