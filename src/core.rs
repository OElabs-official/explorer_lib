/// explorer_lib 核心库
/// 包含    
///     MetaData    存放元数据
///     PathType    分辨路径的类型，是否被扫描过，是递归的
///     Explorer    每一个会话实例，外层是扫描的根目录，和总大小
///     Err0        错误处理类型
/// 
use 
{
    std::
    {
        time::SystemTime,
        error::Error,
        path::{PathBuf,Path,},
        fs,fmt,
    },
    serde::{Serialize,Deserialize}
};

#[derive(Debug,Clone,Copy,Serialize,Deserialize)]
pub struct MetaData
{
    pub len : u64,
    pub permissions : bool, // Struct std::fs::Permissions , read only ?
    pub modified : SystemTime,// Struct std::time::SystemTime
    pub accessed : SystemTime,
    pub created : SystemTime,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum PathType
{
    Dir(PathBuf), //未扫描的文件夹
    KnowDir(PathBuf,Vec<PathType>,u64),    //已扫描完成的文件夹
    File(PathBuf,MetaData),SymLink(PathBuf),
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Explorer(pub PathBuf,pub Vec<PathType>,pub u64); // abs path

#[derive(Debug)]
pub struct Err0 (String);
impl Err0 {pub fn new(msg: &str) -> Err0 {Self(msg.to_string())}    }  
impl fmt::Display for Err0 
{   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"{}",self.0)}    }
impl Error for Err0 
{fn description(&self) -> &str {&self.0}    }


impl MetaData
{
    pub fn get_size(&self)->u64{self.len}
    pub fn get_permissions(&self)->bool{self.permissions}
    pub fn get_times(&self)->(SystemTime,SystemTime,SystemTime){(self.modified,self.accessed,self.created)}
}


impl PathType
{
    pub fn get_type<P: AsRef<Path>>(path:P)->Result<PathType,Box<dyn Error>>
    {
        let p0 = PathBuf::from(path.as_ref());
        let p1 = PathBuf::from(p0.file_name().unwrap());
        if p0.is_dir(){Ok(PathType::Dir(p1))}
        else if p0.is_file()
        {
            let buf = p0.metadata()?;
            Ok(PathType::File
            (
                p1,//p0.metadata()?.len()
                MetaData {len: buf.len(), permissions: buf.permissions().readonly(), modified: buf.modified().unwrap_or(SystemTime::now()), accessed: buf.accessed().unwrap_or(SystemTime::now()), created: buf.created().unwrap_or(SystemTime::now()) } //如果错误则定义为当前时间
            ))
        }
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
            Self::File(_p,s )=>{s.len},
            Self::KnowDir(_p,_v ,s )=>{*s}
            Self::SymLink(_p)=>{0}
        }
    }
}


impl Explorer // pub
{
    pub fn new<P: AsRef<Path>>(path:P) ->Result<Self,Box<dyn Error>>
    {   
        fn recursion(full_path:&PathBuf) -> Result<PathType,Box<dyn Error>>
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
                        let know_dir0= recursion(&full_path.join(&p))?;
                        match know_dir0
                        {
                            PathType::File(_,metadata)=>{len0+=metadata.get_size()},
                            PathType::KnowDir(_,_,size)=>{len0+=size},
                            _=>{}
                        }
                        // len0 += ilen;
                        *i0 = know_dir0;
                    },
                    PathType::File(_,l)=>{len0+=l.get_size()},
                    _=>{}
                }
            };
            Ok(PathType::KnowDir(full_path.clone(),output,len0))
        }   

        let pb0 = PathBuf::from(path.as_ref());
        let mut output = Self(pb0.clone(), PathType::read_dir(pb0.clone())?,0);// init, 0size
        for i0 in output.1.iter_mut()
        {
            match i0
            {
                PathType::Dir(p)=>
                {
                    let know_dir0= recursion(&pb0.join(&p))?;
                    match know_dir0
                    {
                        PathType::File(_,metadata)=>{output.2+=metadata.get_size()},
                        PathType::KnowDir(_,_,size)=>{output.2+=size},
                        _=>{}
                    }
                    // output.2 += ilen;
                    *i0 = know_dir0;
                },
                PathType::File(_,l)=>{output.2+=l.get_size()},
                _=>{}
            }
        }
        Ok(   output   )
    }
}


#[test]
fn f0()->Result<(),Box<dyn Error>>
{
    let x = Explorer::new("/home/oelabs/Downloads/videotest/");
    println!("{:?}",x);
    Ok(())
}