/// 提供的功能
///     Filter 使用 filter方法筛选符合条件的文件
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
    crate::core::{Explorer,PathType,Err0}
};


#[derive(Clone,Copy)]
pub struct Filter<'a>
{
    pub extensions: Option<&'a[&'a str]>,
    pub size_range: [Option<u64>;2], //min,max
    pub name_slice: Option<&'a str>,
    pub time:[Option<SystemTime>;6],//mod, acc, create
    pub read_only: bool,
    pub symlink:bool
}

impl <'a> Default for Filter<'a>{fn default()->Self{Self{extensions:None,size_range:[None;2],name_slice:None,time:[None;6],read_only:false,symlink:false}}}

impl Explorer
{
    /*
    flatten 平铺，以一维向量方式输出所有文件名（绝对路径）
    filter 过滤，只保留符合条件的文件
    classfication 
    */
    pub fn flatten(&self)->Result<Vec<PathBuf>,Box<dyn Error>>
    {// 展开为绝对路径

        fn recursion(abs_path:PathBuf, part:PathType)-> Result<Vec<PathBuf>,Box<dyn Error>> // return abs path
        {
            let mut output = vec![];
            match part
            {
                PathType::KnowDir(_path, inner,_size)=>
                {
                    for i0 in inner
                    {
                        let mut i0_abs_path = abs_path.clone();i0_abs_path.push(i0.get_path());
                        output.append(&mut recursion(i0_abs_path,i0)?)
                    }
                },
                PathType::File(_rlt_path, _i0_metadata)=>
                {
                    output.push(abs_path)
                },
                PathType::SymLink(_path)=>{},
                PathType::Dir(_path)=>{return Err(Box::new(Err0::new("why a not readed dir here")));}
            }
            Ok(output)
        }

        let mut output = vec![];
        for i0 in self.1.iter()
        {
            let mut i0_abs_path = self.0.clone();i0_abs_path.push(i0.get_path());
            output.append(&mut recursion(i0_abs_path, i0.clone())?)
        }
        Ok(output)
    }

    pub fn filter(&mut self,filter:&Filter)->Result<(),Box<dyn Error>>
    {//待完成，文件过滤器
        fn recursion(i0:&mut PathType,filter:&Filter)->Result<Option<(u64,bool)>,Box<dyn Error>> //true : 文件 false 文件夹，不会移除文件夹
        {
            match i0
            {
                PathType::Dir(_path)=>{return Err(Box::new(Err0::new("why a not readed dir here")));},
                PathType::KnowDir(_,inner ,size )=>
                {
                    let mut remove_size = 0;
                    for i0 in (0..inner.len()).rev()
                    {
                        if let Some((inner_size,remove)) = recursion(&mut inner[i0],filter)?
                        {
                            // println!("remove{}",inner_size);
                            remove_size += inner_size;
                            if remove{let _ = inner.remove(i0);}
                            // else {*size = inner_size}
                        }
                    }
                    *size -= remove_size;
                    return Ok(Some((remove_size,false)))
                },
                PathType::File(path,metadata )=>
                {   
                    let size = metadata.get_size();
                    {}//ext
                    {
                        // println!("{:?},{}",path,size); //ok
                        let [min,max] = filter.size_range;
                        if let Some(min) = min{if size < min{return Ok(Some((size,true)))}}
                        if let Some(max) = max{if size > max{return Ok(Some((size,true)))}}
                    }//size
                    {}//name
                    {}//time
                    {}//readonly
                },
                PathType::SymLink(_path)=>
                {
                    {}//ext
                    {}//name
                    {}//time
                    {}//readonly                    
                }
            }
            Ok(None)
        }

        let mut remove_size = 0;
        // for i0 in self.0.iter_mut()
        for i0 in (0..self.1.len()).rev()
        {
            
            //let mut i0_abs_path = self.1.clone();i0_abs_path.push(i0.get_path());
            if let Some((size,remove)) = recursion(&mut self.1[i0],filter)?
            {
                // println!("remove{}",size);
                // self.2 -= size;
                // let _ = self.0.remove(i0);
                remove_size += size;
                if remove{let _ = self.1.remove(i0);}
                // else {self.2 = size}
                // println!("removed:{:?}",x);
            }
        }
        self.2 -= remove_size;
        Ok(())
    }

    pub fn same_name_finder(&self)->Result<(),Box<dyn Error>>
    {//使用hashset比对同名文件
        Ok(())
    }

}



//----------------------------------------------------------------
#[test]
fn f0()->Result<(),Box<dyn Error>>
{
    let x = Explorer::new("/home/oelabs/Downloads/darktable_exported/")?;
    let x2 = x.flatten()?;
    println!("{:?}",x2);
    Ok(())
}
#[test]fn f1()->Result<(),Box<dyn Error>>
{
    for i0 in (0..10).rev(){println!("{}",i0)};
    Ok(())
}
#[test]
fn f2()->Result<(),Box<dyn Error>>
{
    let mut x = Explorer::new("/home/oelabs/Downloads/darktable_exported/")?;
    println!("{:?}",x);
    let mut filter = Filter::default();filter.size_range=[(Some(50000000)),None];
    x.filter(&filter)?;
    let x2 = x.flatten()?;
    println!("{:?}",x);
    Ok(())
}
//看看是不是explorer统计大小算错

#[test]
fn to_json()->Result<(),Box<dyn Error>>
{
    let mut x = Explorer::new("/home/oelabs/Downloads/darktable_exported/")?;
    let jsn = serde_json::to_string(&x)?;
    println!("{}",jsn);
    Ok(())
}