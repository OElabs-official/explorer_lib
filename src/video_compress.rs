use
{
    serde::{Serialize,Deserialize},
    std::
    {
        error::Error,
        path::PathBuf,
        io,
        process::Command,
    },
    // mapcomp::vecc
};
const CONFIG_PATH : &'static str = "video_compress_cfg.toml";


pub fn video_compress_main() ->Result<(),Box<dyn Error>>{
    // println!("Hello, world!");
    // let _ = Config::make_default();
    let cfg = Config::load_config()?;
    let [pb0,pb1] = get_path()?;
    // let videos = scan_videos(pb0,cfg.extensions.iter().map(|x|x.as_str()).collect(),cfg.min_size_mb)?;
    let videos = cfg.scan_videos(pb0)?;
    println!("found <{}> videos",videos.len());
    cfg.encode(videos, pb1)?;
    Ok(())
}

impl Config
{
    fn scan_videos(&self,pb0:PathBuf)->Result<Vec<PathBuf>,Box<dyn Error>>
    {
        let mut explorer = crate::core::Explorer::new(pb0)?;
        let mut filter = crate::base_funcs::Filter::default();
        let tmp_ext = self.extensions.iter().map(|x|x.as_str()).collect::<Vec<_>>();
        filter.extensions = Some(&tmp_ext);
        filter.size_range = [Some(&self.min_size_mb *1024 *1024),None];
        explorer.filter(&filter)?;
        let output = explorer.flatten()?;
        Ok(output)
    }

    fn encode(&self,videos:Vec<PathBuf>,output:PathBuf)->Result<(),Box<dyn Error>>
{
    match self.method
    {
        Method::HevcNvenc=>
        {
            for i0 in 0..videos.len()
            {
                let mut new_file_name = videos[i0].clone();
                new_file_name.set_file_name(format!("{}-nvenc.mp4",new_file_name.file_name().unwrap().to_str().unwrap()));
                println!("create new video : {} ", new_file_name.display());
                let mut i0_output_path = output.clone();i0_output_path.push(new_file_name.file_name().unwrap());
                let c = Command::new(&self.ffmpeg_path)
                .args
                ([
                    "-i",videos[i0].as_os_str().to_str().unwrap(),
                    "-c:v","hevc_nvenc","-c:a","aac","-b:a","192k","-qp","25",
                    i0_output_path.as_os_str().to_str().unwrap(),
                ]).status()?;
                if c.success(){println!("encode {}/{} complete!",i0+1,videos.len())}
                else {println!("Error : encode {} fail !",videos[i0].display())}
            }
            
        },
        _=>{}
    }
    Ok(())
}
    
}
fn get_path()->Result<[PathBuf;2],Box<dyn Error>>
{
    println!("输入视频路径: \n");
    let (mut s0,mut s1) = (String::new(),String::new());
    io::stdin().read_line(&mut s0)?;s0 = s0.trim().to_string();
    let pb0 = PathBuf::from(s0);
    println!("输出视频路径: \n");
    io::stdin().read_line(&mut s1)?;s1 = s1.trim().to_string();
    let pb1 = PathBuf::from(s1);
    Ok([pb0,pb1])
}

// fn encode(videos:Vec<PathBuf>,output:PathBuf,method:Method)->Result<(),Box<dyn Error>>
// {
//     match method
//     {
//         Method::HevcNvenc=>
//         {
//             for i0 in 0..videos.len()
//             {
//                 let mut new_file_name = videos[i0].clone();
//                 new_file_name.set_file_name(format!("{}-nvenc.mp4",new_file_name.file_name().unwrap().to_str().unwrap()));
//                 println!("create new video : {} ", new_file_name.display());
//                 let mut i0_output_path = output.clone();i0_output_path.push(new_file_name);
//                 // Command::new(program)
//             }
            
//         },
//         _=>{}
//     }
//     Ok(())
// }
// fn scan_videos(pb0:PathBuf,ext:Vec<&str>,min_size:u64)->Result<Vec<PathBuf>,Box<dyn Error>>
// {
//     let mut explorer = explorer_lib::core::Explorer::new(pb0)?;
//     let mut filter = explorer_lib::base_funcs::Filter::default();
//     filter.extensions = Some(&ext);
//     filter.size_range = [Some(min_size *1024 *1024),None];
//     explorer.filter(&filter)?;
//     let output = explorer.flatten()?;
//     Ok(output)
// }
// #[test]
// fn t0()
// {
//     let pb0 = PathBuf::from("/home/oelabs/Documents");
//     let cfg = Config::default();
//     let (ext,min_size) = (cfg.extensions,cfg.min_size_mb);
//     let x = scan_videos(pb0, ext.iter().map(|x|x.as_str()).collect(), min_size).unwrap();
//     println!("{:?}",x);
// }

#[derive(Debug,Clone,Serialize,Deserialize)]
enum Method
{
    HevcNvenc,
    HevcQSV,
    AV1QSV,
    AV1STV,
}
#[derive(Debug,Clone,Serialize,Deserialize)]
struct Config
{
    pub ffmpeg_path : String,
    pub method : Method,
    pub min_size_mb : u64,
    pub extensions : Vec<String>
}
impl Default for Config 
{fn default() -> Self {Self { ffmpeg_path: "ffmpeg".to_string() , method: Method::HevcNvenc ,min_size_mb: 128 ,extensions:["avi","mkv","mp4","wmv","ts","rmvb"].iter().map(|x|x.to_string()).collect()}}}
impl Config
{
    fn make_default()->Result<(),Box<dyn Error>>
    {
        let dft = Self::default();
        let file = toml::to_string(&dft)?;
            // println!("{}",file);
        std::fs::write(CONFIG_PATH, file)?;
        Ok(())
    }   
    fn load_config()->Result<Config,Box<dyn Error>>
    {
        let cfg : Config;
        match std::fs::read_to_string(CONFIG_PATH)
        {
            Ok(file)=>{cfg = toml::from_str(file.as_str())?;},
            Err(e)=>{println!("load cfg err: {:?} ",e);cfg = Config::default();Config::make_default()?}
        }
            // let dft = Self::default();
            // let file = toml::to_string(&dft)?;
            // println!("{}",file);
            // let cfg = toml::from_str(&file)?;
            // println!("{:?}",cfg);
        Ok(cfg)
    }   
}

#[test]
fn command()
{
    let c = Command::new("/home/oelabs/static_apps/ffmpeg-5.1.1-arm64-static/ffmpeg").args(["-i","/home/oelabs/Documents/test.mp4","-c:v","libsvtav1","-c:a","aac","-b:a","192k","-cq","25","/home/oelabs/Documents/output.mp4"]).status().unwrap();
    println!("{:?}",c);
}
