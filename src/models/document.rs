
pub struct DocumentInfo {
    pub title: String
}

impl DocumentInfo {    
    pub fn new(text: &str) -> Self {
        let mut doc = DocumentInfo {
            title: String::new()
        };
        doc.find_title_line(text);
        doc
    }
    
    fn find_title_line(&mut self, text: &str) -> Option<(usize, String)> {
        for (i, line) in text.lines().enumerate() {
            if line.contains("@title") {
                if let Some(title_text) = line.split("@title").nth(1) {
                    self.title = title_text.trim().to_string();
                    return Some((i + 1, line.to_string()));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_title() {
        let text = "Some text\n@title Mi Documento\nMore text";
        let doc = DocumentInfo::new(text);
        assert_eq!(doc.title, "Mi Documento");
    }

    #[test]
    fn test_new_without_title() {
        let text = "Some text\nMore text";
        let doc = DocumentInfo::new(text);
        assert!(doc.title.is_empty());
    }

    #[test]
    fn test_find_title_line_with_title() {
        let mut doc = DocumentInfo::new("");
        let text = "First line\n@title Test Document\nLast line";
        let result = doc.find_title_line(text);
        
        assert!(result.is_some());
        let (line_number, line_text) = result.unwrap();
        assert_eq!(line_number, 2); // segunda línea (1-based)
        assert_eq!(doc.title, "Test Document");
        assert_eq!(line_text, "@title Test Document");
    }

    #[test]
    fn test_find_title_line_without_title() {
        let mut doc = DocumentInfo::new("");
        let text = "First line\nSecond line\nLast line";
        let result = doc.find_title_line(text);
        
        assert!(result.is_none());
        assert!(doc.title.is_empty());
    }

    #[test]
    fn test_find_title_line_with_empty_title() {
        let mut doc = DocumentInfo::new("");
        let text = "First line\n@title \nLast line";
        let result = doc.find_title_line(text);
        
        assert!(result.is_some());
        let (line_number, _) = result.unwrap();
        assert_eq!(line_number, 2);
        assert!(doc.title.is_empty());
    }
}