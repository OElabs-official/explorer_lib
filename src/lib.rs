#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(),Box<dyn std::error::Error>>
    {
        let v = super::Explorer::new
        (
            std::path::PathBuf::from("/run/media/akitsuki/cz430/111-2/")
        )?;
        println!("{:?}",v);
        Ok(())
    }#[test]
    fn match_photos() -> Result<(),Box<dyn std::error::Error>>
    {
        let v = super::Explorer::new
        (
            std::path::PathBuf::from("/run/media/akitsuki/Windows/Users/lstsw/Videos/Captures")
        )?;
        // println!("{:?}",v);
        let mut matchs = super::MatchBy::default(); matchs.extensions=Some(super::EXT_PHOTOS);
        let v1 = v.match_files(matchs)?;
        println!("{:?}",v1);
        for i0 in v1.iter()
        {
            if let super::PathType::File(a, _b)=i0
            {
                if !a.is_file(){panic!("not exist!")}
                else
                {
                    println!("{:?}",i0.get_path())
                }
            }
        }
        Ok(())
    }

}

use 
{
    std::
    {
        error::Error,
        path::{PathBuf,Path,},
        fs,fmt,
    },
};

#[derive(Debug)]
struct Err0 (String);
impl Err0 { fn new(msg: &str) -> Err0 {Self(msg.to_string())}    }
impl fmt::Display for Err0 
{   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"{}",self.0)}    }
impl Error for Err0 
{fn description(&self) -> &str {&self.0}    }

#[derive(Debug,Clone)]
pub enum PathType
{
    Dir(PathBuf),
    KnowDir(PathBuf,Vec<PathType>,u64),    
    File(PathBuf,u64),SymLink(PathBuf),
}
impl PathType
{
    pub fn get_type<P: AsRef<Path>>(path:P)->Result<PathType,Box<dyn Error>>
    {
        let p0 = PathBuf::from(path.as_ref());
        let p1 = PathBuf::from(p0.file_name().unwrap());
        if p0.is_dir(){Ok(PathType::Dir(p1))}
        else if p0.is_file(){Ok(PathType::File(p1,p0.metadata()?.len()))}
        else if p0.is_symlink(){Ok(PathType::SymLink(p1))}
        else {Err(Box::new(Err0::new("msg: unknow pathtype")))}
    }

    pub fn read_dir(root_path:PathBuf) -> Result<Vec<PathType>,Box<dyn Error>>
    {
        let rdir0 = fs::read_dir(root_path)? ;
        let mut v0 = vec![]; for i0 in rdir0 {v0.push(PathType::get_type(i0?.path())?)};
        Ok(v0)
    }    

    pub fn get_path(&self)->PathBuf
    {
        match  self 
        {
            Self::Dir(p)=>{p.clone()},
            Self::File(p,_s )=>{p.clone()},
            Self::KnowDir(p,_v ,_s )=>{p.clone()}
            Self::SymLink(p)=>{p.clone()}
        }
    }
    fn _get_size(&self)->u64
    {
        match  self 
        {
            Self::Dir(_)=>{0},
            Self::File(_p,s )=>{*s},
            Self::KnowDir(_p,_v ,s )=>{*s}
            Self::SymLink(_p)=>{0}
        }
    }
}

#[derive(Debug)]
pub struct Explorer(Vec<PathType>,PathBuf,u64); // abs path
impl Explorer
{
    pub fn new<P: AsRef<Path>>(path:P) ->Result<Self,Box<dyn Error>>
    {   let pb0 = PathBuf::from(path.as_ref());
        let mut output = Self(PathType::read_dir(pb0.clone())?, pb0.clone(),0);
        for i0 in output.0.iter_mut()
        {
            match i0
            {
                PathType::Dir(p)=>
                {
                    let (know_dir0,ilen)= Explorer::recursion_read_dir(&pb0.join(&p))?;
                    output.2 += ilen;
                    *i0 = know_dir0;
                },
                PathType::File(_,l)=>{output.2+=*l},
                _=>{}
            }
        }
        Ok(   output   )
    }
    fn recursion_read_dir(full_path:&PathBuf) -> Result<(PathType,u64),Box<dyn Error>>
    {
        // println!("loc: {:?}",full_path);
        let mut output = PathType::read_dir(full_path.clone())?;
        let mut len0 = 0u64;
        for i0 in output.iter_mut()
        {
            match i0
            {
                PathType::Dir(p)=>
                {
                    let (know_dir0,ilen)= Explorer::recursion_read_dir(&full_path.join(&p))?;
                    len0 += ilen;
                    *i0 = know_dir0;
                },
                PathType::File(_,l)=>{len0+=*l},
                _=>{}
            }
        };
        Ok((PathType::KnowDir(full_path.clone(),output,len0),len0))
    }

}


#[derive(Clone, Copy)]
pub struct MatchBy <'a> // current version only avalible extensions
{
    pub extensions:Option<&'a[&'a str]>,
    pub larger_than:Option<u64>,pub smaller_than:Option<u64>,
    pub name_slice:Option<&'a str>,  // regrex support
    pub modify_time:Option<u64>,pub access_time:Option<u64>
}
impl <'a>  Default  for MatchBy <'a>  {fn default()->Self{Self{extensions:None,larger_than:None,smaller_than:None,name_slice:None,modify_time:None,access_time:None}}}

impl Explorer
{
    pub fn match_files(&self,matchs :MatchBy)-> Result<Vec<PathType>,Box<dyn Error>>
    {
        let mut output = vec![];
        for i0 in self.0.iter()
        {
            let mut i0_abs_path = self.1.clone();i0_abs_path.push(i0.get_path());
            output.append(&mut Explorer::recursion_match(i0_abs_path, i0.clone(), matchs)?)
        }
        Ok(output)
    }
    fn recursion_match(abs_path:PathBuf, part:PathType, matchs:MatchBy)-> Result<Vec<PathType>,Box<dyn Error>> // return abs path
    {
        let mut output = vec![];
        match part
        {
            PathType::KnowDir(_path, inner,_size)=>
            {
                for i0 in inner
                {
                    let mut i0_abs_path = abs_path.clone();i0_abs_path.push(i0.get_path());
                    output.append(&mut Explorer::recursion_match(i0_abs_path,i0,matchs)?)
                }
            },
            PathType::File(_rlt_path, i0_size)=>
            {
                match abs_path.extension()
                {
                    Some(ostr)=>
                    {
                        if let Some(x)=matchs.extensions
                        {
                            for i1 in x{if ostr.to_ascii_lowercase().to_str().unwrap().contains(i1){output.push(PathType::File(abs_path,i0_size));break;}}
                        };
                    },
                    None=>{}
                }

            },
            PathType::SymLink(_path)=>{},
            PathType::Dir(_path)=>{return Err(Box::new(Err0::new("why a not readed dir here")));}
        }
        Ok(output)
    }
}
/* to do : 
    with match type , output files in vec
*/


pub const EXT_PHOTOS : &'static[&'static str]  = &["jpg","bmp","heif","avif","tiff","png","jp2","webp","heic","gif","pnm","dds","tga","exr","ico"];
pub const EXT_VIDEOS : &'static[&'static str]  = &["avi","mkv","mp4","wmv","ts","rmvb"];
pub const EXT_AUDIOS : &'static[&'static str]  = &["mp3","aac","ape","flac"];
pub const EXT_DOCUMENTS : &'static[&'static str]  = &["xls","doc","ppt","odt","pdf"];
pub const EXT_COMPRESSED : &'static[&'static str]  = &["7z","rar","zip","tar","gz","xz"];
