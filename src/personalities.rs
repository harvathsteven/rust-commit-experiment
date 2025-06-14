use crate::git::ChangeAnalysis;
use rand::seq::SliceRandom;

pub trait Personality {
    fn generate_message(&self, analysis: &ChangeAnalysis) -> String;
}

pub struct Corporate;
pub struct Pirate;
pub struct Shakespeare;
pub struct Haiku;

impl Personality for Corporate {
    fn generate_message(&self, analysis: &ChangeAnalysis) -> String {
        let templates = [
            "Optimize codebase efficiency and enhance performance metrics",
            "Implement strategic refactoring for improved maintainability",
            "Streamline development workflow and enhance code quality",
            "Leverage best practices to optimize system architecture",
        ];

        let mut rng = rand::thread_rng();
        let template = templates.choose(&mut rng).unwrap();
        
        format!("{} (+{} -{})", template, analysis.lines_added, analysis.lines_removed)
    }
}

impl Personality for Pirate {
    fn generate_message(&self, analysis: &ChangeAnalysis) -> String {
        let templates = [
            "Arr, ye scurvy code be fixed now!",
            "Shiver me timbers! These changes be mighty fine!",
            "Yo ho ho! The code be cleaner than a ship's deck!",
            "Avast ye! The bugs be walking the plank!",
        ];

        let mut rng = rand::thread_rng();
        let template = templates.choose(&mut rng).unwrap();
        
        format!("{} (+{} -{})", template, analysis.lines_added, analysis.lines_removed)
    }
}

impl Personality for Shakespeare {
    fn generate_message(&self, analysis: &ChangeAnalysis) -> String {
        let templates = [
            "To fix, or not to fix, that is the question",
            "Alas, poor code! I knew it well",
            "The code doth protest too much, methinks",
            "All's well that ends well in this commit",
        ];

        let mut rng = rand::thread_rng();
        let template = templates.choose(&mut rng).unwrap();
        
        format!("{} (+{} -{})", template, analysis.lines_added, analysis.lines_removed)
    }
}

impl Personality for Haiku {
    fn generate_message(&self, analysis: &ChangeAnalysis) -> String {
        let templates = [
            "Code changes flow\nLike a gentle stream of thought\nBugs now float away",
            "Lines of code transform\nLike leaves in autumn's embrace\nNew features emerge",
            "Debugging complete\nLike morning dew on flowers\nFresh start begins now",
        ];

        let mut rng = rand::thread_rng();
        let template = templates.choose(&mut rng).unwrap();
        
        format!("{}\n(+{} -{})", template, analysis.lines_added, analysis.lines_removed)
    }
}

pub fn get_personality(name: &str) -> Option<Box<dyn Personality>> {
    match name.to_lowercase().as_str() {
        "corporate" => Some(Box::new(Corporate)),
        "pirate" => Some(Box::new(Pirate)),
        "shakespeare" => Some(Box::new(Shakespeare)),
        "haiku" => Some(Box::new(Haiku)),
        _ => None,
    }
} 