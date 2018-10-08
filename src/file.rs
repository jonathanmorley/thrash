use client::Client;
use failure::Error;
use project::Project;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use repository::Repository;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct FileLocation {
    project: Project,
    repository: Repository,
    path: PathBuf,
}

impl Client {
    pub fn file_locations(
        &self,
        project_filter: impl Fn(&Project) -> bool + Sync + Send,
        repository_filter: impl Fn(&Repository) -> bool + Sync + Send,
        file_filter: impl Fn(&PathBuf) -> bool + Sync + Send,
    ) -> Result<Vec<FileLocation>, Error> {
        let projects = self.projects()?.into_par_iter().filter(project_filter);

        let repositories =
            projects.flat_map(move |project| match self.repositories(&project.key()) {
                Ok(repositories) => repositories
                    .into_par_iter()
                    .filter(|r| repository_filter(&r))
                    .map(|r| Ok((project.clone(), r)))
                    .collect(),
                Err(e) => vec![Err(e)],
            });

        repositories
            .flat_map(|repository| match repository {
                Ok(repository) => {
                    match self.repository_files(&repository.0.key(), &repository.1.slug()) {
                        Ok(locations) => locations
                            .into_par_iter()
                            .filter(|f| file_filter(&f))
                            .map(|path| {
                                Ok(FileLocation {
                                    project: repository.0.clone(),
                                    repository: repository.1.clone(),
                                    path,
                                })
                            })
                            .collect(),
                        Err(e) => vec![Err(e)],
                    }
                }
                Err(e) => vec![Err(e)],
            })
            .collect()
    }
}
