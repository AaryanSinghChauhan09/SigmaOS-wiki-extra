#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WikiArticle {
    pub title: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Default)]
pub struct WikiIndex {
    articles: Vec<WikiArticle>,
}

impl WikiIndex {
    pub fn new() -> Self { Self { articles: Vec::new() } }
    pub fn index_article(&mut self, title: &str, tags: &[&str]) {
        self.articles.push(WikiArticle {
            title: title.into(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
        });
    }
    pub fn search_by_tag(&self, tag: &str) -> Vec<&WikiArticle> {
        self.articles.iter().filter(|a| a.tags.iter().any(|t| t == tag)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wiki_index_search_aaa() {
        // Arrange
        let mut index = WikiIndex::new();
        index.index_article("Security Model", &["security", "kernel"]);
        index.index_article("Desktop Usability", &["desktop", "ui"]);
        // Act
        let results = index.search_by_tag("security");
        // Assert
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Security Model");
    }
}
