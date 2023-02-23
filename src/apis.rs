use 
{
    std::
    {
        error::Error,
        path::PathBuf
    },
    crate::{core::*,base_funcs::*},
    csv,
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

pub fn fx() ->Result<(),Box<dyn std::error::Error>>
{
    Ok(())

}

pub fn export_table_to_csv(path:&str) ->Result<(),Box<dyn Error>> 
{
    let explorer = Explorer::new(path)?;
    let table0 = explorer.export_table()?;
    let mut csv0 = csv::Writer::from_writer(vec![]);
    for i0 in table0.0
    {
        csv0.serialize(i0)?;
    }
    // csv0.serialize(table0);
    std::fs::write("output.csv", String::from_utf8(csv0.into_inner()?)?)?;
    Ok(())
}

pub fn export_table_to_json(path:&str) ->Result<(),Box<dyn Error>>
{
    let explorer = Explorer::new(path)?;
    let table0 = explorer.export_table()?;
    let jsn0 = serde_json::to_string(&table0)?;
    
    println!("{}",jsn0);
    std::fs::write("output.json", jsn0)?;
    Ok(())
}
pub fn export_table_to_toml(path:&str) ->Result<(),Box<dyn Error>>
{
    let explorer = Explorer::new(path)?;
    let table0 = explorer.export_table()?;
    let toml0 = toml::to_string(&table0)?;
    
    println!("{}",toml0);
    std::fs::write("output.toml", toml0)?;
    Ok(())
}

#[test]
fn 导出测试()
{
    export_table_to_json("/home/oelabs/Documents/OP GAME/OP GAME/H PHOTO/").unwrap();
}