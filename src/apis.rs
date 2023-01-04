use 
{
    std::
    {
        error::Error,
        path::PathBuf
    },
    crate::{core::*,base_funcs::*}
};

pub fn get_files_by_extensions(path:&str,extensions:&[&str]) ->Result<Vec<PathBuf>,Box<dyn Error>>
{
    let mut explorer = Explorer::new(path)?;
    let mut filter = Filter::default();filter.extensions = Some(extensions);
    explorer.filter(&filter)?;
    let output = explorer.flatten()?;
    Ok(output)
}
pub fn export_json()
{

}

pub fn flatten_in_json()
{}