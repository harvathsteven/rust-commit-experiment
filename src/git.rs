use git2::{Repository, Diff, DiffOptions, DiffLine};
use anyhow::Result;

pub struct GitAnalyzer {
    repo: Repository,
}

impl GitAnalyzer {
    pub fn new() -> Result<Self> {
        let repo = Repository::open_from_env()?;
        Ok(Self { repo })
    }

    pub fn get_diff(&self) -> Result<Diff> {
        let mut opts = DiffOptions::new();
        opts.include_untracked(true);
        opts.include_ignored(false);
        opts.include_unmodified(false);
        
        let diff = self.repo.diff_index_to_workdir(None, Some(&mut opts))?;
        Ok(diff)
    }

    pub fn analyze_changes(&self) -> Result<ChangeAnalysis> {
        let diff = self.get_diff()?;
        let mut analysis = ChangeAnalysis::default();

        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    let extension = path.extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or("unknown");
                    
                    analysis.file_types.insert(extension.to_string());
                }
                true
            },
            None,
            None, 
            Some(&mut |_delta, _hunk, line: DiffLine<'_>| {
                match line.origin() {
                    '+' => analysis.lines_added += 1,
                    '-' => analysis.lines_removed += 1,
                    _ => {}
                }
                true
            }),
        )?;

        Ok(analysis)
    }
}

#[derive(Default)]
pub struct ChangeAnalysis {
    pub file_types: std::collections::HashSet<String>,
    pub lines_added: i32,
    pub lines_removed: i32,
} 